use crate::types::BuckTypes;

#[derive(Debug, PartialEq)]
pub enum BuckQuery {
    Get(String),
    Set(String, BuckTypes),
    Unknown,
}