use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EncodingError {
    InternalError(String),
    UnexpectedEndOf(String),
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodingError::InternalError(msg) => write!(f, "{}", msg),
            EncodingError::UnexpectedEndOf(msg) => write!(f, "{}", msg),
        }
    }
}