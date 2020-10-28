// use std::io::{stdout, Write};
use dotenv::dotenv;
use std::convert::TryInto;
use std::env;
use curl::easy::{Easy, Form};
use std::str;

use crate::output;

pub fn api(filename: String, folder: &String, outputline: &u16) -> Result<(),String> {
    dotenv().ok();

//    let mut dst = Vec::new();
    let mut easy = Easy::new();
    let url = env::var("API_URL").expect("API_URL not set") + &"api.php".to_string();
    easy.url(&url).unwrap();
    // println!("{:?}",url);
    let mut form = Form::new();

    let path = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set");

    let fullpath: String;
    if path != "/" {
        fullpath = path.to_owned() + "/" + &filename;
    } else {
        fullpath = path.to_owned() + &filename;
    }

    let ppath: String;
    if folder != "/" {
        ppath = folder.to_owned() + "/" + &filename;
    } else {
        ppath = folder.to_owned() + &filename;
    }
    form.part("file").file(&fullpath).add().expect("error form");
    form.part("fullpath").contents(&ppath.as_bytes()).add().expect("error form");
    easy.httppost(form).unwrap();
    easy.progress(true).unwrap();
    let mut dst = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
        }).unwrap();
        transfer.progress_function(|_,_,exp_upl,curr_upl| {
            let hundred: f64 = 100.try_into().unwrap();
            let proz = (hundred * curr_upl / exp_upl).round();
            let len: usize;
            if proz >= 10.try_into().unwrap() {
                len = 2;
            } else if proz == hundred {
                len = 3;
            } else {
                len = 1;
            }
            let leerz = " ".repeat(len-3);
            let text = format!("uploading file {}%{}", proz, leerz).to_string();
            output::print_in_line(&text, outputline, false);
            true
        }).unwrap();
        transfer.perform().unwrap();
    }
//    println!("{:?}",&dst);
    let answer = str::from_utf8(&dst).unwrap().to_string();
    if answer == "Done." {
        return Ok(());
    }
    else {
        return Err(answer);
    }
}
pub fn get_url() -> String {
        dotenv().ok();
        let ftp_url = env::var("FTP_URL");
        if ftp_url.is_ok() {
            return ftp_url.unwrap();
        }

        let mut dst = Vec::new();
        let mut easy = Easy::new();
        let url = env::var("FTP_GETTER_URL").expect("FTP_GETTER_URL not set");
        easy.url(&url).unwrap();
        let mut form = Form::new();
        form.part("fullpath").contents("".as_bytes()).add().expect("error form");
        easy.httppost(form).unwrap();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
            dst.extend_from_slice(data);
    	    Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        str::from_utf8(&dst).unwrap().to_string()
}
