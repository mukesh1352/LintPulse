use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn init_log(log_file: &str) {
    let _ = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .expect("Failed to create log file");
}

pub fn log_message(log_file: &str, msg: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .expect("Unable to open log file");

    let entry = format!("{}: {}\n", Local::now(), msg);
    file.write_all(entry.as_bytes()).expect("Failed to write log");
}
