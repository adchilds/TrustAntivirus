use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

///
/// Represents local configuration for Trust Antivirus. Configuration is stored in JSON format in a
/// configuration file that's stored in the current user's home directory. This directory is hidden
/// by default but can be accessed by showing hidden files on the Operating System of choice.
///
#[derive(Serialize, Deserialize)]
pub struct Config {

    parallel: bool,
    hash_algorithm: HashAlgorithm,
    db_path: String

}

impl Config {

    ///
    /// Generates new default `Config` settings. These settings will always be defaulted the first
    /// time the application is started. To change settings, users must manually modify the config
    /// file located in the application's configuration directory.
    ///
    pub fn new() -> Config {
        Config {
            parallel: true,
            hash_algorithm: HashAlgorithm::MD5,
            db_path: String::from("")
        }
    }

    ///
    /// Open's the main configuration file. The location of this file depends on the underlying
    /// Operating System. By default, this will be located in the current user's home directory. If
    /// the configuration file cannot be found, a new one is created with default settings.
    ///
    pub fn open() -> Config {
        let user_home: PathBuf = get_user_home();

        // Create the TrustAntivirus home/config directory if it doesn't exist
        if !user_home.exists() {
            if let Err(err) = fs::create_dir_all(user_home) {
                println!("Unexpected error while creating TrustAntivirus configuration directories. err=[{}]", err);

                process::exit(1);
            }
        }

        Config::new()
    }

}

///
/// Represents the hashing algorithm used against files. Each algorithm has pros and cons in regards
/// to reliability and speed. The default selected algorithm is MD5 though it must be noted that
/// this hashing algorithm has known upper-limits and cannot be deemed entirely reliable. To change
/// the default algorithm, manually update the configuration file located in the application's
/// configuration directory.
///
#[derive(Serialize, Deserialize)]
enum HashAlgorithm {

    MD5,
    SHA256

}

///
/// Retrieves the current user's home directory.
///
fn get_user_home() -> PathBuf {
    // TODO: Add fallback to detect OS type and then select a known directory for that OS
    env::home_dir().expect("Unable to determine current user's home directory.")
}