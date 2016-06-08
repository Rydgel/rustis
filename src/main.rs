#![feature(slice_patterns)]

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
