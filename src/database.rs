use crate::configuration::Configuration;
use crate::logger::{log_and_print_message, log_message};
use num_format::{Locale, ToFormattedString};
use sqlite::Connection;
use std::time::{Duration, Instant};

/// Open a connection to the database
/// # Arguments
/// * `db_path` - The path to the database
/// * `logger` - The logger to log messages
/// # Returns
/// A connection to the database
pub fn open_connection(db_path: &str) -> Connection {
    match Connection::open(db_path) {
        Ok(c) => c,
        Err(e) => {
            log_and_print_message(&format!("Error opening connection to database: {e:?}"));
            std::process::exit(1);
        }
    }
}

/// Execute an SQL statement
#[inline]
pub fn execute_sql(conn: &Connection, sql: &str) -> Result<(), sqlite::Error> {
    match conn.execute(sql) {
        Ok(()) => Ok(()),
        Err(e) => {
            log_and_print_message(&format!("Error executing SQL '{sql}': {e:?}"));
            Err(e)
        }
    }
}

/// Get all tables in the database that are not system tables
/// # Arguments
/// * `conn` - The connection to the database
/// * `logger` - The logger to log messages
/// # Returns
/// A vector with the names of the tables
pub fn get_all_tables(conn: &Connection) -> Result<Vec<String>, sqlite::Error> {
    const QUERY_ALL_TABLE_SQL: &str =
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%';";

    let mut result_all_tables: Vec<String> = Vec::new(); // Query the tables

    match conn.iterate(QUERY_ALL_TABLE_SQL, |pairs| {
        for &(_, value) in pairs {
            if let Some(value) = value {
                result_all_tables.push(value.into());
            }
        }
        true
    }) {
        Ok(()) => Ok(result_all_tables),
        Err(e) => {
            log_and_print_message(&format!("Error getting all tables: {e:?}"));
            Err(e)
        }
    }
}

/// Print the end message with the size of the database
/// # Arguments
/// * `start_time` - The start time of the process
/// * `start_bytes_size` - The size of the database at the start
/// * `config` - The configuration of the process
/// * `logger` - The logger to log messages
/// # Returns
/// The result
pub fn print_report(start_time: Instant, start_bytes_size: u64, config: &Configuration) {
    let end_size: u64 = config.get_size_of_database().unwrap_or_default(); // Get the size of the database
    let optimized_bytes: u64 = if start_bytes_size == 0 || end_size > start_bytes_size {
        0
    } else {
        start_bytes_size - end_size
    };
    let percentage_of_reduction: u64 = if start_bytes_size == 0 || end_size > start_bytes_size {
        0
    } else {
        (optimized_bytes * 100) / start_bytes_size
    };

    let elapsed_time: Duration = start_time.elapsed();

    println!(
        "Size at end {:?} bytes",
        end_size.to_formatted_string(&Locale::en)
    );
    println!(
        "Total optimized: {:?} bytes, it's reduced by {percentage_of_reduction:?}% the size",
        optimized_bytes.to_formatted_string(&Locale::en)
    );
    println!("Elapsed time: {elapsed_time:?}");
    let json_log: String = format!(
        r#"{{"from_bytes": {start_bytes_size},"to_bytes": {end_size},"optimized_bytes": {optimized_bytes},"percentage_of_reduction": {percentage_of_reduction},"elapsed_time_ms": "{:?}"}}"#,
        elapsed_time.as_millis(),
    );
    log_message(&json_log);
}

/// Process the cleaning of the database
/// # Arguments
/// * `conn` - The connection to the database
/// * `logger` - The logger to log messages
/// # Returns
/// The result
pub fn process_db_cleaning(conn: &Connection) -> Result<(), sqlite::Error> {
    const REINDEX_SQL: &str = "REINDEX ";
    const ANALYZE_SQL: &str = "ANALYZE ";
    const VACUUM_SQL: &str = "VACUUM ";

    let result_all_tables: Vec<String> = match get_all_tables(conn) {
        Ok(tables) => tables,
        Err(e) => {
            log_and_print_message(&format!("Error getting all tables: {e:?}"));
            return Err(e);
        }
    };

    for table_name in &result_all_tables {
        let sql_commands: [String; 3] = [
            format!("{VACUUM_SQL}'{table_name}';"),
            format!("{REINDEX_SQL}'{table_name}';"),
            format!("{ANALYZE_SQL}'{table_name}';"),
        ];

        for sql in &sql_commands {
            if let Err(e) = execute_sql(conn, sql) {
                log_and_print_message(&format!("Error executing SQL '{sql}': {e:?}"));
            }
        }
    }
    Ok(())
}
