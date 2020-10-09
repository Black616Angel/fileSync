use ftp::FtpStream;
use crate::sql::models::*;
use std::env;

pub fn get_filelist(ip: String, port: String) -> Vec<File> {

	let url = ip.to_owned() + ":" + &port;
	println!("get Stream");
	let mut ftp_stream = get_stream(url);
	println!("get list");
	let files = get_folder_list(&mut ftp_stream, &mut (&"/").to_string());
	let _ = ftp_stream.quit();
	return files;
}

fn get_folder_list(stream: &mut FtpStream, path: &mut String) -> Vec<File> {
	let mut r_files =  Vec::<File>::new();
	let filelist_str = stream.nlst(Some(&path)).unwrap();
	println!("path: {:?}",path);
	for line in filelist_str {
		let mut abs_path: String;
		if path != "/" {
			abs_path = path.to_owned() + "/" + &line;
		} else {
			abs_path = path.to_owned() + &line;
		}
		let size = stream.size(&abs_path);
		if size.is_ok() {
			let new_file = File { path: path.to_string(), filename: line, id: 0, synced: false, deleted:false }; //, chdate: dat };
			r_files.push(new_file);
		} else {
			if line != "." && line != ".." && line != ".trash" {
				r_files.extend(get_folder_list(stream, &mut abs_path));
			}
		}
		let flen = r_files.len();
		if flen != 0 && flen % 100 == 0 {
			println!("read {} files",r_files.len());
		}
	}
	return r_files.to_vec();
}

fn get_stream (url:String) -> FtpStream {
	let mut ftp_stream = FtpStream::connect(url).unwrap();
	let user = env::var("FTP_USER").expect("ftp-username not set");
	let pass = env::var("FTP_PASS").expect("ftp-password not set");
	ftp_stream.login(&user, &pass).unwrap();
	return ftp_stream;
}
