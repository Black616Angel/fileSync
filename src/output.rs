use crossterm::{ExecutableCommand, cursor};
use crossterm::terminal::{Clear, ClearType};
use std::{thread, time};
use dotenv::dotenv;
use std::env;
use chrono::*;
use lazy_static::lazy_static;
use std::io::{stdout, Write};
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
	go_to_line(line, true);
	if clear {
		stdout.execute(Clear(ClearType::CurrentLine)).expect("");
	}
	let otext: String = text.chars().take(60).collect();
	println!("{}",otext);
	go_to_line(line, false);
//	stdout.execute(cursor::RestorePosition).expect("");
	// drop(GLOBAL_THREAD_LOCK);
}

pub fn print_log(text: String) {
    dotenv().ok();
    let mut filename = env::var("LOG_FILE").unwrap();
    filename = filename.replace("[timestamp]", &*GLOBAL_TIME);
    let mut file = std::fs::OpenOptions::new().append(true).open(filename).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

fn go_to_line(line: &u16, up: bool) {
	if up {
		stdout().execute(cursor::MoveToPreviousLine(*line)).expect("");
	} else 
	{
		stdout().execute(cursor::MoveToNextLine(*line)).is_err();
	}
}
