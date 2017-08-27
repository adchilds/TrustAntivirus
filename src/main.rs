extern crate clap;
extern crate rayon;
extern crate crypto;
extern crate rusqlite;
extern crate serde;
extern crate time;
extern crate walkdir;

#[macro_use] extern crate serde_derive;

pub mod core;
pub mod db;
pub mod io;

use clap::{Arg, App};
use core::config::Config;
use core::engine::Engine;
use std::process;

const NAME: &'static str = "TrustAntivirus";
const VERSION: &'static str = "1.0.0";
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

    // Setup/verify configuration
    let config: Config = Config::open();
    if let Err(err) = config.verify_integrity() {
        println!("Encountered integrity issue with configuration file. err[{}]", err);

        process::exit(1);
    }

    // Run the scan
    let engine: Engine = Engine::from(dir);
    engine.do_scan();
}