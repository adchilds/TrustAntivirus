use core::config::Config;
use std::path::{MAIN_SEPARATOR, PathBuf};
use rusqlite::Connection;
use time::Timespec;

const DB_NAME: &'static str = "db.tav";

pub const SQL_MD5_SELECT: &'static str = "SELECT name FROM malware WHERE md5 = ?";

pub struct Database {

    pub location: PathBuf,
    pub conn: Connection

}

impl Database {

    ///
    /// Returns an instance of the inner database, with an open connection.
    ///
    pub fn default() -> Database {
        Database {
            location: get_default_location(),
            conn: Database::open(get_default_location())
        }
    }

    ///
    /// Opens a connection to the local database stored on disk.
    ///
    pub fn open(path: PathBuf) -> Connection {
        Connection::open(path).unwrap()
    }

    ///
    /// Provisions the database with tables and metadata.
    ///
    pub fn init(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS malware (
                    id              INTEGER PRIMARY KEY,
                    create_date     TEXT NOT NULL,
                    md5             TEXT NOT NULL,
                    sha256          TEXT,
                    name            TEXT NOT NULL
                  )", &[]).expect("Unable to create table 'malware'.");
    }

}

///
/// Represents a single instance of malware, storing metadata used in identifying the malware.
///
pub struct Malware {

    id: i64,
    create_date: Timespec,
    md5: String,
    sha256: String,
    name: String

}

///
/// Returns the default location of the database stored on disk. This is dependent on the host
/// operating system of the current user's machine.
///
fn get_default_location() -> PathBuf {
    let mut default_location: String = Config::get_config_home();
    default_location.push(MAIN_SEPARATOR);
    default_location.push_str(DB_NAME);

    PathBuf::from(default_location)
}