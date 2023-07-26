use std::collections::{BTreeMap, HashMap};

use crate::{errors::BuckEngineError, log::BuckLog, types::BuckTypes};

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
}

impl BuckDB {
    pub fn new() -> Self {
        BuckDB {
            data: BTreeMap::new(),
            uncommitted_data: HashMap::new(),
            transaction_backup: Some(BTreeMap::new()),
            status: TransactionStatus::Uncommitted,
        }
    }

    pub fn begin_transaction(&mut self) {
        // clear the uncommitted data to ensure that the transaction is clean
        self.uncommitted_data.clear();
        self.status = TransactionStatus::Uncommitted;
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

    /// Insert a key-value pair into the database.
    ///
    /// Newly added data is added to `uncommitted_data`,
    /// regardless of the transaction status.
    pub fn insert(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        match self.status {
            TransactionStatus::Committed | TransactionStatus::Uncommitted => {
                self.status = TransactionStatus::Uncommitted;
                self.uncommitted_data.insert(key.clone(), value);
            }
            TransactionStatus::Abort => return self.abort(),
        }

        Ok(BuckLog::InsertOk(key.to_string()))
    }

    /// Get a value from the database.
    pub fn get(&self, key: &str) -> Result<&BuckTypes, BuckEngineError> {
        match self.status {
            // if the transaction is uncommitted, check the uncommitted data first
            TransactionStatus::Uncommitted => match self.uncommitted_data.get(key) {
                Some(value) => Ok(value),
                None => match self.data.get(key) {
                    Some(value) => Ok(value),
                    None => Err(BuckEngineError::KeyNotFound(key.to_string())),
                },
            },
            TransactionStatus::Committed => match self.data.get(key) {
                Some(value) => Ok(value),
                None => Err(BuckEngineError::KeyNotFound(key.to_string())),
            },
            _ => Err(BuckEngineError::AbortError),
        }
    }

    /// Remove a value from the database.
    pub fn remove(&mut self, key: &str) -> Result<BuckLog, BuckEngineError> {
        match self.status {
            TransactionStatus::Committed => match self.data.remove(key) {
                Some(_) => Ok(BuckLog::RemoveOk(key.to_string())),
                None => Err(BuckEngineError::KeyNotFound(key.to_string())),
            },
            TransactionStatus::Uncommitted => match self.uncommitted_data.remove(key) {
                Some(_) => Ok(BuckLog::RemoveOk(key.to_string())),
                None => Err(BuckEngineError::KeyNotFound(key.to_string())),
            },
            TransactionStatus::Abort => return self.abort(),
        }
    }

    /// Update a value in the database.
    pub fn update(&mut self, key: &str, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        match self.status {
            TransactionStatus::Committed => match self.data.get_mut(key) {
                Some(v) => {
                    *v = value;
                    Ok(BuckLog::UpdateOk(key.to_string()))
                }
                None => Err(BuckEngineError::KeyNotFound(key.to_string())),
            },
            TransactionStatus::Uncommitted => match self.uncommitted_data.get_mut(key) {
                Some(v) => {
                    *v = value;
                    Ok(BuckLog::UpdateOk(key.to_string()))
                }
                None => Err(BuckEngineError::KeyNotFound(key.to_string())),
            },
            TransactionStatus::Abort => return self.abort(),
        }
    }
}
