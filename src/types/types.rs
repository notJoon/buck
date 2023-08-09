use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::parser::errors::BuckParserError;
use crate::parser::parse::get_value_type;

use super::hash::BuckHash;
use super::list::BuckList;
use super::sets::{BuckSets, Setable};

#[derive(Debug, PartialEq, Clone)]
pub enum BuckTypes {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(BuckList),
    Hash(BuckHash),
    Sets(BuckSets),
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

pub fn parse_hash(hash_input: &str) -> Result<BuckHash, BuckParserError> {
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

    Ok(BuckHash { data: hash })
}

pub fn parse_sets(set_input: &str) -> Result<BuckSets, BuckParserError> {
    let set_input = set_input.replace(' ', "");

    // expect input -> value1,value2, ...
    let mut set = HashSet::new();
    for value in set_input.split(',') {
        // insert value into set which type is Setable
        match get_value_type(value)? {
            BuckTypes::Integer(ival) => {
                set.insert(Setable::Integer(ival));
            }
            BuckTypes::Boolean(bval) => {
                set.insert(Setable::Boolean(bval));
            }
            BuckTypes::String(sval) => {
                set.insert(Setable::String(sval));
            }
            _ => {
                return Err(BuckParserError::InvalidSetType(value.to_owned()));
            }
        }
    }

    Ok(BuckSets { data: set })
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
                for (key, value) in hval.data.iter() {
                    hash_string.push_str(&format!("{}: {}\n", key, value));
                }

                write!(f, "{}", hash_string)
            }
            BuckTypes::Sets(sval) => {
                let mut set_string = String::new();
                for (i, item) in sval.data.iter().enumerate() {
                    set_string.push_str(&format!("{}: {}\n", i, item));
                }

                write!(f, "{}", set_string)
            }
            BuckTypes::Unknown(uval) => write!(f, "{}", uval),
        }
    }
}
