mod configuration;
mod database;
mod logger;
#[cfg(test)]
mod test;

use crate::configuration::Configuration;
use crate::database::{open_connection, print_report, process_db_cleaning};
use crate::logger::Logger;
use sqlite::Connection;
use std::time::Instant;

fn main() {
    let start_time: Instant = Instant::now();
    let logger: Logger = Logger::new();
    let config: Configuration = Configuration::get_from_args();

    let conn: Connection = open_connection(config.get_db_path(), &logger);

    let start_bytes_size: u64 = config.get_size_of_database().unwrap_or_default();

    println!("Optimizing database...");
    println!("Size at start {start_bytes_size:?} bytes");

    match process_db_cleaning(&conn, &logger) {
        Ok(()) => (),
        Err(e) => {
            logger.log_and_print(&format!("Error processing the cleaning: {e:?}"));
            return;
        }
    }

    print_report(start_time, start_bytes_size, &config, &logger);
}
