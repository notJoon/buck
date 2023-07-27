use crate::types::BuckTypes;

#[derive(Debug, PartialEq)]
pub enum BuckQuery {
    Get(Vec<String>),
    Set(String, BuckTypes),
    Unknown,
}
