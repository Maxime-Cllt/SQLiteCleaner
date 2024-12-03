mod configuration;
mod database;
mod logger;
mod test;

use crate::configuration::Configuration;
use crate::database::{open_connection, print_report, process_db_cleaning};
use crate::logger::Logger;
use rusqlite::Connection;
use std::time::Instant;

fn main() {
    let start_time: Instant = Instant::now();
    let mut logger: Logger = Logger::new();
    let config: Configuration = Configuration::get_from_args(); // Get the configuration object
    let conn: Connection = open_connection(config.get_db_path(), &mut logger); // Open the connection to the database
    let start_size: u64 = config.get_size_of_database();

    println!("Optimizing database...");
    println!("Size at start {:?} bytes", start_size);
    process_db_cleaning(&conn, &mut logger); // Process the database cleaning
    print_report(start_time, start_size, &config, &mut logger);
    return;
}
