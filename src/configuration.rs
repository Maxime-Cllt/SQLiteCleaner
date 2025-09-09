use std::fs::Metadata;

/// Configuration object to hold the configuration of the program (e.g. database path)
#[must_use]
pub struct Configuration {
    db_path: String, // Path to the sqlite database file
}

impl Configuration {
    /// Create a new configuration object
    /// # Arguments
    /// * `db_path` - The path to the sqlite database file
    /// # Returns
    /// * A new configuration object
    pub const fn new(db_path: String) -> Self {
        Self { db_path }
    }

    /// Get the configuration object from the command line arguments
    /// # Arguments
    /// * `args` - The command line arguments
    /// # Returns
    /// * A new configuration object
    pub fn get_from_args(args: &[String]) -> Result<Self, std::io::Error> {
        if args.len() != 2 && args.len() != 4 {
            println!("{args:?}");
            println!("The program needs 1 argument: the path to the database file");
            println!("Example: ./SQLiteCleaner \"sqlite_db.db\"");
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The program needs 1 argument: the path to the database file",
            ));
        }

        let db_path: String = match args.len() {
            2 => args[1].clone(),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The program needs 1 argument: the path to the database file",
            ))?,
        };

        let extension: &str = std::path::Path::new(&db_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default();

        if extension.eq_ignore_ascii_case("db") || extension.eq_ignore_ascii_case("sqlite") {
            Ok(Self::new(db_path))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "The database file must be a sqlite file with extension .db or .sqlite, but got: {extension}"
                ),
            ))
        }
    }

    /// Get the path to the database file to use
    /// # Returns
    /// * The path to the database file
    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    /// Get the size of using the file system
    /// # Returns
    /// * The size of the database file
    pub fn get_size_of_database(&self) -> Result<u64, std::io::Error> {
        let fs: Metadata = std::fs::metadata(&self.db_path)?;
        Ok(fs.len())
    }
}
