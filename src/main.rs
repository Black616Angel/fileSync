#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate ftp;
extern crate curl;
extern crate crossterm;
extern crate tokio;
extern crate futures;

pub mod sql;
pub mod myftp;
pub mod web;

use crate::sql::models::NFile;
use crate::sql::models::File;

use std::convert::TryInto;
use std::env;
use dotenv::dotenv;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};

use std::io::stdout;
use crossterm::{ExecutableCommand, cursor};
use crossterm::terminal::{Clear, ClearType};

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
static MAX_THREAD_COUNT: i32 = 5;

#[tokio::main]
async fn main() {
    dotenv().ok();

	let not_synced = sql::select_files_not_synced();
	let ten_millis = time::Duration::from_millis(10);
	if not_synced.is_ok() {
		let nsynced_files = not_synced.unwrap();
		if nsynced_files.len() > 0 {
			let mut t_num = 0;
			let mut first = true;
			for nsynced_file in nsynced_files {
				while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) >= MAX_THREAD_COUNT.try_into().unwrap() {
					thread::sleep(ten_millis);
				}
				t_num = t_num + 1;
				if t_num > MAX_THREAD_COUNT {
					t_num = 0;
					first = false;
				}
				tokio::spawn(upload_file(nsynced_file.to_nfile(), false, 0, t_num, first));
			}
		}
	}

	println!("get ftp stream");
	let ignore_synced = env::var("IGNORE_SYNCED").is_ok();
	let mut ftp_stream = myftp::get_stream();
	println!("get filelist");

	let ftp_list = myftp::get_filelist(&mut ftp_stream);
	let _ = ftp_stream.quit();
	println!("upload {:?} files", ftp_list.len());
	println!("");
	let mut t_num = 0;
	let mut first = true;
	for n_file in ftp_list {
		while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) >= MAX_THREAD_COUNT.try_into().unwrap() {
			thread::sleep(ten_millis);
		}
		let file = File { path: n_file.path.to_string(), filename: n_file.filename.to_string(), synced: false, deleted: false, id: 0};
		let res = sql::select_file(file);
		if res.is_err() || ignore_synced {
			t_num = t_num + 1;
			if t_num > MAX_THREAD_COUNT {
				t_num = 0;
				first = false;
			}
			tokio::spawn(upload_file(n_file, false, 0, t_num, first));
		}
		else {
			let sel_file = res.unwrap();
			if sel_file.synced == false {
				t_num = t_num + 1;
				if t_num > MAX_THREAD_COUNT {
					t_num = 0;
					first = false;
				}
				tokio::spawn(upload_file(n_file, true, sel_file.id, t_num, first));
			}
		}
	}
}

async fn upload_file(i_file: NFile, update: bool, id: i32, tnum: i32, imove: bool) {
	//get data from ftp
	let file = &i_file;
	if !update {
		sql::insert_file( file );
	}
	GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
	stdout().execute(cursor::SavePosition).expect("");
	if !imove {
		let mut stdout = stdout();
		stdout.execute(cursor::MoveToPreviousLine((MAX_THREAD_COUNT+1-tnum).try_into().unwrap())).expect("");
		stdout.execute(Clear(ClearType::CurrentLine)).expect("");
	}
	println!("getting   file: {:?}", i_file.filename);
	stdout().execute(cursor::RestorePosition).expect("");
	let mut stream = myftp::get_stream();
	myftp::get_file(&i_file, &mut stream);
	stdout().execute(cursor::SavePosition).expect("");
	if !imove {
		let mut stdout = stdout();
		stdout.execute(cursor::MoveToPreviousLine((MAX_THREAD_COUNT+1-tnum).try_into().unwrap())).expect("");
		stdout.execute(Clear(ClearType::CurrentLine)).expect("");
	} else {
		let num: i32 = GLOBAL_THREAD_COUNT.load(Ordering::SeqCst).try_into().unwrap();
		let mut stdout = stdout();
		stdout.execute(cursor::MoveToPreviousLine((num-tnum).try_into().unwrap())).expect("");
		stdout.execute(Clear(ClearType::CurrentLine)).expect("");
	}
	println!("uploading file: {:?}", i_file.filename);
	stdout().execute(cursor::RestorePosition).expect("");
	if web::api(file.filename.to_string(), &file.path).is_err() {
		println!("error uploading file: {:?}", i_file.path + "/" + &i_file.filename);
		return;
	}
	sql::update_synced( id, true);
	delete_ftp_file(file);
	GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
}

fn delete_ftp_file(file: &NFile) {
	use std::fs::remove_file;
	let fpath = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set") + &file.filename;
	remove_file(fpath).unwrap();
}
