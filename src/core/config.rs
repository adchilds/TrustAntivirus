use db::Database;
use std::env;
use std::fs;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};
use std::process;
use std::result::Result;

const PROGRAM_HOME: &'static str = ".trustantivirus";

///
/// Represents local configuration for Trust Antivirus. Configuration is stored in JSON format in a
/// configuration file that's stored in the current user's home directory. This directory is hidden
/// by default but can be accessed by showing hidden files on the Operating System of choice.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {

    pub parallel: bool,
    pub hash_algorithm: HashAlgorithm,
    pub program_version: String

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
            program_version: String::from("1.0.0")
        }
    }

    ///
    /// Open's the main configuration file. The location of this file depends on the underlying
    /// Operating System. By default, this will be located in the current user's home directory. If
    /// the configuration file cannot be found, a new one is created with default settings.
    ///
    pub fn open() -> Config {
        let config_home: String = Config::get_config_home();
        let config_home_path: &Path = Path::new(config_home.as_str());

        // Create the TrustAntivirus home/config directory if it doesn't exist
        if !config_home_path.exists() {
            if let Err(err) = fs::create_dir_all(config_home_path) {
                println!("Unexpected error while creating TrustAntivirus configuration directories. err=[{}]", err);

                process::exit(1);
            }

            println!("Created TrustAntivirus home directory [{}]", config_home_path.to_str().unwrap());
        }

        // Initialize the database
        Database::default().init();

        // TODO: Read or create config
        Config::new()
    }

    ///
    /// Determines and returns the configuration home directory for TrustAntivirus. This directory
    /// is dependent on two factors:
    /// * The current user's home directory
    /// * The Operating System of the host machine
    ///
    pub fn get_config_home() -> String {
        let user_home: PathBuf = get_user_home();
        let mut program_home: String = String::from(user_home.to_str().unwrap());
        program_home.push(MAIN_SEPARATOR);
        program_home.push_str(PROGRAM_HOME);

        program_home
    }

    ///
    /// Verifies the integrity of TrustAntivirus' configuration file. If there are any issues with
    /// verifying the integrity, TrustAntivirus this function will return an Err along with a
    /// message that explains the encountered issue.
    ///
    pub fn verify_integrity(&self) -> Result<&'static str, &'static str> {
        let config_home: String = Config::get_config_home();
        let config_home_path: &Path = Path::new(config_home.as_str());

        // Make sure the configuration directory exists
        if !config_home_path.exists() {
            return Err::<&'static str, &'static str>("TrustAntivirus configuration location does not exist.");
        }

        // TODO: Implement integrity checking of config file (file exists, formatted correctly, etc.)
        Ok("Success.")
    }

}

///
/// Represents the hashing algorithm used against files. Each algorithm has pros and cons in regards
/// to reliability and speed. The default selected algorithm is MD5 though it must be noted that
/// this hashing algorithm has known upper-limits and cannot be deemed entirely reliable. To change
/// the default algorithm, manually update the configuration file located in the application's
/// configuration directory.
///
#[derive(Debug, Serialize, Deserialize)]
pub enum HashAlgorithm {

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