use std::{rc::Rc, cell::RefCell, collections::HashMap};

use crate::{engine::BuckDB, types::BuckTypes, errors::{TransactionError, BuckEngineError}, log::BuckLog};

#[derive(Debug, Eq, PartialEq)]
enum TransactionStatus {
    Committed,
    Uncommitted,
}

/// Transaction Management
#[derive(Debug)]
pub struct BuckTransaction {
    db: Rc<RefCell<BuckDB>>,
    uncommitted_data: HashMap<String, BuckTypes>,
    status: TransactionStatus,
}

impl BuckTransaction {
    pub fn new() -> Self {
        BuckTransaction {
            db: Rc::new(RefCell::new(BuckDB::new())),
            uncommitted_data: HashMap::new(),
            status: TransactionStatus::Uncommitted,
        }
    }

    fn insert(&mut self, key: String, value: BuckTypes) -> Result<BuckLog, BuckEngineError> {
        self.uncommitted_data.insert(key.to_owned(), value);

        Ok(BuckLog::InsertOk(key.to_owned()))
    }

    fn get(&self, key: &str) -> Option<&BuckTypes> {
        self.uncommitted_data.get(key)
    }

    fn remove(&mut self, key: &str) -> Result<BuckLog, BuckEngineError> {
        self.uncommitted_data.remove(key);

        Ok(BuckLog::RemoveOk(key.to_string()))
    }

    fn commit(&mut self) -> Result<BuckLog, TransactionError> {
        if self.status == TransactionStatus::Committed {
            return Err(TransactionError::AlreadyCommitted);
        }

        let mut db = self.db.borrow_mut();

        for (key, value) in self.uncommitted_data.drain() {
            db.insert(key, value).unwrap();
        }

        // update transaction status
        self.status = TransactionStatus::Committed;

        format!("[log] Transaction committed");

        Ok(BuckLog::TransactionOk)
    }

    fn rollback(&mut self) -> Result<BuckLog, TransactionError> {
        if self.uncommitted_data.is_empty() {
            return Err(TransactionError::NoBackup);
        }
        // clear uncommitted data
        self.uncommitted_data.clear();

        // update transaction status
        self.status = TransactionStatus::Committed;

        Ok(BuckLog::RollbackOk)
    }
}