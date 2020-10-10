use std::io::{stdout, Write};
use dotenv::dotenv;
use std::env;

pub fn api(filename: String, folder: String) {
    dotenv().ok();

    use curl::easy::{Easy, Form};
//    let mut dst = Vec::new();
    let mut easy = Easy::new();
    let url = env::var("API_URL").expect("API_URL not set") + &"api.php".to_string();
    easy.url(&url).unwrap();
    println!("{:?}",url);
    let mut form = Form::new();

    let path = env::var("FTP_UPLOAD_PATH").expect("FTP_UPLOAD_PATH not set");
    let fullpath = path + &filename;
    let ppath = folder + &filename;
    form.part("file").file(&fullpath).add().expect("error form");
    form.part("fullpath").contents(&ppath.as_bytes()).add().expect("error form");
    easy.httppost(form).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            /*dst.extend_from_slice(data);
            Ok(data.len())*/
//	    dst.push(data.to_owned());
	    Ok(stdout().write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }
//    println!("{:?}",&dst);
}
