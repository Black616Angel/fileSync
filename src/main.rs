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
	let ftp_list = myftp::get_filelist(&mut ftp_stream);
	for n_file in ftp_list {
		let file = File { path: n_file.path.to_string(), filename: n_file.filename.to_string(), synced: false, deleted: false, id: 0};
		let sel_file = sql::select_file(file);
		if sel_file.is_err() {
			upload_file(n_file, ftp_stream, false, 0);
		}
		else if sel_file.unwrap().synced == false {
			upload_file(n_file, ftp_stream, true, sel_file.unwrap().id);
		}
	}
	let _ = ftp_stream.quit();
}

fn upload_file(file: N_File, stream: FtpStream, update: bool, id: i32) {
	//get data from ftp
	myftp::get_file(file, &mut stream);
	web::api(file.filename.to_string(), file.path.to_string());
	if update {
		sql::update_synced( id, true);
	} else {
		sql::insert_file( file );
	}
}