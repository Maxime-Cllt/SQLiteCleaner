// Logger class

use std::fs::File;
use std::io::Write;

pub struct Logger {
    log_file: File,
}

impl Logger {
    /// Create a new logger object
    pub fn new() -> Logger {
        let log_file: File = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("SqltieClient.log")
            .unwrap();

        Logger { log_file }
    }

    /// Log a message to the log file
    pub fn log(&mut self, message: &str) {
        let mut log_writer = std::io::BufWriter::new(&self.log_file);
        writeln!(log_writer, "[{}] {}", chrono::Local::now(), message).unwrap();
    }

    /// Log a message to the log file and print it to the console
    pub fn log_and_print(&mut self, message: &str) {
        println!("{}", message);
        self.log(message);
    }
}
