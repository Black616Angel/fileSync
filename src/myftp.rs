use ftp::FtpStream;
use crate::sql::models::*;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{stdout, Write};
use crossterm::{ExecutableCommand, cursor};

use crate::web;

pub fn get_filelist(mut stream: &mut FtpStream) -> Vec<NFile> {
	println!("get list");
	let mut dir: String = env::var("FTP_DIR").expect("FTP_DIR not set");
	let files = get_folder_list(&mut stream, &mut dir);
	return files;
}

pub fn get_file (file: &NFile, stream: &mut FtpStream) {
	let ftp_path: String;
	if file.path != "/" {
		ftp_path = file.path.to_owned() + "/" + &file.filename;
	} else {
		ftp_path = file.path.to_owned() + &file.filename;
	}
	let path = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set");
	let fullpath = path + &file.filename;
	let mut fs_file = fs::File::create(fullpath).expect("error creating file");
	stream.retr_with_file(&ftp_path, &mut fs_file, |stream, file| {
		for byte in stream.bytes() {
			file.write_all(&[byte.unwrap()]).unwrap();
		}
		Ok(())
	}).expect("impossible");
}

pub fn get_stream () -> FtpStream {
	let ftp_url = web::get_url();
	let url = ftp_url.to_owned() + ":" + &"21".to_string();
	let mut ftp_stream = FtpStream::connect(url).unwrap();
	let user = env::var("FTP_USER").expect("FTP_USER not set");
	let pass = env::var("FTP_PASS").expect("FTP_PASS not set");
	ftp_stream.login(&user, &pass).unwrap();
	return ftp_stream;
}

fn get_folder_list(stream: &mut FtpStream, path: &mut String) -> Vec<NFile> {
	let mut r_files =  Vec::<NFile>::new();
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
			let new_file = NFile { path: path.to_string(), filename: line };
			r_files.push(new_file);
		} else {
			if line != "." && line != ".." && line != ".trash" {
				r_files.extend(get_folder_list(stream, &mut abs_path));
			}
		}
		let flen = r_files.len();
		if flen != 0 && flen % 100 == 0 {
			print!("read {} files",r_files.len());
			stdout().execute(cursor::MoveToColumn(0)).expect("");
		}
	}
	return r_files.to_vec();
}
