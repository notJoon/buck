use regex::Regex;

use crate::types::{
    types::{parse_hash, parse_list, parse_sets, BuckTypes},
};

use super::{errors::BuckParserError, query::BuckQuery, tokens::BuckTokens};

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
                return Ok(BuckTypes::String(value[1..value.len() - 1].to_owned()));
            }

            if value.starts_with('[') && value.ends_with(']') {
                return Ok(BuckTypes::List(parse_list(&value[1..value.len() - 1])?));
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
    let command = BuckTokens::from_str(parts[0]);

    match command {
        BuckTokens::Get => {
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
        // TODO: handle range operator
        BuckTokens::Insert => {
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
        // TODO: handle range operator
        BuckTokens::Update => {
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let buck_type = get_value_type(value)?;

                    if value.contains(' ') {
                        match buck_type {
                            BuckTypes::String(_)
                            | BuckTypes::Hash(_)
                            | BuckTypes::Sets(_)
                            | BuckTypes::List(_) => {}
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
        // TODO: handle range operator
        BuckTokens::Remove => {
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
        BuckTokens::Commit => Ok(BuckQuery::Commit),
        BuckTokens::Rollback => Ok(BuckQuery::Rollback),
        BuckTokens::Shard => {
            if let Some(shard) = parts.get(1) {
                let n_shard = shard.trim().parse::<usize>();

                if let Ok(n_shard) = n_shard {
                    return Ok(BuckQuery::Shard(n_shard));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::Type => {
            if let Some(key) = parts.get(1) {
                if !is_valid_key(key) {
                    return Err(BuckParserError::InvalidKey(key.to_string()));
                }

                return Ok(BuckQuery::Type(key.to_string()));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }

        // list things
        BuckTokens::LPush => {
            // lpush key value1 value2 ...
            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let values: Vec<BuckTypes> = parse_range(value)?;

                    return Ok(BuckQuery::LPush(key.to_string(), values));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::LPop => {
            // lpop key

            if let Some(key) = parts.get(1) {
                if !is_valid_key(key) {
                    return Err(BuckParserError::InvalidKey(key.to_string()));
                }

                return Ok(BuckQuery::LPop(key.to_string()));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::SAdd => {
            // sadd key value1 value2 ... | sadd key (value1, value2, ...) | sadd key value1..value2

            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if key_value.len() == 1 {
                    // insert empty set
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    return Ok(BuckQuery::SAdd(key.to_string(), vec![]));
                }

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let values: Vec<BuckTypes> = parse_range(value)?;

                    return Ok(BuckQuery::SAdd(key.to_string(), values));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::SRem => {
            // srem key value1 value2 ...

            if let Some(key) = parts.get(1) {
                let key_value: Vec<&str> = key.splitn(2, ' ').collect();

                if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
                    if !is_valid_key(key) {
                        return Err(BuckParserError::InvalidKey(key.to_string()));
                    }

                    let values: Vec<BuckTypes> = parse_range(value)?;

                    return Ok(BuckQuery::SRem(key.to_string(), values));
                }
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::SInter => {
            // sinter key1 key2 ...

            if let Some(key) = parts.get(1) {
                let keys: Vec<String> = key.split(' ').map(|s| s.to_string()).collect();

                let invalid_keys = get_invalid_keys(keys.clone());

                if !invalid_keys.is_empty() {
                    return Err(BuckParserError::InvalidKey(invalid_keys.join(", ")));
                }

                return Ok(BuckQuery::SInter("".to_string(), keys));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::Length => {
            // len key
            if let Some(key) = parts.get(1) {
                if !is_valid_key(key) {
                    return Err(BuckParserError::InvalidKey(key.to_string()));
                }

                return Ok(BuckQuery::Len(key.to_string()));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
        }
        BuckTokens::Exit => Ok(BuckQuery::Exit),
        _ => Err(BuckParserError::InvalidQueryCommand(query.to_owned())),
    }
}
