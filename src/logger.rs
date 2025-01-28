use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Logger {
    log_file: File,
}

impl Logger {
    /// Create a new logger object
    /// # Returns
    /// A new logger object
    pub fn new() -> Self {
        let log_file: File = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("SQLiteCleaner.log")
            .unwrap();

        Self { log_file }
    }

    /// Log a message to the log file
    /// # Arguments
    /// * `message` - The message to log
    pub fn log(&self, message: &str) {
        let mut log_writer: BufWriter<&File> = BufWriter::new(&self.log_file);
        writeln!(log_writer, "[{}] {message}", chrono::Local::now()).unwrap();
    }

    /// Log a message to the log file and print it to the console
    /// # Arguments
    /// * `message` - The message to log and print
    pub fn log_and_print(&self, message: &str) {
        println!("{message}");
        self.log(message);
    }
}
