use std::collections::BTreeMap;

use crate::{types::BuckTypes, errors::BuckEngineError, log::BuckLog};

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
    pub fn insert(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, ()> {
        self.data.insert(key.to_owned(), value);

        Ok(BuckLog::InsertOk(key.to_owned()))
    }

    /// Get a value from the database.
    pub fn get(&self, key: &str) -> Result<&BuckTypes, BuckEngineError> {
        match self.data.get(key) {
            Some(value) => Ok(value),
            None => Err(BuckEngineError::KeyNotFound(key.to_string())),
        }
    }

    /// Remove a value from the database.
    pub fn remove(&mut self, key: &str) -> Result<BuckLog, BuckEngineError> {
        match self.data.get(key) {
            Some(_) => {
                self.data.remove(key);

                Ok(BuckLog::RemoveOk(key.to_string()))
            },
            None => Err(BuckEngineError::KeyNotFound(key.to_string()))
        }
    }

    /// Update a value in the database.
    pub fn update(&mut self, key: &str, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {

        match self.data.get(key) {
            Some(_) => {
                self.data.insert(key.to_string(), value);

                Ok(BuckLog::UpdateOk(key.to_string()))
            },
            None => Err(BuckEngineError::KeyNotFound(key.to_string()))
        }
    }
}