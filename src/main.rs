#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate ftp;
extern crate curl;

pub mod sql;
pub mod myftp;
pub mod web;

fn main() {
	println!("get ftp url");
	web::api("main.rs".to_string());
	let ftp_ip = sql::get_ip();
	//let sql_list = sql::get_filelist();
	println!("get file list");
	let ftp_list = myftp::get_filelist(ftp_ip, "21".to_string());
	for file in ftp_list {
		sql::select_file(file);
//		println!("{:?}/{:?}",file.path,file.filename);
	}
}
