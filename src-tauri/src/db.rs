use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use std::path::PathBuf;

pub fn establish_connection() -> SqliteConnection {
    let database_path = get_database_path();
    let database_url = format!("file:{}", database_path.to_str().unwrap());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn get_database_path() -> PathBuf {
    // Check if we're running from the installed location
    if let Ok(exe_path) = std::env::current_exe() {
        if exe_path.starts_with("/usr/bin") {
            // We're installed, use the path in /usr/lib
            return PathBuf::from("/usr/lib/course-progress/db/db.sqlite");
        }
    }
    
    // If we're not installed (e.g., running in development), use a relative path
    PathBuf::from("db/db.sqlite")
}