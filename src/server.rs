use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

use types::*;
use database::*;
use parser::*;


#[derive(Debug)]
pub struct Server {
    database: Arc<Mutex<Database>>,
    socket: TcpListener,
}

impl Server {

    pub fn new(db: Database) -> Self {
        let database = Arc::new(Mutex::new(db));
        let address = "127.0.0.1:7777".parse::<SocketAddr>().unwrap();
        let server_socket = TcpListener::bind(&address).unwrap();
        println!("Listening on localhost:7777");

        Server {
            database: database,
            socket: server_socket
        }
    }

    pub fn listen(&self) {
        // accept connections and process them, spawning a new thread for each one
        for stream in self.socket.incoming() {
            match stream {
                Ok(stream) => self.handle_client(stream),
                Err(_) => { /* connection failed */ },
            }
        }
    }

    pub fn close_server(&mut self) {
        drop(&self.socket);
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let database = self.database.clone();
        thread::spawn(move|| {
            loop {
                // todo implement the need for more buffer?
                let mut buf = [0; 4 * 1024];
                match stream.read(&mut buf) {
                    // Can't read socket
                    Err(e) => {
                        println!("Error while reading socket: {:?}", e);
                        return;
                    },
                    // EOF
                    Ok(0) => break,
                    // Receiving data
                    Ok(_) => {
                        // parse buffer
                        let command = Parser::parse(&buf);
                        let response = Self::run_command(&database, command);
                        // write response
                        match stream.write(response.as_bytes()) {
                            Err(_) => break,
                            Ok(_) => continue,
                        }
                    }
                }
            }
        });
    }

    fn run_command(database: &Arc<Mutex<Database>>, command: Option<Command>) -> String {
        // we lock the database in an atomic way
        let mut data = database.lock().unwrap();
        match command {
            Some(Command::Get(k)) => {
                let value = data.get_value(k);
                return
                    String::from("$") +
                    &value.len().to_string() + CRLF +
                    &value + CRLF;
            },
            Some(Command::Set(k, v)) => {
                data.update_value(k, v);
                return OK.to_string();
            },
            Some(Command::Unknown)
                => String::from("-ERR unknown command") + CRLF,
            None
                => String::from("-ERR nothing to do") + CRLF,
        }
    }

}
