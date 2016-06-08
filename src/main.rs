#![feature(slice_patterns)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};


type Key = &'static str;
type Value = &'static str;
type DB = HashMap<Key, Value>;

#[derive(PartialEq, Debug)]
enum Command {
    Get(Key),
    Set(Key, Value),
    Unknown,
}

#[derive(PartialEq, Debug)]
enum Reply {
    Bulk(Option<&'static str>),
    MultiBulk(Option<Vec<Reply>>),
}

const VERSION: &'static str = "0.1.0";
const CRLF: &'static str = "\r\n";
const OK: &'static str = "+OK\r\n";

fn parse_reply(reply: Reply) -> Option<Command> {
    match reply {
        Reply::MultiBulk(Some(xs)) => {
            match &xs[..] {
                [Reply::Bulk(Some("get")), Reply::Bulk(Some(a))]
                    => Some(Command::Get(a)),
                [Reply::Bulk(Some("set")), Reply::Bulk(Some(a)), Reply::Bulk(Some(b))]
                    => Some(Command::Set(a, b)),
                _
                    => Some(Command::Unknown),
            }
        },
        _ => None,
    }
}

#[derive(Debug)]
pub struct Database {
    db: DB,
}

impl Database {
    pub fn new() -> Database {
        let mut rustis_db = HashMap::new();
        rustis_db.insert("__VERSION__", VERSION);
        Database { db: rustis_db }
    }

    fn update_value(&mut self, key: Key, value: Value) {
        let entry = self.db.entry(key).or_insert(value);
        *entry = value;
    }

    fn get_value(&self, key: Key) -> Value {
        self.db.get(key).unwrap_or(&"null")
    }
}

fn listen(server_socket: &TcpListener, database: Database) {
    let data = Arc::new(Mutex::new(database));
    // accept connections and process them, spawning a new thread for each one
    for stream in server_socket.incoming() {
        match stream {
            Ok(stream) => {
                let data = data.clone();
                thread::spawn(move|| {
                    // fixme only lock when necessary
                    let mut data = data.lock().unwrap();
                    // connection succeeded
                    handle_client(stream, &mut data);

                    /*thread::spawn(move|| {
                        let mut data = data.lock().unwrap();
                    });*/
                });
            }
            Err(_) => {
                /* connection failed */
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, database: &mut MutexGuard<Database>) {
    let mut buffer = String::new();
    let result = stream.read_to_string(&mut buffer);
    println!("{:?}", result);
    println!("{:?}", buffer);
    database.update_value("tamer", "la grosse pute 3");
}

fn main() {
    let mut database = Database::new();
    database.update_value("tamer", "la grosse pute");
    database.update_value("tamer", "la grosse pute 2");
    let address = "127.0.0.1:7777".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();
    println!("Listening on localhost:7777");
    listen(&server_socket, database);


    drop(server_socket);
}
