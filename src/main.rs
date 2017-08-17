extern crate clap;
extern crate md5;
extern crate rayon;
extern crate rusqlite;
extern crate serde;
extern crate time;
extern crate walkdir;

#[macro_use] extern crate serde_derive;

pub mod core;
pub mod io;

use clap::{Arg, App};
use core::config::Config;
use core::engine::Engine;
use std::process;

use rusqlite::Connection;
use time::Timespec;

const NAME: &'static str = "TrustAntivirus";
const VERSION: &'static str = "1.0.0";
const AUTHOR: &'static str = "Adam Childs <adamdchilds@gmail.com>";
const ABOUT: &'static str = "";

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
}

fn main() {
    let conn: Connection = Connection::open("C:\\Users\\adam8\\Desktop\\data.db").unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();

    let me: Person = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None
    };
    conn.execute("INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
                 &[&me.name, &me.time_created, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            data: row.get(3)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
/*
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
*/
}