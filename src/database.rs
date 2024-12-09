use crate::configuration::Configuration;
use crate::logger::Logger;
use rusqlite::Connection;
use std::time::{Duration, Instant};

/// Open a connection to the database
pub fn open_connection(db_path: &str, logger: &mut Logger) -> Result<Connection, rusqlite::Error> {
    match Connection::open(db_path) {
        Ok(c) => Ok(c),
        Err(e) => {
            logger.log_and_print(&format!("Error opening connection to database: {:?}", e));
            std::process::exit(1);
        }
    }
}

/// Execute an SQL statement
pub fn execute_sql(
    conn: &Connection,
    sql: &str,
    logger: &mut Logger,
) -> Result<(), rusqlite::Error> {
    match conn.execute(sql, []) {
        Ok(_) => Ok(()),
        Err(e) => {
            logger.log_and_print(&format!("Error executing SQL '{}': {:?}", sql, e));
            Err(e)
        }
    }
}

/// Get all tables in the database
pub fn get_all_tables(
    conn: &Connection,
    logger: &mut Logger,
) -> Result<Vec<String>, rusqlite::Error> {
    let sql: &str =
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%';";

    // Query the tables
    let mut result_all_tables: Vec<String> = Vec::new();
    match conn.prepare(sql) {
        Ok(mut stmt) => {
            let table_iter = stmt.query_map([], |row| row.get(0));
            match table_iter {
                Ok(rows) => {
                    for row in rows {
                        match row {
                            Ok(table_name) => result_all_tables.push(table_name),
                            Err(e) => {
                                logger.log_and_print(&format!("Error getting table name: {:?}", e))
                            }
                        }
                    }
                }
                Err(e) => logger.log_and_print(&format!("Error getting table names: {:?}", e)),
            }
        }
        Err(e) => {
            logger.log_and_print(&format!("Error preparing SQL '{}': {:?}", sql, e));
        }
    }
    Ok(result_all_tables)
}

/// Print the end message with the size of the database
pub fn print_report(
    start_time: Instant,
    start_bytes_size: u64,
    config: &Configuration,
    logger: &mut Logger,
) -> () {
    let end_size: u64 = config.get_size_of_database().unwrap_or_default(); // Get the size of the database
    let optimized_bytes: u64 = start_bytes_size - end_size;
    let percentage_of_reduction: f32 = if start_bytes_size == 0 {
        0.0
    } else {
        (optimized_bytes as f32 / start_bytes_size as f32) * 100.0
    };
    let elapsed_time: Duration = start_time.elapsed();
    println!("Size at end {:?} bytes", end_size);
    println!(
        "Total optimized: {:?} bytes, it's reduced by {:.2}% the size",
        optimized_bytes, percentage_of_reduction
    );
    println!("Elapsed time: {:?}", elapsed_time);
    logger.log(&format!(
        "FROM : {:?} bytes, TO : {:?} bytes, OPTIMIZED : {:?} bytes, DURATION : {:?}",
        start_bytes_size, end_size, optimized_bytes, elapsed_time
    ));
}

/// Process the cleaning of the database
pub fn process_db_cleaning(conn: &Connection, logger: &mut Logger) -> Result<(), rusqlite::Error> {
    let result_all_tables: Vec<String>;

    match get_all_tables(&conn, logger) {
        Ok(tables) => result_all_tables = tables,
        Err(e) => {
            logger.log_and_print(&format!("Error getting all tables: {:?}", e));
            return Err(e);
        }
    }

    const REINDEX_SQL: &str = "REINDEX ";
    const ANALYZE_SQL: &str = "ANALYZE ";
    const VACUUM_SQL: &str = "VACUUM ";

    let size: usize = result_all_tables.len();
    for i in 0..size {
        let table_name: &str = &result_all_tables[i];

        let sql_commands: [String; 3] = [
            format!("{}'{}';", &VACUUM_SQL, &table_name),
            format!("{}'{}';", &REINDEX_SQL, &table_name),
            format!("{}'{}';", &ANALYZE_SQL, &table_name),
        ];

        for sql in &sql_commands {
            if let Err(e) = execute_sql(&conn, sql, logger) {
                logger.log_and_print(&format!("Error executing SQL '{}': {:?}", sql, e));
            }
        }
    }
    Ok(())
}
