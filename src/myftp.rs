
use ftp::FtpStream;
use ftp::openssl::ssl::{ SslContext, SslMethod };
use crate::sql::models::*;
use std::env;

pub fn get_filelist(ip: String, port: String) -> Vec<File> {

	let mut files = Vec::<File>::new();
	let url = format!("{:?}:{:?}",ip,port);
	let mut ftp_stream = get_stream(url);
	let filelist_str = ftp_stream.list(Some(""));
	for line in filelist_str {
		println!("{:?}",line);
	}
	let _ = ftp_stream.quit();
	return files;
}

//fn get_folder_list(stream:FtpStream, files:Vec<File>, path:String) -> Vec<File> {
//	stream.list(path);
//}

fn get_stream (url:String) -> FtpStream {

	let ftp_stream = FtpStream::connect(url).unwrap();
	let ctx = SslContext::builder(SslMethod::tls()).unwrap().build();
	// Switch to the secure mode
	let user = env::var("FTP_USER");
	let pass = env::var("FTP_PASS");
	let mut ftp_stream = ftp_stream.into_secure(ctx).unwrap();
	ftp_stream.login(user, pass).unwrap();
}
