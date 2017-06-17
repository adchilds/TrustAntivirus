
///
/// Represents local configuration for Trust Antivirus. Configuration is stored in JSON format in a
///
///
#[derive(Serialize, Deserialize)]
pub struct Config {

    parallel: bool,
    hash_algorithm: HashAlgorithm

}

impl Config {

    ///
    ///
    ///
    pub fn new() -> Config {
        Config {
            parallel: true,
            hash_algorithm: HashAlgorithm::MD5
        }
    }

    ///
    ///
    ///
    pub fn open() -> Config {
        Config::new()
    }

}

#[derive(Serialize, Deserialize)]
enum HashAlgorithm {

    MD5,
    SHA256

}