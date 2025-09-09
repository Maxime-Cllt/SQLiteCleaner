<div align="center">
<h1>SQLiteCleaner</h1>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Sqlite-Cleaner-53a863?style=for-the-badge" alt="Database Cleaner" />
    <img src="https://img.shields.io/badge/Version-1.0.1-informational?style=for-the-badge" alt="Version" />
</div>

## 📜 Description

SQLiteCleaner is a program designed to clean your SQLite database efficiently. It reduces database storage and
optimizes all tables (except system tables). Built with Rust, it is compatible with all platforms, ensuring smooth
operation across servers and applications. Download the program from the releases section and keep your database in top
shape!

## ✨ Features

- 🗃️ Reduce database storage but don't delete any data.
- 🚀 Optimize all tables except system tables.
- 💡 Simple and efficient way to clean SQLite databases.
- 🌍 Cross-platform compatibility.
- ⚙️ Maintain databases in optimal condition.
- ❌ No need for dumps or backups.
- 🔒 Does not modify your configuration files.
- 🖥️ Easily run on any server or application.
- 🛠️ User-friendly and easy to use.

## 💻 Platform Support

<div align="center">
  <a href="#macos">
    <img src="https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white&labelColor=gray" alt="macOS" />
  </a>
  <a href="#linux">
    <img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black&labelColor=gray" alt="Linux" />
  </a>
  <a href="#windows">  
    <img src="https://img.shields.io/badge/Windows-0078D4?style=for-the-badge&logo=windows&logoColor=white&labelColor=gray" alt="Windows" />
  </a>
</div>

## 🖼️ Example of Execution

<div align="center">
<img src="assets/Example.png" alt="Example" width="500px" height="auto" />
</div>

## 📥 Installation

To run the program:

1. Clone the repository:

   ```bash
   git clone https://github.com/Maxime-Cllt/SQLiteCleaner.git
   ```

2. Build the program:

   ```bash
   cargo build --release
   ```

3. Execute the program:

   You may need to give the program execute permissions on Linux and macOS:

   ```bash
   chmod +x target/release/SQLiteCleaner
   ```

### MacOS & Linux

```bash
./target/release/SQLiteCleaner "path/to/your_database.db"
```

### Windows

```bash
.\target\release\SQLiteCleaner.exe "path/to/your_database.db"
```

### Cargo

You can also run the program using Cargo:

```bash
cargo run --release -- "path/to/your_database.db"
```

## 🧪 Code Quality

### Unit Tests available

To run unit tests, use the following command:

```bash
cargo test
```

## 📝 Notes

- ⏱️ Fast execution.
- ⚠️ Does not clean triggers, stored procedures, functions, and views.
- 📉 May not reduce storage significantly but is quick to run and can be executed frequently.

## 🔗 See Also

- 🌐 [DBMSCleaner](https://github.com/Maxime-Cllt/DBMSCleaner)
