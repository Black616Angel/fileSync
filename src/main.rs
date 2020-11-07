#![allow(deprecated)]
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
pub mod output;

use crate::sql::models::NFile;
use crate::sql::models::File;

use std::convert::TryInto;
use std::env;
use dotenv::dotenv;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};
use lazy_static::lazy_static;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
lazy_static! {
    static ref MAX_THREAD_COUNT: i32 = {
        dotenv().ok();
        (env::var("THREADS").unwrap()).parse().unwrap()
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    cleanup();

    let ten_millis = time::Duration::from_millis(10);
	println!("get ftp stream");
	let ignore_synced = env::var("IGNORE_SYNCED").is_ok();
	let mut ftp_stream = myftp::get_stream();
	println!("get filelist");

	let ftp_list = myftp::get_filelist(&mut ftp_stream);
	let _ = ftp_stream.quit();
	println!("upload {:?} files", ftp_list.len());
	println!("");
	let mut t_num = 0;
    let mut id = 0;
    for _ in 1..*MAX_THREAD_COUNT{
        println!();
    }
	let print = false;
	for n_file in ftp_list {
        id+=1;
		while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) >= (*MAX_THREAD_COUNT).try_into().unwrap() {
			thread::sleep(ten_millis);
		}
		let file = File { path: n_file.path.to_string(), filename: n_file.filename.to_string(), synced: false, deleted: false, id};
		let res = sql::select_file(file);
		if res.is_err() || ignore_synced {
//			println!("{:?}: {:?}",GLOBAL_THREAD_COUNT.load(Ordering::SeqCst), (n_file.filename).to_string());
			t_num = t_num + 1;
			if t_num > *MAX_THREAD_COUNT {
				t_num = 0;
			}
			tokio::spawn(upload_file(n_file, false, id, t_num, print));
		}
		else {
			let sel_file = res.unwrap();
			if sel_file.synced == false {
				t_num = t_num + 1;
				if t_num > *MAX_THREAD_COUNT {
					t_num = 0;
				}
				tokio::spawn(upload_file(n_file, true, sel_file.id, t_num, print));
			}
		}
	}
	println!("Done!");
}

async fn upload_file(i_file: NFile, update: bool, id: i32, tnum: i32, print: bool) {
	let file = &i_file;
	if !update {
		sql::insert_file( file );
	}
    let lnum: u16 = (*MAX_THREAD_COUNT+1-tnum).try_into().unwrap();
	GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    let fnam: String = i_file.filename.chars().take(30).collect();
//	if print {
		output::print_in_line(&format!("getting   file      {}: {:?}", id, fnam).to_string(), &lnum, true);
//	}

	let mut stream = myftp::get_stream();
	myftp::get_file(&i_file, &mut stream, &lnum, print);
//	if print {
		output::print_in_line(&format!("uploading file      {}: {:?}", id, fnam).to_string(), &lnum, true);
//	}
	let answer = web::api(file.filename.to_string(), &file.path, &lnum, print);
	if answer.is_err() { //&& print {
		output::print_in_line(&format!("error uploading file: {:?}", i_file.path + "/" + &fnam).to_string(), &lnum, true);
        output::print_log(format!("{:?}", answer.unwrap_err()).to_string());
    	GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
		return;
	}
	sql::update_synced( id, true);
	delete_ftp_file(file);
	GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
}

fn cleanup() {
    let paths = std::fs::read_dir(env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set")).unwrap();

    for path in paths {
	       std::fs::remove_file(path.unwrap().path()).unwrap();
    }
}

fn delete_ftp_file(file: &NFile) {
	let fpath = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set") + &file.filename;
	std::fs::remove_file(fpath).unwrap();
}
