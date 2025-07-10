use sqlite::Connection;
use std::fs::{remove_file, OpenOptions};
use std::path::Path;
use std::process::Command;

pub fn setup(db_name: &str) {
    // If the file exists, remove it
    if Path::new(db_name).exists() {
        teardown(db_name);
    }

    let _file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append data to the file
        .open(db_name) // Open the file
        .unwrap();
}

pub fn teardown(db_name: &str) {
    if Path::new(db_name).exists() {
        if cfg!(windows) {
            Command::new("attrib")
                .arg("-r")
                .arg(db_name)
                .output()
                .expect("Failed to change file permissions");
        } else {
            Command::new("chmod")
                .arg("+w")
                .arg(db_name)
                .output()
                .expect("Failed to change file permissions");
        }
        match remove_file(db_name) {
            Ok(()) => {
                println!("File {db_name} has been removed.");
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}

pub fn create_table(conn: &Connection, table_name: &str) {
    match conn.execute(format!(
        "CREATE TABLE {table_name} (id INTEGER PRIMARY KEY, name TEXT);"
    )) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}

pub fn drop_table(conn: &Connection, table_name: &str) {
    match conn.execute(format!("DROP TABLE IF EXISTS {table_name};")) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
