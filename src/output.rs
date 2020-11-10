use crossterm::{QueueableCommand, cursor};
use crossterm::terminal::{Clear, ClearType};
use std::{thread, time};
use dotenv::dotenv;
use std::env;
use chrono::*;
use lazy_static::lazy_static;
use std::io::Write;
use std::io::{stdout, Stdout};
//use tokio::io::{stdout, Stdout};
use std::sync::Mutex;

// static GLOBAL_THREAD_LOCK: AtomicUsize = AtomicUsize::new(0);
lazy_static! {
    static ref GLOBAL_TIME: String = Utc::now().to_rfc3339();
  static ref GLOBAL_THREAD_LOCK: Mutex<i32> = Mutex::new(0);
}

pub fn print_in_line(text: &String, line: &u16, clear: bool) {
	while GLOBAL_THREAD_LOCK.try_lock().is_err() {
		thread::sleep(time::Duration::from_millis(100));
	}
	
	let mut stdout = stdout();
	
//    stdout.execute(cursor::SavePosition).expect("");
	go_to_line(&mut stdout, line, true);
	if clear {
		stdout.queue(Clear(ClearType::CurrentLine)).expect("");
	}
	let otext: String = text.chars().take(60).collect();
	
//	writeln!(lck, "{}", text).unwrap();
	println!("{}", otext);
	go_to_line(&mut stdout, line, false);
//	stdout.execute(cursor::RestorePosition).expect("");
	// drop(GLOBAL_THREAD_LOCK);
}

pub fn print_log(text: String) {
    dotenv().ok();
    let mut filename = env::var("LOG_FILE").unwrap();
    filename = filename.replace("[timestamp]", &*GLOBAL_TIME);
    let file_res = std::fs::OpenOptions::new().append(true).open(&filename);
	let mut file: std::fs::File;
	if file_res.is_ok() {
		file = file_res.unwrap();
	} else {
		file = std::fs::File::create(filename).unwrap();
	}
    file.write_all(text.as_bytes()).unwrap();
}

fn go_to_line(sout: &mut Stdout, line: &u16, up: bool) {
	if up {
<<<<<<< HEAD
		stdout().execute(cursor::MoveToPreviousLine(*line)).expect("");
	} else
	{
		if stdout().execute(cursor::MoveToNextLine(*line)).is_err() {
            //we ignore that alltogether
        }
=======
		sout.queue(cursor::MoveToPreviousLine(*line)).expect("");
	} else 
	{
		sout.queue(cursor::MoveToNextLine(*line)).is_err();
>>>>>>> 637a536cdf4ae07c9a0d6cdb435e02439c0dceb4
	}
}
