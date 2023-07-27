use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BuckParserError {
    UnknownQueryCommand,
    InvalidQueryCommand(String),
    InvalidKey(String),
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
            BuckParserError::UpdateValueContainsSpace(key) => {
                write!(f, "[Error] Update query value contains space: {}", key)
            }
        }
    }
}
