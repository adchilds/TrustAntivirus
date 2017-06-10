extern crate clap;
extern crate md5;
extern crate walkdir;

use clap::{Arg, App};
use md5::{Digest};
use std::cmp;
use std::fmt::{Display, Formatter, Result};
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let matches = App::new("T_Rust")
        .version("1.0")
        .author("Adam Childs <adam.childs@gmail.com>")
        .about("")
        .arg(Arg::with_name("DIRECTORY")
            .help("Sets the input directory to use")
            .required(true)
            .index(1))
        .get_matches();

    let dir: &str = matches.value_of("DIRECTORY").unwrap_or("/Users/adam.childs/Desktop");
    println!("Dir: {}", dir);

    let mut total_size: f64 = 0.0;
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let file: File = File::open(entry.path()).unwrap();
        let metadata: Metadata = file.metadata().unwrap();
        let file_path: String = String::from(entry.path().to_str().unwrap());
        let sys_file: SystemFile = SystemFile::from(file_path);

        total_size += metadata.len() as f64;

        println!("{}", sys_file);
    }

    println!("Total size: {}", SystemFile::human_readable_size(total_size));
}

///
/// Simple representation of a File on the user's system.
///
struct SystemFile {
    name: String,
    path: String,
    size: String,
    md5: Digest,
    is_dir: bool
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

        SystemFile {
            name: String::from(Path::new(&path).file_name().unwrap().to_str().unwrap()),
            path: path,
            size: SystemFile::human_readable_size(metadata.len() as f64),
            md5: md5::compute(buffer),
            is_dir: metadata.is_dir()
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
        write!(f, "({}, {}, {}, {})", format!("{:x}", self.md5), self.name, self.size, self.path)
    }
}