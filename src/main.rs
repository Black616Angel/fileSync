#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate ftp;

pub mod sql;
pub mod myftp;

fn main() {
	let ftp_ip = sql::get_ip();
	println!("{:?}",ftp_ip);
	let filelist = myftp::get_filelist(ftp_ip, "21".to_string());
}
