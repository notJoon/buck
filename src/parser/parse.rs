use regex::Regex;

use crate::types::types::{parse_hash, parse_sets, parse_list, BuckTypes};

use super::{errors::BuckParserError, query::BuckQuery};

pub type BuckParserResult = Result<BuckQuery, BuckParserError>;

pub fn get_value_type(value: &str) -> Result<BuckTypes, BuckParserError> {
    // remove all underscores from value to allow for parse large numbers
    if value.contains('_') {
        let value = value.replace('_', "");
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
                return Ok(BuckTypes::String(value[1..value.len() - 1].into()));
            }

            if value.starts_with('[') && value.ends_with(']') {
                return Ok(BuckTypes::List(parse_list(&value[1..value.len() - 1]).unwrap()));
            }

            if value.starts_with('{') && value.ends_with('}') {
                return Ok(BuckTypes::Hash(parse_hash(&value[1..value.len() - 1])?));
            }

            if value.starts_with('(') && value.ends_with(')') {
                return Ok(BuckTypes::Sets(parse_sets(&value[1..value.len() - 1])?));
            }
        }
    }

    Ok(BuckTypes::Unknown(value.to_owned()))
}

fn is_valid_key(key: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();

    re.is_match(key)
}

fn get_invalid_keys(keys: Vec<String>) -> Vec<String> {
    keys.into_iter()
        .filter(|key| !is_valid_key(key))
        .collect::<Vec<String>>()
}

fn parse_range(input: &str) -> Result<Vec<BuckTypes>, BuckParserError> {
    if input.contains("..") {
        let parts = input.split("..").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(BuckParserError::InvalidRange(input.to_owned()));
        }

        let start = parts[0].parse::<i32>().map_err(|_| {
            BuckParserError::InvalidRange(format!("Invalid start value: {}", parts[0]))
        })?;

        let end = parts[1].parse::<i32>().map_err(|_| {
            BuckParserError::InvalidRange(format!("Invalid end value: {}", parts[1]))
        })?;

        let values = (start..end).map(|i| BuckTypes::Integer(i as i64)).collect();

        return Ok(values);
    }

    let values: Vec<BuckTypes> = input
        .split(' ')
        .map(|s| get_value_type(s))
        .collect::<Result<Vec<BuckTypes>, BuckParserError>>()?;

    Ok(values)
}

pub fn parse_query(query: &str) -> BuckParserResult {
    let parts: Vec<&str> = query.splitn(2, ' ').collect();

    match parts.first() {
        Some(&"GET") => {
            if let Some(key) = parts.get(1) {
                let keys: Vec<String> = key.split(' ').map(|s| s.to_string()).collect();

                let invalid_keys = get_invalid_keys(keys.clone());

                if !invalid_keys.is_empty() {
                    return Err(BuckParserError::InvalidKey(invalid_keys.join(", ")));
                }

                return Ok(BuckQuery::Get(keys));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        Some(&"INSERT") => {
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let buck_type = get_value_type(value);

                    return Ok(BuckQuery::Insert(key.to_string(), buck_type?));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        Some(&"UPDATE") => {
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let buck_type = get_value_type(value)?;

                    if value.contains(' ') {
                        match buck_type {
                            BuckTypes::String(_) | BuckTypes::Hash(_) | BuckTypes::Sets(_) | BuckTypes::List(_) => {}
                            _ => {
                                return Err(BuckParserError::UpdateValueContainsSpace(
                                    value.to_string(),
                                ))
                            }
                        }
                    }

                    return Ok(BuckQuery::Update(key.to_string(), buck_type));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
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

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        Some(&"COMMIT") => Ok(BuckQuery::Commit),
        Some(&"ROLLBACK") => Ok(BuckQuery::Rollback),
        Some(&"SHARD") => {
            if let Some(shard) = parts.get(1) {
                let n_shard = shard.trim().parse::<usize>();

                if let Ok(n_shard) = n_shard {
                    return Ok(BuckQuery::Shard(n_shard));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        Some(&"TYPE") => {
            if let Some(key) = parts.get(1) {
                if !is_valid_key(key) {
                    return Err(BuckParserError::InvalidKey(key.to_string()));
                }

                return Ok(BuckQuery::Type(key.to_string()));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }

        // list things
        Some(&"lpush") => {
            // lpush key value1 value2 ...
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let values: Vec<BuckTypes> = parse_range(value)?;

                    return Ok(BuckQuery::Lpush(key.to_string(), values));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        Some(&"exit") => Ok(BuckQuery::Exit),
        _ => Err(BuckParserError::InvalidQueryCommand(query.to_owned())),
    }
}
