use crossterm::{ExecutableCommand, cursor};
use crossterm::terminal::{Clear, ClearType};
use std::convert::TryInto;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};
use dotenv::dotenv;
use std::env;
use chrono::*;
use lazy_static::lazy_static;
use std::io::{stdout, Write};

static GLOBAL_THREAD_LOCK: AtomicUsize = AtomicUsize::new(0);
lazy_static! {
    static ref GLOBAL_TIME: String = Utc::now().to_rfc3339();
}

pub fn print_in_line(text: &String, line: &u16, clear: bool) {
    while GLOBAL_THREAD_LOCK.fetch_add(1, Ordering::SeqCst) > 0.try_into().unwrap() {
        GLOBAL_THREAD_LOCK.fetch_sub(1, Ordering::SeqCst);
        thread::sleep(time::Duration::from_millis(20));
    }

    let mut stdout = stdout();
    stdout.execute(cursor::SavePosition).expect("");
    go_to_line(line);
    if clear {
        stdout.execute(Clear(ClearType::CurrentLine)).expect("");
    }
    let otext: String = text.chars().take(30).collect();
    println!("{}",otext);
	stdout.execute(cursor::RestorePosition).expect("");
	GLOBAL_THREAD_LOCK.fetch_sub(1, Ordering::SeqCst);
}

pub fn print_log(text: String) {
    dotenv().ok();
    let mut filename = env::var("LOG_FILE").unwrap();
    filename = filename.replace("[timestamp]", &*GLOBAL_TIME);
    let mut file = std::fs::OpenOptions::new().append(true).open(filename).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

fn go_to_line(line: &u16) {
    stdout().execute(cursor::MoveToPreviousLine(*line)).expect("");
}
