use serde::Deserialize;
use std::fs::Metadata;

/// Configuration object for the application, loaded from a configuration file

#[derive(Debug, Deserialize)]
pub struct Configuration {
    db_path: String,
}

impl Configuration {
    /// Create a new configuration object
    pub fn new(db_path: String) -> Configuration {
        Configuration { db_path }
    }

    pub fn get_from_args() -> Configuration {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 4 {
            println!("{:?}", args);
            std::process::exit(1);
        }
        Configuration::new(args[1].clone())
    }

    /// Get the path to the database file to use
    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    /// Get the size of using the file system
    pub fn get_size_of_database(&self) -> u64 {
        let fs: Metadata = std::fs::metadata(&self.db_path).unwrap();
        fs.len()
    }
}
