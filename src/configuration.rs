use std::fs::Metadata;

/// Configuration object to hold the configuration of the program (e.g. database path)
pub struct Configuration {
    db_path: String, // Path to the sqlite database file
}

impl Configuration {
    /// Create a new configuration object
    pub const fn new(db_path: String) -> Self {
        Self { db_path }
    }

    pub fn get_from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 4 {
            println!("{args:?}");
            std::process::exit(1);
        }
        Self::new(args[1].clone())
    }

    /// Get the path to the database file to use
    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    /// Get the size of using the file system
    pub fn get_size_of_database(&self) -> Result<u64, std::io::Error> {
        let fs: Metadata = std::fs::metadata(&self.db_path)?;
        Ok(fs.len())
    }
}
