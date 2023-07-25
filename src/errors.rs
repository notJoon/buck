use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum BuckEngineError {
    KeyNotFound(String),
    InvalidType,
}

impl fmt::Display for BuckEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckEngineError::KeyNotFound(key) => write!(f, "[Error] Key not found: {}", key),
            BuckEngineError::InvalidType => write!(f, "Invalid type"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionError {
    AlreadyCommitted,
    NoBackup,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::AlreadyCommitted => write!(f, "[Error] Transaction has already been committed"),
            TransactionError::NoBackup => write!(f, "[Error] Can't rollback transaction, no backup found"),
        }
    }
}

impl std::error::Error for TransactionError {}