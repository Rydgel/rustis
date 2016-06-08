use std::collections::HashMap;

pub type Key = &'static str;
pub type Value = &'static str;
pub type DB = HashMap<Key, Value>;

#[derive(PartialEq, Debug)]
pub enum Command {
    Get(Key),
    Set(Key, Value),
    Unknown,
}

#[derive(PartialEq, Debug)]
pub enum Reply {
    Bulk(Option<&'static str>),
    MultiBulk(Option<Vec<Reply>>),
}

pub const VERSION: &'static str = "0.1.0";
pub const CRLF: &'static str = "\r\n";
pub const OK: &'static str = "+OK\r\n";
