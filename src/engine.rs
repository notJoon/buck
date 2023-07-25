use std::collections::BTreeMap;

use crate::types::BuckTypes;

#[derive(Debug)]
pub struct BuckDB {
    data: BTreeMap<String, BuckTypes>,
}

impl BuckDB {
    pub fn new() -> Self {
        BuckDB { 
            data: BTreeMap::new()
        }
    }

    /// Insert a key-value pair into the database.
    pub fn insert(&mut self, key: String, value: BuckTypes) -> String {
        self.data.insert(key.to_owned(), value);

        format!("{key} inserted into database")
    }

    /// Get a value from the database.
    pub fn get(&self, key: &str) -> Option<&BuckTypes> {
        self.data.get(key)
    }

    /// Remove a value from the database.
    pub fn remove(&mut self, key: &str) -> String {
        self.data.remove(key);

        format!("{key} removed from database")
    }

    /// Update a value in the database.
    pub fn update(&mut self, key: &str, value: BuckTypes) -> String {
        self.data.insert(key.to_string(), value);

        let new_value = self.data.get(key).unwrap();

        format!("Updated value: {:?}", new_value)
    }
}