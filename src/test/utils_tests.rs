use sqlite::Connection;
use std::fs::{remove_file, OpenOptions};
use std::path::Path;
use std::process::Command;

pub fn setup(db_name: &str) {
    if Path::new(db_name).exists() {
        Command::new("chmod")
            .arg("+w")
            .arg(db_name)
            .output()
            .expect("Failed to change file permissions");

        remove_file(db_name).unwrap();
    }

    let _file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append data to the file
        .open(db_name) // Open the file
        .unwrap();
}

pub fn teardown(db_name: &str) {
    if Path::new(db_name).exists() {
        Command::new("chmod")
            .arg("+w")
            .arg(db_name)
            .output()
            .expect("Failed to change file permissions");

        remove_file(db_name).unwrap();
    }
}

pub fn create_table(conn: &Connection, table_name: &str) {
    conn.execute(format!(
        "CREATE TABLE {table_name} (id INTEGER PRIMARY KEY, name TEXT);"
    ))
    .unwrap();
}

pub fn drop_table(conn: &Connection, table_name: &str) {
    conn.execute(format!("DROP TABLE IF EXISTS {table_name};"))
        .unwrap();
}
