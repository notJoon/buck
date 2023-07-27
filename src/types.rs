use std::collections::{HashMap, HashSet};

use crate::parser::errors::BuckParserError;
use crate::parser::parse::get_value_type;

#[derive(Debug, PartialEq, Clone)]
pub enum BuckTypes {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Hash(HashMap<String, BuckTypes>),
    Sets(HashSet<String>),
    Unknown(String),
}

pub fn parse_hash(hash_input: &str) -> Result<HashMap<String, BuckTypes>, BuckParserError> {
    let hash_input = hash_input.replace(" ", "");

    // expect input -> key1:value1,key2:value2, ...
    let mut hash = HashMap::new();
    for part in hash_input.split(',') {
        let kv: Vec<&str> = part.splitn(2, ':').collect();

        if kv[0].is_empty() {
            let value = kv[1].replace(":", "").to_owned();
            return Err(BuckParserError::HashKeyIsEmpty(value));
        }

        if kv[1].is_empty() {
            let key = kv[0].replace(":", "").to_owned();
            return Err(BuckParserError::HashValueIsEmpty(key));
        }

        let value = get_value_type(kv[1])?;
        hash.insert(kv[0].to_string(), value);
    }

    Ok(hash)
}

pub fn parse_sets(set_input: &str) -> Result<HashSet<String>, BuckParserError> {
    let set = set_input
        .replace(" ", "")
        .split(',')
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    Ok(set)
}
