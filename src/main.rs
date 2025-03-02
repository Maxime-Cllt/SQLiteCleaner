mod configuration;
mod database;
mod logger;
#[cfg(test)]
mod test;

use crate::configuration::Configuration;
use crate::database::{open_connection, print_report, process_db_cleaning};
use crate::logger::{log_and_print_message};
use num_format::{Locale, ToFormattedString};
use sqlite::Connection;
use std::time::Instant;

fn main() {
    let start_time: Instant = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let config: Configuration = match Configuration::get_from_args(&args) {
        Ok(c) => c,
        Err(e) => {
            log_and_print_message(&format!("Error getting configuration: {e:?}"));
            std::process::exit(1);
        }
    };

    let conn: Connection = open_connection(config.get_db_path());
    let start_bytes_size: u64 = config.get_size_of_database().unwrap_or_default();

    println!("Optimizing database...");
    println!(
        "Size at start {:?} bytes",
        start_bytes_size.to_formatted_string(&Locale::en)
    );

    match process_db_cleaning(&conn) {
        Ok(()) => (),
        Err(e) => {
            log_and_print_message(&format!("Error processing the cleaning: {e:?}"));
            return;
        }
    }

    print_report(start_time, start_bytes_size, &config);
}
