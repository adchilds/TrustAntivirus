use core::config::Config;
use std::path::PathBuf;
use rusqlite::Connection;
use time::{self, Timespec};

///
///
///
pub struct Database;
impl Database {

    ///
    ///
    ///
    pub fn new() -> Database {
        let conn: Connection = Database::open(Config::get_config_home());

    }

    ///
    ///
    ///
    pub fn open(path: PathBuf) -> Connection {
        Connection::open(path).unwrap()
    }

}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
}

pub fn whatever() {
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
}