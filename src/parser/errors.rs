#[derive(Debug, PartialEq)]
pub enum BuckParserError {
    UnknownQueryCommand,
    InvalidQueryCommand(String),
}