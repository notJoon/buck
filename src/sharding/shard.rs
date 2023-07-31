use std::collections::BTreeMap;

use crate::types::sets::Setable;
use crate::types::types::BuckTypes;
use crate::{errors::BuckEngineError, log::BuckLog};

#[derive(Debug, Clone, Default)]
pub struct BuckDBShard {
    data: BTreeMap<String, BuckTypes>,
}

impl BuckDBShard {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        print!("shard insert");
        self.data.insert(key.to_owned(), value);
        Ok(BuckLog::InsertOk(key.to_owned()))
    }

    pub fn get(&self, key: &str) -> Result<BuckTypes, BuckEngineError> {
        match self.data.get(key) {
            Some(v) => Ok(v.to_owned()),
            None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<BuckLog, BuckEngineError> {
        match self.data.remove(key) {
            Some(_) => Ok(BuckLog::RemoveOk(key.to_owned())),
            None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    pub fn insert_set_value_to_shard(
        &mut self,
        key: &str,
        value: &Setable,
    ) -> Result<BuckLog, BuckEngineError> {
        match self.data.get_mut(key) {
            Some(v) => {
                if let BuckTypes::Sets(set) = v {
                    set.insert(&[value.to_owned()]);
                    return Ok(BuckLog::InsertOk(key.to_owned()));
                }

                Err(BuckEngineError::KeyNotFound(key.to_owned()))
            }
            None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    pub fn remove_set_key_value_from_shard(
        &mut self,
        key: &str,
        value: &Setable,
    ) -> Result<BuckLog, BuckEngineError> {
        match self.data.get_mut(key) {
            Some(v) => {
                if let BuckTypes::Sets(set) = v {
                    set.remove(&[value.to_owned()]);
                    return Ok(BuckLog::RemoveOk(key.to_owned()));
                }

                Err(BuckEngineError::KeyNotFound(key.to_owned()))
            }
            None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    pub fn update(&mut self, key: &str, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        match self.data.get_mut(key) {
            Some(v) => {
                *v = value;
                Ok(BuckLog::UpdateOk(key.to_owned()))
            }
            None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }
}
