use crypto::digest::Digest;
use crypto::md5::Md5;
use std::cmp;
use std::fmt::{Display, Formatter, Result};
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::Path;

///
/// Simple representation of a File on the user's system.
///
pub struct SystemFile {
    pub name: String,
    pub path: String,
    pub size: String,
    pub md5: String
}

impl SystemFile {

    ///
    /// Constructs a new `SystemFile` from the given arguments.
    ///
    pub fn from(path: String) -> SystemFile {
        let mut file: File = File::open(&path).unwrap();
        let metadata: Metadata = file.metadata().unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        if !metadata.is_dir() {
            // Read the entire file to a buffer
            file.read_to_end(&mut buffer).unwrap();
        }

        let mut md5 = Md5::new();
        md5.input(buffer.as_ref());

        SystemFile {
            name: String::from(Path::new(&path).file_name().unwrap().to_str().unwrap()),
            path: path,
            size: SystemFile::human_readable_size(metadata.len() as f64),
            md5: String::from(md5.result_str())
        }
    }

    ///
    /// Returns the human readable representation of the current files size in bytes as a `String`.
    ///
    pub fn human_readable_size(num: f64) -> String {
        let negative = if num.is_sign_positive() { "" } else { "-" };
        let num = num.abs();
        let units = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        if num < 1_f64 {
            return format!("{}{} {}", negative, num, "B");
        }
        let delimiter = 1000_f64;
        let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
        let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
        let unit = units[exponent as usize];
        format!("{}{} {}", negative, pretty_bytes, unit)
    }

}

impl Display for SystemFile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {}, {})", self.md5, self.name, self.size, self.path)
    }
}