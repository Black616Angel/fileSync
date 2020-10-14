#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate ftp;
extern crate curl;

pub mod sql;
pub mod myftp;
pub mod web;

use ftp::FtpStream;
use crate::sql::models::NFile;
use crate::sql::models::File;

use std::env;

fn main() {
	println!("get ftp stream");
	let ftp_url = web::get_url();
	let url = ftp_url.to_owned() + ":" + &"21".to_string();
	let mut ftp_stream = myftp::get_stream(url);
	println!("get filelist");

	let ftp_list = myftp::get_filelist(&mut ftp_stream);
	println!("upload files");
	for n_file in ftp_list {
		let file = File { path: n_file.path.to_string(), filename: n_file.filename.to_string(), synced: false, deleted: false, id: 0};
		let res = sql::select_file(file);
		println!("{:?}", res.is_err());
		if res.is_err() {
			upload_file(n_file, &mut ftp_stream, false, 0);
		}
		else {
			// let sel_file = res.unwrap();
			// if sel_file.synced == false {
			// 	upload_file(n_file, &mut ftp_stream, true, sel_file.id);
			// }
			upload_file(n_file, &mut ftp_stream, false, 0);
		}
	}
	let _ = ftp_stream.quit();
}

fn upload_file(i_file: NFile, mut stream: &mut FtpStream, update: bool, id: i32) {
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

fn delete_ftp_file(file: &NFile) {
	use std::fs::remove_file;
	let fpath = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set") + &file.filename;
	remove_file(fpath).unwrap();
}
