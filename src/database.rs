use std::collections::HashMap;

use types::*;

#[derive(Debug)]
pub struct Database {
    db: DB,
}

impl Database {

    pub fn new() -> Self {
        let mut rustis_db = HashMap::new();
        rustis_db.insert("__VERSION__", VERSION);
        Database { db: rustis_db }
    }

    pub fn update_value(&mut self, key: Key, value: Value) {
        let entry = self.db.entry(key).or_insert(value);
        *entry = value;
    }

    pub fn get_value(&self, key: Key) -> Value {
        self.db.get(key).unwrap_or(&"null")
    }

}
