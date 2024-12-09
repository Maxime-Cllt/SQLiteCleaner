#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;
    use crate::database::{get_all_tables, open_connection};
    use crate::logger::Logger;
    use once_cell::sync::Lazy;
    use rusqlite::Connection;
    use std::fs::{remove_file, File, OpenOptions};
    use std::path::Path;
    use std::process::Command;
    use std::sync::Mutex;
    use serial_test::serial;

    const DB_PATH: &str = "for_test.db";

    static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));

    fn setup() {
        if Path::new(DB_PATH).exists() {
            Command::new("chmod")
                .arg("+w")
                .arg(DB_PATH)
                .output()
                .expect("Failed to change file permissions");

            remove_file(DB_PATH).unwrap();
        }

        let _file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(DB_PATH)
            .unwrap();
    }

    fn teardown() {
        if Path::new(DB_PATH).exists() {
            Command::new("chmod")
                .arg("+w")
                .arg(DB_PATH)
                .output()
                .expect("Failed to change file permissions");

            remove_file(DB_PATH).unwrap();
        }
    }

    fn create_table(conn: &Connection, table_name: &str) {
        conn.execute_batch(&format!(
            "CREATE TABLE {} (id INTEGER PRIMARY KEY, name TEXT);",
            table_name
        ))
        .unwrap();
    }

    fn drop_table(conn: &Connection, table_name: &str) {
        conn.execute_batch(&format!("DROP TABLE IF EXISTS {};", table_name))
            .unwrap();
    }

    #[test]
    #[serial]
    fn test_open_connection() {
        setup();
        assert!(Path::new(DB_PATH).exists());
        assert!(File::open(DB_PATH).is_ok());
        teardown();
    }

    #[test]
    #[serial]
    fn test_get_all_tables() {
        setup();
        let mut logger = LOGGER.lock().unwrap();
        let conn: Connection = open_connection(DB_PATH, &mut *logger).unwrap();
        let tables: Vec<String> = get_all_tables(&conn, &mut *logger).unwrap();
        assert_eq!(tables.len(), 0);

        create_table(&conn, "users");
        create_table(&conn, "posts");

        let tables: Vec<String> = get_all_tables(&conn, &mut *logger).unwrap();
        assert_eq!(tables.len(), 2);

        assert!(tables.contains(&"users".to_string()));
        assert!(tables.contains(&"posts".to_string()));

        drop_table(&conn, "users");
        drop_table(&conn, "posts");

        let tables: Vec<String> = get_all_tables(&conn, &mut *logger).unwrap();
        assert_eq!(tables.len(), 0);

        teardown();
    }

    #[test]
    #[serial]
    fn test_execute_sql() {
        setup();
        let mut logger = LOGGER.lock().unwrap();
        let conn: Connection = Connection::open(DB_PATH).unwrap();

        assert!(Path::new(DB_PATH).exists());

        let sql: &str = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
        crate::database::execute_sql(&conn, sql, &mut *logger).unwrap();
        let tables: Vec<String> = get_all_tables(&conn, &mut *logger).unwrap();
        assert_eq!(tables.len(), 1);
        assert!(tables.contains(&"users".to_string()));

        let sql: &str = "DROP TABLE IF EXISTS users;";
        crate::database::execute_sql(&conn, sql, &mut *logger).unwrap();
        let tables: Vec<String> = get_all_tables(&conn, &mut *logger).unwrap();
        assert_eq!(tables.len(), 0);

        teardown();
    }

    #[test]
    #[serial]
    fn test_get_size_of_database() {
        setup();

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

        teardown();
    }
}
