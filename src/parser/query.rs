use crate::types::BuckTypes;

#[derive(Debug, PartialEq)]
pub enum BuckQuery {
    Get(Vec<String>),
    Insert(String, BuckTypes),
    Update(String, BuckTypes),
    Remove(Vec<String>),
    Unknown,
}
