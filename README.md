<div align="center">
<h1>Sqlite Cleaner</h1>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Sqlite-Cleaner-53a863?style=for-the-badge" alt="Database Cleaner" />
    <img src="https://img.shields.io/badge/Version-1.0.0-informational?style=for-the-badge" alt="Version" />
</div>

## 📜 Description

SQLite Cleaner is a program designed to clean your SQLite database efficiently. It reduces database storage and optimizes all tables (except system tables). Built with Rust, it is compatible with all platforms, ensuring smooth operation across servers and applications. Download the program from the releases section and keep your database in top shape!

## ✨ Features

- 🗃️ Reduce database storage.
- 🚀 Optimize all tables except system tables.
- 💡 Simple and efficient way to clean SQLite databases.
- 🌍 Cross-platform compatibility.
- ⚙️ Maintain databases in optimal condition.
- ❌ No need for dumps or backups.
- 🔒 Does not modify your configuration files.
- 🖥️ Easily run on any server or application.
- 🛠️ User-friendly and easy to use.

## 💻 Platforms & Requirements

<div align="center">
<img src="https://img.shields.io/badge/OS-MacOS-informational?style=flat&logo=apple&logoColor=white&color=53a863" alt="MacOS" />
<img src="https://img.shields.io/badge/OS-Linux-informational?style=flat&logo=linux&logoColor=white&color=53a863" alt="Linux" />
<img src="https://img.shields.io/badge/OS-Windows-informational?style=flat&logo=windows&logoColor=white&color=53a863" alt="Windows" />
</div>

<div align="center">
<img src="https://img.shields.io/badge/Rust-1.83+-informational?style=flat&logo=rust&logoColor=white&color=53a863" alt="Rust" />
<img src="https://img.shields.io/badge/Cargo-informational?style=flat&logo=rust&logoColor=white&color=53a863" alt="Cargo" />
</div>

## 🖼️ Example of Execution

<div align="center">
<img src="assets/Example.png" alt="Example" width="500px" height="auto" />
</div>

## 📥 Installation

To run the program:

1. Clone the repository:

   ```bash
   git clone https://github.com/Maxime-Cllt/SqliteCleaner.git
   ```

2. Build the program:

   ```bash
   cargo build --release
   ```

3. Execute the program:

   You may need to give the program execute permissions on Linux and macOS:

   ```bash
   chmod +x target/release/SqliteCleaner
   ```

### MacOS & Linux

```bash
./target/release/SqliteCleaner "path/to/your_database.db"
```

### Windows

```bash
.\target\release\SqliteCleaner.exe "path/to/your_database.db"
```

## 📝 Notes

- ⏱️ Time complexity: O(n), where n is the number of tables in the database.
- ⚠️ Does not clean triggers, stored procedures, functions, and views.
- 📉 May not reduce storage significantly but is quick to run and can be executed frequently.

## 🔗 See Also

- 🌐 [DBMSCleaner](https://github.com/Maxime-Cllt/DBMSCleaner)
