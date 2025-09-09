use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Mutex, MutexGuard};

#[non_exhaustive]
#[must_use]
pub struct Logger {
    log_file: File,
}

impl Logger {
    /// Create a new logger object
    fn new() -> Self {
        let log_file: File = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("SQLiteCleaner.log")
            .unwrap();

        Self { log_file }
    }

    /// Log a message to the log file
    fn log(&self, message: &str) {
        let mut log_writer: BufWriter<&File> = BufWriter::new(&self.log_file);
        writeln!(log_writer, "[{}] {message}", chrono::Local::now()).unwrap();
    }

    /// Log a message to the log file and print it to the console
    fn log_and_print(&self, message: &str) {
        println!("{message}");
        self.log(message);
    }
}

/// Static logger instance
pub static LOGGER: std::sync::LazyLock<Mutex<Logger>> = std::sync::LazyLock::new(|| Mutex::new(Logger::new()));

/// Static function to log a message
pub fn log_message(message: &str) {
    let logger: MutexGuard<Logger> = LOGGER.lock().unwrap();
    logger.log(message);
}

/// Static function to log a message and print it to the console
pub fn log_and_print_message(message: &str) {
    let logger: MutexGuard<Logger> = LOGGER.lock().unwrap();
    logger.log_and_print(message);
}