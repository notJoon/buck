use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BuckTypeError {
    UnknownCommand(String),
    ListIsEmpty,
}

impl fmt::Display for BuckTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckTypeError::UnknownCommand(command) => write!(f, "[Error] Unknown command: {}", command),
            BuckTypeError::ListIsEmpty => write!(f, "[Error] List is empty"),
        }
    }
}

