use regex::Regex;

use crate::types::BuckTypes;

use super::{errors::BuckParserError, query::BuckQuery};

pub type BuckParserResult = Result<BuckQuery, BuckParserError>;

pub fn get_value_type(value: &str) -> Result<BuckTypes, BuckParserError> {
    // remove all underscores from value to allow for parse large numbers
    if value.contains('_') {
        let value = value.replace("_", "");
        return get_value_type(&value);
    }

    if let Ok(ival) = value.parse::<i64>() {
        return Ok(BuckTypes::Integer(ival));
    }

    if let Ok(fval) = value.parse::<f64>() {
        return Ok(BuckTypes::Float(fval));
    }

    match value {
        "true" => return Ok(BuckTypes::Boolean(true)),
        "false" => return Ok(BuckTypes::Boolean(false)),
        // string value must be wrapped in double quotes
        _ => {
            if value.starts_with('"') && value.ends_with('"') {
                return Ok(BuckTypes::String(value[1..value.len() - 1].to_string()));
            }
        }
    }

    Ok(BuckTypes::Unknown(value.to_string()))
}

fn is_valid_key(key: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();

    re.is_match(key)
}

fn get_invalid_keys(keys: Vec<String>) -> Vec<String> {
    keys
        .clone()
        .into_iter()
        .filter(|key| !is_valid_key(key))
        .collect::<Vec<String>>()
} 

pub fn parse_query(query: &str) -> BuckParserResult {
    let parts: Vec<&str> = query.splitn(2, ' ').collect();

    match parts.get(0) {
        Some(&"GET") => {
            if let Some(key) = parts.get(1) {
                let keys: Vec<String> = key.split(' ').map(|s| s.to_string()).collect();

                let invalid_keys = get_invalid_keys(keys.clone());

                if !invalid_keys.is_empty() {
                    return Err(BuckParserError::InvalidKey(invalid_keys.join(", ")));
                }

                return Ok(BuckQuery::Get(keys));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        }
        Some(&"INSERT") => {
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.get(0), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let buck_type = get_value_type(value);

                    return Ok(BuckQuery::Insert(key.to_string(), buck_type?));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        }
        Some(&"UPDATE") => {
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.get(0), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let buck_type = get_value_type(value)?;

                    if value.contains(' ') {
                        if let BuckTypes::String(_) = buck_type {
                            // do nothing
                        } else {
                            return Err(BuckParserError::UpdateValueContainsSpace(value.to_string()));
                        }
                    }

                    return Ok(BuckQuery::Update(key.to_string(), buck_type));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        }
        Some(&"REMOVE") => {
            if let Some(key) = parts.get(1) {
                let keys: Vec<String> = key.split(' ').map(|s| s.to_string()).collect();

                let invalid_keys = get_invalid_keys(keys.clone());

                if !invalid_keys.is_empty() {
                    return Err(BuckParserError::InvalidKey(invalid_keys.join(", ")));
                }

                return Ok(BuckQuery::Remove(keys));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        }
        _ => Err(BuckParserError::InvalidQueryCommand(query.to_string())),
    }
}
