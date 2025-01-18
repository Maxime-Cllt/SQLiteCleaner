use crate::configuration::Configuration;
use crate::database::{get_all_tables, open_connection};
use crate::logger::Logger;
use crate::test::utils_tests::{create_table, drop_table, setup, teardown};
use sqlite::Connection;
use std::fs::File;
use std::path::Path;

#[test]
fn test_open_connection() {
    const DB_PATH: &str = "open_connection.db";
    setup(DB_PATH);
    assert!(Path::new(DB_PATH).exists());
    assert!(File::open(DB_PATH).is_ok());
    teardown(DB_PATH);
}

#[test]
fn test_get_all_tables() {
    const DB_PATH: &str = "get_all_tables.db";
    setup(DB_PATH);
    let logger: Logger = Logger::new();
    let conn: Connection = open_connection(DB_PATH, &logger);
    let tables: Vec<String> = get_all_tables(&conn, &logger).unwrap();
    assert_eq!(tables.len(), 0);

    create_table(&conn, "users");
    create_table(&conn, "posts");

    let tables: Vec<String> = get_all_tables(&conn, &logger).unwrap();
    assert_eq!(tables.len(), 2);

    assert!(tables.contains(&"users".to_string()));
    assert!(tables.contains(&"posts".to_string()));

    drop_table(&conn, "users");
    drop_table(&conn, "posts");

    let tables: Vec<String> = get_all_tables(&conn, &logger).unwrap();
    assert_eq!(tables.len(), 0);

    teardown(DB_PATH);
}

#[test]
fn test_execute_sql() {
    const DB_PATH: &str = "execute_sql.db";
    setup(DB_PATH);
    let logger: Logger = Logger::new();
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    assert!(Path::new(DB_PATH).exists());

    let sql: &str = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
    crate::database::execute_sql(&conn, sql, &logger).unwrap();
    let tables: Vec<String> = get_all_tables(&conn, &logger).unwrap();
    assert_eq!(tables.len(), 1);
    assert!(tables.contains(&"users".to_string()));

    let sql: &str = "DROP TABLE IF EXISTS users;";
    crate::database::execute_sql(&conn, sql, &logger).unwrap();
    let tables: Vec<String> = get_all_tables(&conn, &logger).unwrap();
    assert_eq!(tables.len(), 0);

    teardown(DB_PATH);
}

#[test]
fn test_get_from_args() {
    const EXE_PATH: &str = "./SqliteCleaner";
    const DB_PATH: &str = "get_from_args.db";
    const EXTRA: &str = "extra";

    setup(DB_PATH);

    let args_1: Vec<String> = vec![EXE_PATH.to_string()];
    let args_2: Vec<String> = vec![EXE_PATH.to_string(), DB_PATH.to_string()];
    let args_3: Vec<String> = vec![EXE_PATH.to_string(), DB_PATH.to_string(), EXTRA.to_string()];
    let args_4: Vec<String> = vec![
        EXE_PATH.to_string(),
        DB_PATH.to_string(),
        EXTRA.to_string(),
        EXTRA.to_string(),
    ];

    let config_1: Result<Configuration, std::io::Error> = Configuration::get_from_args(&args_1);
    let config_2: Result<Configuration, std::io::Error> = Configuration::get_from_args(&args_2);
    let config_3: Result<Configuration, std::io::Error> = Configuration::get_from_args(&args_3);
    let config_4: Result<Configuration, std::io::Error> = Configuration::get_from_args(&args_4);

    assert!(config_1.is_err());
    assert!(config_2.is_ok());
    assert!(config_3.is_err());
    assert!(config_4.is_ok());

    assert_eq!(config_2.unwrap().get_db_path(), DB_PATH);
    assert_eq!(config_4.unwrap().get_db_path(), DB_PATH);

    teardown(DB_PATH);
}

#[test]
fn test_get_size_of_database() {
    const DB_PATH: &str = "get_size_of_database.db";
    setup(DB_PATH);

    let conn: Connection = Connection::open(DB_PATH).unwrap();

    assert!(Path::new(DB_PATH).exists());

    let config: Configuration = Configuration::new(DB_PATH.to_string());
    let size: u64 = config.get_size_of_database().unwrap();
    assert_eq!(config.get_db_path(), DB_PATH);
    assert_eq!(size, 0);

    create_table(&conn, "users");
    create_table(&conn, "posts");

    let size: u64 = config.get_size_of_database().unwrap();
    assert_ne!(size, 0);

    teardown(DB_PATH);
}
