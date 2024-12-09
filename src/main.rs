mod configuration;
mod database;
mod logger;
#[cfg(test)]
mod test;

use crate::configuration::Configuration;
use crate::database::{open_connection, print_report, process_db_cleaning};
use crate::logger::Logger;
use rusqlite::Connection;
use std::time::Instant;

fn main() {
    let start_time: Instant = Instant::now();
    let mut logger: Logger = Logger::new();
    let config: Configuration = match Configuration::get_from_args() {
        Ok(c) => c,
        Err(e) => {
            logger.log_and_print(&format!("Error getting configuration: {:?}", e));
            return;
        }
    };

    let conn: Connection = match open_connection(config.get_db_path(), &mut logger) {
        Ok(c) => c,
        Err(e) => {
            logger.log_and_print(&format!("Error opening connection to database: {:?}", e));
            return;
        }
    };

    let start_bytes_size: u64 = config.get_size_of_database().unwrap_or_default();

    println!("Optimizing database...");
    println!("Size at start {:?} bytes", start_bytes_size);

    match process_db_cleaning(&conn, &mut logger) {
        Ok(_) => (),
        Err(e) => {
            logger.log_and_print(&format!("Error processing the cleaning: {:?}", e));
            return;
        }
    }

    print_report(start_time, start_bytes_size, &config, &mut logger);
    return;
}
