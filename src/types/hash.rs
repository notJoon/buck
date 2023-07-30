use std::collections::HashMap;

use crate::parser::errors::BuckParserError;

use super::types::BuckTypes;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuckHash {
    pub data: HashMap<String, BuckTypes>,
}

impl BuckHash {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: String, value: BuckTypes) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, key: &[&str]) {
        for item in key {
            self.data.remove(*item);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn increment_value(&mut self, key: &str) -> Result<(), BuckParserError> {
        match self.data.get(key) {
            Some(value) => {
                if let BuckTypes::Integer(value) = value {
                    self.data.insert(key.to_owned(), BuckTypes::Integer(value + 1));
                    return Ok(());
                }

                return Err(BuckParserError::HashValueIsNotInteger(key.to_owned()));
            }
            None => {
                self.data.insert(key.to_owned(), BuckTypes::Integer(1));
                return Ok(());
            }
        }
    }
}