#[macro_use]

extern crate nom;

mod types;
mod database;
use database::*;
mod server;
use server::*;
mod parser;


fn main() {
    let mut server = Server::new(Database::new());
    server.listen();
    server.close_server();
}
