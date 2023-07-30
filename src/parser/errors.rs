use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BuckParserError {
    UnknownQueryCommand,
    InvalidQueryCommand(String),
    InvalidKey(String),
    HashKeyIsEmpty(String),
    HashValueIsEmpty(String),
    HashValueIsNotInteger(String),
    // InvalidSetString(String),
    InvalidSetType(String),
    InvalidRange(String),
    UpdateValueContainsSpace(String),
}

impl fmt::Display for BuckParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckParserError::UnknownQueryCommand => write!(f, "[Error] Unknown query command"),
            BuckParserError::InvalidQueryCommand(query) => {
                write!(f, "[Error] Invalid query command: {}", query)
            }
            BuckParserError::InvalidKey(key) => write!(f, "[Error] Invalid key: {}", key),
            BuckParserError::HashKeyIsEmpty(hash) => {
                write!(f, "[Error] Invalid hash string. Value {} has no key", hash)
            }
            BuckParserError::HashValueIsEmpty(hash) => {
                write!(f, "[Error] Invalid hash string. Key {} has no value", hash)
            }
            BuckParserError::HashValueIsNotInteger(key) => {
                write!(f, "[Error] Hash value is not integer: {}", key)
            }
            // BuckParserError::InvalidSetString(set) => {
            //     write!(f, "[Error] Invalid set string: {}", set)
            // }
            BuckParserError::InvalidSetType(set) => {
                write!(f, "[Error] Invalid set type: {}", set)
            }
            BuckParserError::InvalidRange(range) => {
                write!(f, "[Error] Invalid range: {}", range)
            }
            BuckParserError::UpdateValueContainsSpace(key) => {
                write!(f, "[Error] Update query value contains space: {}", key)
            }
        }
    }
}
