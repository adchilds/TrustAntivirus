use core::scanner::ScanResult;
use db::{Database, SQL_MD5_SELECT};
use io::SystemFile;
use rayon::prelude::*;
use rusqlite::Connection;
use std::fs::{File, Metadata};
use std::path::Path;
use std::process;
use std::result::Result;
use std::time::SystemTime;
use walkdir::{DirEntry, Error, IntoIter, WalkDir};

///
///
///
pub struct Engine<'a> {

    path: &'a Path

}

impl<'a> Engine<'a> {

    ///
    ///
    ///
    pub fn new() -> Engine<'a> {
        Engine {
            path: Path::new("") // Defaults to the root directory of the current user's main disk
        }
    }

    ///
    ///
    ///
    pub fn from(path: &str) -> Engine {
        Engine {
            path: Path::new(path)
        }
    }

    ///
    ///
    ///
    pub fn do_scan(&self) {
        let cur_time: SystemTime = SystemTime::now();

        if !self.path.exists() {
            println!("Directory or file does not exist.");

            process::exit(1);
        }

        let db: Database = Database::default();

        let result: ScanResult = match self.path.is_dir() {
            true => Engine::scan_dir(self.path).unwrap(),
            false => Engine::scan_file(self.path, &db.conn).unwrap()
        };

        println!("Total size: {}", SystemFile::human_readable_size(result.total_scan_size));
        println!("Finished in {} seconds", SystemTime::now().duration_since(cur_time).unwrap().as_secs());
    }

    ///
    ///
    ///
    pub fn scan_dir(dir: &Path) -> Option<ScanResult> {
        println!("Scanning dir: {}", dir.to_str().unwrap());

        let dir_iter: IntoIter = WalkDir::new(dir).into_iter();
        let dir_entries: Vec<Result<DirEntry, Error>> = dir_iter.collect();
        let dir_iter_par = dir_entries.into_par_iter();

        // Scan in parallel using Rayon
        let total_size: f64 = dir_iter_par.map(|result| {
            let db: Database = Database::default();
            let conn: Connection = db.conn;
            let dir_entry: DirEntry = result.unwrap();
            let path: &Path = dir_entry.path();

            // Don't scan directories
            if path.is_dir() {
                return 0.0;
            }

            let mut file_size: f64 = 0.0;
            match File::open(path) {
                Err(_) => println!("Unable to open file: {}", path.to_str().unwrap()),
                Ok(file) => {
                    let metadata: Metadata = file.metadata().unwrap();
                    let file_path: String = String::from(path.to_str().unwrap());
                    let sys_file: SystemFile = SystemFile::from(file_path);

                    let mut stmt = conn.prepare(SQL_MD5_SELECT).unwrap();
                    if stmt.exists(&[&sys_file.md5]).unwrap() {
                        println!("Found malware: {}", sys_file.md5);
                    }

                    println!("{}", sys_file);

                    file_size = metadata.len() as f64;
                }
            };

            file_size
        }).sum();

        Some(ScanResult {
            total_scan_size: total_size
        })
    }

    ///
    ///
    ///
    pub fn scan_file(file_path: &Path, conn: &Connection) -> Option<ScanResult> {
        println!("Scanning file: {}", file_path.to_str().unwrap());

        let file: File = File::open(file_path).unwrap();
        let metadata: Metadata = file.metadata().unwrap();
        let file_path_as_str: String = String::from(file_path.to_str().unwrap());
        let sys_file: SystemFile = SystemFile::from(file_path_as_str);

        println!("{}", sys_file);

        let md5 = String::from(sys_file.md5.as_str());

        let result = conn.execute(SQL_MD5_SELECT, &[&md5]).unwrap();
        println!("Result: {}", result);

        // Determine which Scanner to use


        Some(ScanResult {
            total_scan_size: metadata.len() as f64
        })
    }

}

#[allow(dead_code)]
pub struct FileType {

    extension: String,
    mime_type: String

}

pub trait FileTypeScan {

    ///
    ///
    ///
    fn get_file_type() -> FileType;

}