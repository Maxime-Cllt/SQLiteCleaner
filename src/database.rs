use crate::configuration::Configuration;
use crate::logger::Logger;
use sqlite::Connection;
use std::time::{Duration, Instant};

/// Open a connection to the database
pub fn open_connection(db_path: &str, logger: &Logger) -> Connection {
    match Connection::open(db_path) {
        Ok(c) => c,
        Err(e) => {
            logger.log_and_print(&format!("Error opening connection to database: {e:?}"));
            std::process::exit(1);
        }
    }
}

/// Execute an SQL statement
pub fn execute_sql(conn: &Connection, sql: &str, logger: &Logger) -> Result<(), sqlite::Error> {
    match conn.execute(sql) {
        Ok(()) => Ok(()),
        Err(e) => {
            logger.log_and_print(&format!("Error executing SQL '{sql}': {e:?}"));
            Err(e)
        }
    }
}

/// Get all tables in the database that are not system tables
pub fn get_all_tables(conn: &Connection, logger: &Logger) -> Result<Vec<String>, sqlite::Error> {
    const QUERY_ALL_TABLE_SQL: &str =
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%';";

    // Query the tables
    let mut result_all_tables: Vec<String> = Vec::new();

    match conn.iterate(QUERY_ALL_TABLE_SQL, |pairs| {
        for &(_, value) in pairs {
            if let Some(value) = value {
                result_all_tables.push(value.to_string());
            }
        }
        true
    }) {
        Ok(()) => Ok(result_all_tables),
        Err(e) => {
            logger.log_and_print(&format!("Error getting all tables: {e:?}"));
            Err(e)
        }
    }
}

/// Print the end message with the size of the database
pub fn print_report(
    start_time: Instant,
    start_bytes_size: u64,
    config: &Configuration,
    logger: &Logger,
) {
    let end_size: u64 = config.get_size_of_database().unwrap_or_default(); // Get the size of the database
    let optimized_bytes: u64 = start_bytes_size - end_size;
    let percentage_of_reduction: u64 = if start_bytes_size == 0 || (end_size < start_bytes_size) {
        0
    } else {
        (optimized_bytes * 100) / start_bytes_size
    };

    let elapsed_time: Duration = start_time.elapsed();
    println!("Size at end {end_size:?} bytes");
    println!(
        "Total optimized: {optimized_bytes:?} bytes, it's reduced by {percentage_of_reduction:?}% the size"
    );
    println!("Elapsed time: {elapsed_time:?}");
    logger.log(&format!(
        "FROM : {start_bytes_size:?} bytes, TO : {end_size:?} bytes, OPTIMIZED : {optimized_bytes:?} bytes, DURATION : {elapsed_time:?}"
    ));
}

/// Process the cleaning of the database
pub fn process_db_cleaning(conn: &Connection, logger: &Logger) -> Result<(), sqlite::Error> {
    const REINDEX_SQL: &str = "REINDEX ";
    const ANALYZE_SQL: &str = "ANALYZE ";
    const VACUUM_SQL: &str = "VACUUM ";

    let result_all_tables: Vec<String> = match get_all_tables(conn, logger) {
        Ok(tables) => tables,
        Err(e) => {
            logger.log_and_print(&format!("Error getting all tables: {e:?}"));
            return Err(e);
        }
    };

    for table_name in &result_all_tables {
        let sql_commands: [String; 3] = [
            format!("{}'{}';", &VACUUM_SQL, &table_name),
            format!("{}'{}';", &REINDEX_SQL, &table_name),
            format!("{}'{}';", &ANALYZE_SQL, &table_name),
        ];

        for sql in &sql_commands {
            if let Err(e) = execute_sql(conn, sql, logger) {
                logger.log_and_print(&format!("Error executing SQL '{sql}': {e:?}"));
            }
        }
    }
    Ok(())
}
