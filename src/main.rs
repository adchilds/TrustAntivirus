extern crate clap;
extern crate md5;
extern crate walkdir;

pub mod io;

use clap::{Arg, App};
use io::SystemFile;
use std::fs::{File, Metadata};
use std::path::Path;
use std::process;
use walkdir::WalkDir;

const NAME: &'static str = "TrustAntivirus";
const VERSION: &'static str = "1.0";
const AUTHOR: &'static str = "Adam Childs <adamdchilds@gmail.com>";
const ABOUT: &'static str = "";

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(Arg::with_name("DIRECTORY")
            .help("Sets the input directory to use")
            .required(true)
            .index(1))
        .get_matches();

    let dir: &str = matches.value_of("DIRECTORY").unwrap_or("/Users/adchilds/Desktop");
    println!("Dir: {}", dir);

    let path: &Path = Path::new(dir);
    if !path.exists() {
        println!("Directory or file does not exist.");

        process::exit(1);
    }

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