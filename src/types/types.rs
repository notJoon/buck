use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::parser::errors::BuckParserError;
use crate::parser::parse::get_value_type;

use super::list::BuckList;

#[derive(Debug, PartialEq, Clone)]
pub enum BuckTypes {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(BuckList),
    Hash(HashMap<String, BuckTypes>),
    Sets(HashSet<String>),
    Unknown(String),
}

pub fn parse_list(list_input: &str) -> Result<BuckList, BuckParserError> {
    let mut list = BuckList::new();

    // expect input -> value1,value2, ...
    for value in list_input.split(',') {
        let value = get_value_type(value)?;
        list.data.push(value);
    }

    Ok(list)
}

pub fn parse_hash(hash_input: &str) -> Result<HashMap<String, BuckTypes>, BuckParserError> {
    let hash_input = hash_input.replace(' ', "");

    // expect input -> key1:value1,key2:value2, ...
    let mut hash = HashMap::new();
    for part in hash_input.split(',') {
        let kv: Vec<&str> = part.splitn(2, ':').collect();

        if kv[0].is_empty() {
            let value = kv[1].replace(':', "");
            return Err(BuckParserError::HashKeyIsEmpty(value));
        }

        if kv[1].is_empty() {
            let key = kv[0].replace(':', "");
            return Err(BuckParserError::HashValueIsEmpty(key));
        }

        let value = get_value_type(kv[1])?;
        hash.insert(kv[0].to_owned(), value);
    }

    Ok(hash)
}

pub fn parse_sets(set_input: &str) -> Result<HashSet<String>, BuckParserError> {
    let set = set_input
        .replace(' ', "")
        .split(',')
        .map(|s| s.to_owned())
        .collect::<HashSet<String>>();

    Ok(set)
}

impl fmt::Display for BuckTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckTypes::Integer(ival) => write!(f, "{}", ival),
            BuckTypes::Float(fval) => write!(f, "{}", fval),
            BuckTypes::Boolean(bval) => write!(f, "{}", bval),
            BuckTypes::String(sval) => write!(f, "{}", sval),
            BuckTypes::List(lval) => {
                let mut list_string = String::new();
                for (i, item) in lval.data.iter().enumerate() {
                    list_string.push_str(&format!("{}: {}\n", i, item));
                }

                write!(f, "{}", list_string)
            }
            BuckTypes::Hash(hval) => {
                let mut hash_string = String::new();
                for (key, value) in hval {
                    hash_string.push_str(&format!("{}:{},", key, value));
                }

                write!(f, "{}", hash_string)
            }
            BuckTypes::Sets(sval) => {
                let mut set_string = String::new();
                for value in sval {
                    set_string.push_str(&format!("{},", value));
                }

                write!(f, "{}", set_string)
            }
            BuckTypes::Unknown(uval) => write!(f, "{}", uval),
        }
    }
}
