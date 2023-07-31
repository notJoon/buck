use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum BuckEngineError {
    KeyNotFound(String),
    ValueNotFound(String),
    InvalidType,
    AlreadyCommitted,
    NoBackup,
    AbortError,
    ShardingNotActive,
    Unknown,
    LengthNotSupported(String),
    TypeNotSupported(String),
}

impl fmt::Display for BuckEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckEngineError::KeyNotFound(key) => write!(f, "[Error] Key not found: {}", key),
            BuckEngineError::ValueNotFound(key) => write!(f, "[Error] Value not found: {}", key),
            BuckEngineError::InvalidType => write!(f, "[Error] Invalid type"),
            BuckEngineError::AlreadyCommitted => write!(f, "[Error] Transaction already committed"),
            BuckEngineError::NoBackup => write!(f, "[Error] No backup"),
            BuckEngineError::AbortError => write!(f, "[Error] Abort failed"),
            BuckEngineError::ShardingNotActive => write!(f, "[Error] Sharding not active"),
            BuckEngineError::LengthNotSupported(typ) => {
                write!(f, "[Error] Length not supported for type: {}", typ)
            }
            BuckEngineError::Unknown => write!(f, "[Error] Unknown error"),
            BuckEngineError::TypeNotSupported(typ) => {
                write!(f, "[Error] Type not supported: {}", typ)
            }
        }
    }
}
