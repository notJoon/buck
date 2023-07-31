use std::collections::{BTreeMap, HashMap};

use crate::parser::parse::get_value_type;
use crate::sharding::hash::calculate_hash;
use crate::sharding::shard::BuckDBShard;
use crate::types::list::BuckList;
use crate::types::sets::Setable;
use crate::types::types::BuckTypes;
use crate::{errors::BuckEngineError, log::BuckLog};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TransactionStatus {
    Committed,
    Uncommitted,
    Abort,
}

#[derive(Debug, Clone)]
pub struct BuckDB {
    pub data: BTreeMap<String, BuckTypes>,
    pub uncommitted_data: HashMap<String, BuckTypes>,
    pub transaction_backup: Option<BTreeMap<String, BuckTypes>>,
    pub status: TransactionStatus,
    pub shards: Vec<BuckDBShard>,
    pub is_shard_active: bool,
}

impl BuckDB {
    pub fn new() -> Self {
        BuckDB {
            data: BTreeMap::new(),
            uncommitted_data: HashMap::new(),
            transaction_backup: Some(BTreeMap::new()),
            status: TransactionStatus::Uncommitted,
            shards: Vec::new(),
            is_shard_active: false,
        }
    }

    ///////// Transaction /////////

    pub fn begin_transaction(&mut self) -> Result<BuckLog, ()> {
        // clear the uncommitted data to ensure that the transaction is clean
        self.uncommitted_data.clear();
        self.status = TransactionStatus::Uncommitted;

        Ok(BuckLog::ClearTransactionOk)
    }

    pub fn commit(&mut self) -> Result<BuckLog, BuckEngineError> {
        if self.status == TransactionStatus::Committed {
            return Err(BuckEngineError::AlreadyCommitted);
        }

        for (key, value) in self.uncommitted_data.drain() {
            self.data.insert(key, value);
        }

        // update transaction status
        self.status = TransactionStatus::Committed;
        self.uncommitted_data.clear();

        Ok(BuckLog::TransactionOk)
    }

    pub fn abort(&mut self) -> Result<BuckLog, BuckEngineError> {
        if self.transaction_backup.is_none() {
            return Err(BuckEngineError::NoBackup);
        }

        if self.status == TransactionStatus::Abort {
            // restore the data from the backup
            for (k, v) in self.transaction_backup.take().unwrap() {
                self.data.insert(k, v);
            }

            self.uncommitted_data.clear();
            self.status = TransactionStatus::Committed;

            return Ok(BuckLog::RollbackOk);
        }

        Err(BuckEngineError::AbortError)
    }

    ///////// Query /////////

    /// Insert a key-value pair into the database.
    ///
    /// Newly added data is added to `uncommitted_data`,
    /// regardless of the transaction status.
    pub fn insert(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        match self.status {
            TransactionStatus::Committed => {
                self.status = TransactionStatus::Uncommitted;

                if self.is_shard_active {
                    self.with_shard(&key, |shard| shard.insert(key.clone(), value.clone()))?;
                }

                self.uncommitted_data.insert(key.clone(), value);
            }
            TransactionStatus::Uncommitted => {
                if self.is_shard_active {
                    self.with_shard(&key, |shard| shard.insert(key.clone(), value.clone()))?;
                }

                self.uncommitted_data.insert(key.clone(), value);
            }
            TransactionStatus::Abort => return self.abort(),
        }

        Ok(BuckLog::InsertOk(key))
    }

    /// Get a value from the database.
    pub fn get(&self, key: &str) -> Result<&BuckTypes, BuckEngineError> {
        match self.status {
            // if the transaction is uncommitted, check the uncommitted data first
            TransactionStatus::Uncommitted => match self.uncommitted_data.get(key) {
                Some(value) => Ok(value),
                None => match self.data.get(key) {
                    Some(value) => Ok(value),
                    None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
                },
            },
            TransactionStatus::Committed => match self.data.get(key) {
                Some(value) => Ok(value),
                None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
            },
            _ => Err(BuckEngineError::AbortError),
        }
    }

    /// Remove a value from the database.
    pub fn remove(&mut self, key: &str) -> Result<BuckLog, BuckEngineError> {
        if self.is_shard_active {
            self.with_shard(key, |shard| shard.remove(key))?;
        }

        match self.status {
            TransactionStatus::Committed => match self.data.remove(key) {
                Some(_) => Ok(BuckLog::RemoveOk(key.to_owned())),
                None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
            },
            TransactionStatus::Uncommitted => match self.uncommitted_data.remove(key) {
                Some(_) => Ok(BuckLog::RemoveOk(key.to_owned())),
                None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
            },
            TransactionStatus::Abort => self.abort(),
        }
    }

    /// Update a value in the database.
    pub fn update(&mut self, key: &str, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        if self.is_shard_active {
            self.with_shard(key, |shard| shard.update(key, value.clone()))?;
        }

        match self.status {
            // TODO if apply update to uncommitted data, should change the status to uncommitted
            TransactionStatus::Committed => match self.data.get_mut(key) {
                Some(v) => {
                    // exchange the previous value with the new value
                    *v = value;
                    Ok(BuckLog::UpdateOk(key.to_owned()))
                }
                None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
            },
            TransactionStatus::Uncommitted => match self.uncommitted_data.get_mut(key) {
                Some(v) => {
                    *v = value;
                    Ok(BuckLog::UpdateOk(key.to_owned()))
                }
                None => Err(BuckEngineError::KeyNotFound(key.to_owned())),
            },
            TransactionStatus::Abort => self.abort(),
        }
    }

    ///////// Sharding /////////

    pub fn enable_sharding(&mut self, num_shards: usize) -> Result<BuckLog, ()> {
        self.is_shard_active = true;

        for _ in 0..num_shards {
            self.shards.push(BuckDBShard::new());
        }

        Ok(BuckLog::ShardingEnableOk)
    }

    // Only for testing (should be private later)
    pub fn get_shard_data(&self, idx: usize) -> Option<&BuckDBShard> {
        self.shards.get(idx)
    }

    /// query function handler for sharding
    fn with_shard<F, R>(&mut self, key: &str, mut query_function: F) -> Result<R, BuckEngineError>
    where
        F: FnMut(&mut BuckDBShard) -> Result<R, BuckEngineError>,
    {
        if self.is_shard_active {
            let shard_idx = calculate_hash(&key) as usize % self.shards.len();
            let shard = &mut self.shards[shard_idx];

            return query_function(shard);
        }

        Err(BuckEngineError::ShardingNotActive)
    }

    ///////// Type /////////

    /// Insert all the specified values at the head of the list stored at `key`.
    ///
    /// `LPUSH key element | start...end`
    ///
    /// if `key` does not exist, it is create as empty list before performing the push operations.
    pub fn l_push(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        if self.status == TransactionStatus::Committed {
            self.status = TransactionStatus::Uncommitted;
        }

        if self.is_shard_active {
            self.with_shard(&key, |shard| shard.insert(key.clone(), value.clone()))?;
        }

        // if key does not exist, create a new list
        if !self.uncommitted_data.contains_key(&key) {
            let mut list = BuckList::new();
            list.push(value);
            self.uncommitted_data
                .insert(key.clone(), BuckTypes::List(list));

            return Ok(BuckLog::InsertOk(key));
        }

        match self.uncommitted_data.get_mut(&key) {
            Some(BuckTypes::List(list)) => {
                list.push(value);
                Ok(BuckLog::InsertOk(key))
            }
            _ => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    /// Removes and returns the first element of the list stored at `key`.
    pub fn l_pop(&mut self, key: &str) -> Result<BuckLog, BuckEngineError> {
        if self.status == TransactionStatus::Committed {
            self.status = TransactionStatus::Uncommitted;
        }

        if self.is_shard_active {
            self.with_shard(key, |shard| shard.remove(key))?;
        }

        match self.uncommitted_data.get_mut(key) {
            Some(BuckTypes::List(list)) => {
                let value = list.pop().unwrap();
                Ok(BuckLog::ListPopOk(value.unwrap().to_string()))
            }
            _ => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    /// Add the specified members to the set stored at key.
    /// Specified members that are already a member of this set are ignored.
    /// If key does not exist, a new set is created before adding the specified members.
    ///
    /// An error is returned when the value stored at key is not a set.
    pub fn s_add(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        unimplemented!()
    }

    /// Remove the specified members from the set stored at key.
    /// Specified members that are not a member of this set are ignored.
    ///
    /// If key does not exist, it is treated as an empty set and this command returns 0.
    ///
    /// An error is returned when the value stored at key is not a set.
    ///
    /// ## Returns
    ///
    /// Integer reply: the number of members that were removed from the set, not including non existing members.
    pub fn s_rem(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        // check value type is `Setable` and wrap it into a `Setable` if it is.
        let value = match value {
            BuckTypes::String(string) => Setable::String(string),
            BuckTypes::Integer(integer) => Setable::Integer(integer),
            BuckTypes::Boolean(boolean) => Setable::Boolean(boolean),
            _ => return Err(BuckEngineError::TypeNotSupported(value.to_string())),
        };

        if self.status == TransactionStatus::Committed {
            self.status = TransactionStatus::Uncommitted;
        }

        if self.is_shard_active {
            self.with_shard(&key, |shard| shard.remove_set_value_from_key(&key, &value))?;
        }

        match self.uncommitted_data.get_mut(&key) {
            Some(BuckTypes::Sets(set)) => {
                set.remove(&[value]);
                Ok(BuckLog::RemoveOk(key))
            }
            _ => Err(BuckEngineError::KeyNotFound(key.to_owned())),
        }
    }

    /// Returns the members of the set resulting from the intersection of all the given sets.
    ///
    /// For example:
    ///    - key1: (a, b, c, d)
    ///    - key2: (c)
    ///    - key3: (a, c, e)
    ///    - key1.intersection(key2, key3) -> (c)
    ///
    /// Keys that do not exist are considered to be empty sets.
    /// With one of the keys being an empty set, the resulting set is also empty
    /// (since set intersection with an empty set always results in an empty set).
    ///
    /// ## Returns
    ///
    /// Array reply: list with members of the resulting set.
    pub fn s_inter(
        &mut self,
        key: String,
        others: Vec<String>,
    ) -> Result<BuckLog, BuckEngineError> {
        unimplemented!()
    }

    /// Returns if member is a member of the set stored at key.
    ///
    /// ## Returns
    ///
    /// `1` if the element is a member of the set.
    ///
    /// `0` if the element is not a member of the set, or if key does not exist.
    pub fn s_is_member(
        &mut self,
        key: String,
        value: BuckTypes,
    ) -> Result<BuckLog, BuckEngineError> {
        unimplemented!()
    }

    pub fn get_collections_length(&self, key: String) -> Result<usize, BuckEngineError> {
        match self.status {
            TransactionStatus::Uncommitted => {
                self.get_length_from_value(self.uncommitted_data.get(&key), key)
            }
            TransactionStatus::Committed => self.get_length_from_value(self.data.get(&key), key),
            _ => Err(BuckEngineError::AbortError),
        }
    }

    fn get_length_from_value(
        &self,
        value: Option<&BuckTypes>,
        key: String,
    ) -> Result<usize, BuckEngineError> {
        match value {
            Some(BuckTypes::List(list)) => Ok(list.len()),
            Some(BuckTypes::Hash(hash)) => Ok(hash.len()),
            Some(BuckTypes::Sets(set)) => Ok(set.len()),
            Some(BuckTypes::String(string)) => Ok(string.len()),
            _ => Err(BuckEngineError::LengthNotSupported(key.to_owned())),
        }
    }

    /// Get the type of a value in the database.
    pub fn type_of(&self, key: &str) -> Result<String, BuckEngineError> {
        let value = self.get(key)?;

        match value {
            BuckTypes::String(_) => Ok("string".to_owned()),
            BuckTypes::Integer(_) => Ok("integer".to_owned()),
            BuckTypes::Float(_) => Ok("float".to_owned()),
            BuckTypes::Boolean(_) => Ok("boolean".to_owned()),
            BuckTypes::List(_) => Ok("list".to_owned()),
            BuckTypes::Hash(_) => Ok("hash".to_owned()),
            BuckTypes::Sets(_) => Ok("sets".to_owned()),
            BuckTypes::Unknown(_) => Ok("unknown".to_owned()),
        }
    }
}
