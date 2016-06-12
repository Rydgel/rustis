use std::collections::HashMap;

use types::*;

#[derive(Debug)]
pub struct Database {
    db: DB,
}

impl Database {

    pub fn new() -> Self {
        let mut rustis_db = HashMap::new();
        rustis_db.insert("__VERSION__".to_string(), VERSION.to_string());
        Database { db: rustis_db }
    }

    pub fn update_value(&mut self, key: Key, value: Value) {
        let value2 = value.clone();
        let entry = self.db.entry(key).or_insert(value);
        *entry = value2;
    }

    pub fn get_value(&self, key: Key) -> Value {
        match self.db.get(&key) {
            Some(value) => value.clone(),
            None => "null".to_string()
        }
    }

}
