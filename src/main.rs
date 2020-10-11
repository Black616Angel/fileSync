#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate ftp;
extern crate curl;

pub mod sql;
pub mod myftp;
pub mod web;

use ftp::FtpStream;
use crate::sql::models::N_File;
use crate::sql::models::File;

fn main() {
	println!("get ftp stream");
	let ftp_ip = sql::get_ip();
	let url = ftp_ip.to_owned() + ":" + &"21".to_string();
	let mut ftp_stream = myftp::get_stream(url);
	println!("get filelist");
//	upload_file( N_File { path: "/Benni/Writer".to_string(), filename: "TRI.md".to_string() }, &mut ftp_stream, false, 0);

	let ftp_list = myftp::get_filelist(&mut ftp_stream);
	for n_file in ftp_list {
		let file = File { path: n_file.path.to_string(), filename: n_file.filename.to_string(), synced: false, deleted: false, id: 0};
		let res = sql::select_file(file);
		if res.is_err() {
			upload_file(n_file, &mut ftp_stream, false, 0);
		}
		else {
			let sel_file = res.unwrap();
			if sel_file.synced == false {
				upload_file(n_file, &mut ftp_stream, true, sel_file.id);
			}
		}
	}
	let _ = ftp_stream.quit();
}

fn upload_file(i_file: N_File, mut stream: &mut FtpStream, update: bool, id: i32) {
	//get data from ftp
	let file = &i_file;
	myftp::get_file(&file, &mut stream);
	web::api(file.filename.to_string(), &file.path);
	if update {
		sql::update_synced( id, true);
	} else {
		sql::insert_file( file );
	}
	delete_ftp_file(file);
}

fn delete_ftp_file(file: &N_File) {
	use std::fs::remove_file;
	let fpath = file.path.to_string() + &file.filename;
	remove_file(fpath).unwrap();
}
