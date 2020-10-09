use std::io::{stdout, Write};
use std::env;

pub fn api(filename: String) {
    use curl::easy::{Easy, Form};
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url("https://black616angel.de/upload.php").unwrap();
    let mut form = Form::new();
    let path = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set");
    let fullpath = path + &filename;
    form.part("myFile").file(&fullpath).add().expect("error form");
    easy.httppost(form).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            /*dst.extend_from_slice(data);
            Ok(data.len())*/
	    dst.push(data.to_owned());
	    Ok(stdout().write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("{:?}",&dst);
}
