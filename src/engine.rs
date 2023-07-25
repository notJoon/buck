use std::collections::BTreeMap;

use crate::{types::BuckTypes, errors::BuckEngineError};

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
    pub fn get(&self, key: &str) -> Result<&BuckTypes, BuckEngineError> {
        match self.data.get(key) {
            Some(value) => Ok(value),
            None => Err(BuckEngineError::KeyNotFound(key.to_string())),
        }
    }

    /// Remove a value from the database.
    pub fn remove(&mut self, key: &str) -> Result<String, BuckEngineError> {
        match self.data.get(key) {
            Some(_) => {
                self.data.remove(key);

                Ok(format!("[Success] Removed value: {:?}", key.to_string()))
            },
            None => Err(BuckEngineError::KeyNotFound(key.to_string()))
        }
    }

    /// Update a value in the database.
    pub fn update(&mut self, key: &str, value: BuckTypes) -> Result<String, BuckEngineError> {

        match self.data.get(key) {
            Some(_) => {
                self.data.insert(key.to_string(), value);

                Ok(format!("[Success] Updated value: {:?}", self.data.get(key).unwrap()))
            },
            None => Err(BuckEngineError::KeyNotFound(key.to_string()))
        }
    }
}