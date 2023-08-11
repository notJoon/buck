use regex::Regex;
use std::collections::HashMap;

use crate::types::types::{parse_hash, parse_list, parse_sets, BuckTypes};

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
            if value.starts_with('"') && value.ends_with('"') || value.starts_with('\'') && value.ends_with('\'') {
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
        BuckTokens::Get => handle_get(query, parts),
        BuckTokens::Insert => handle_insert(query, parts),
        BuckTokens::Update => handle_update(query, parts),
        BuckTokens::Remove => handle_remove(query, parts),
        BuckTokens::Commit => Ok(BuckQuery::Commit),
        BuckTokens::Rollback => Ok(BuckQuery::Rollback),
        BuckTokens::Shard => handle_shard(query, parts),
        BuckTokens::Type => handle_type(query, parts),

        // list things
        BuckTokens::LPush => handle_lpush(query, parts),
        BuckTokens::LPop => handle_lpop(query, parts),
        BuckTokens::SAdd => handle_sadd(query, parts),
        BuckTokens::SRem => handle_srem(query, parts),
        BuckTokens::SInter => handle_sinter(query, parts),
        BuckTokens::HSet => handle_hset(query, parts),
        BuckTokens::Length => handle_length(query, parts),
        BuckTokens::Exit => Ok(BuckQuery::Exit),
        BuckTokens::Clear => Ok(BuckQuery::Clear),
        _ => Err(BuckParserError::InvalidQueryCommand(query.to_owned())),
    }
}

fn handle_get(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_insert(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_update(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_remove(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_shard(query: &str, parts: Vec<&str>) -> BuckParserResult {
    if let Some(shard) = parts.get(1) {
        let n_shard = shard.trim().parse::<usize>();

        if let Ok(n_shard) = n_shard {
            return Ok(BuckQuery::Shard(n_shard));
        }
    }

    Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
}

fn handle_type(query: &str, parts: Vec<&str>) -> BuckParserResult {
    if let Some(key) = parts.get(1) {
        if !is_valid_key(key) {
            return Err(BuckParserError::InvalidKey(key.to_string()));
        }

        return Ok(BuckQuery::Type(key.to_string()));
    }

    Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
}

fn handle_lpush(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_lpop(query: &str, parts: Vec<&str>) -> BuckParserResult {
    if let Some(key) = parts.get(1) {
        if !is_valid_key(key) {
            return Err(BuckParserError::InvalidKey(key.to_string()));
        }

        return Ok(BuckQuery::LPop(key.to_string()));
    }

    Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
}

fn handle_sadd(query: &str, parts: Vec<&str>) -> BuckParserResult {
    if let Some(key) = parts.get(1) {
        let key_value: Vec<&str> = key.splitn(2, ' ').collect();

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

fn handle_srem(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_sinter(query: &str, parts: Vec<&str>) -> BuckParserResult {
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

fn handle_length(query: &str, parts: Vec<&str>) -> BuckParserResult {
    if let Some(key) = parts.get(1) {
        if !is_valid_key(key) {
            return Err(BuckParserError::InvalidKey(key.to_string()));
        }

        return Ok(BuckQuery::Len(key.to_string()));
    }

    Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
}

// hset bike1 model:Deimos brand:Ergonom type:'Enduro bikes' price:4972
fn handle_hset(query: &str, parts: Vec<&str>) -> BuckParserResult {
    if let Some(key) = parts.get(1) {
        let key_value: Vec<&str> = key.splitn(2, ' ').collect();
        // >>> [bike1, model:Deimos brand:Ergonom type:'Enduro bikes' price:4972]

        if let (Some(key), Some(value)) = (key_value.first(), key_value.get(1)) {
            if !is_valid_key(key) {
                return Err(BuckParserError::InvalidKey(key.to_string()));
            }

            let fields_kv_pair: Vec<&str> = value.split(' ').collect();
            // >>> [model:Deimos, brand:Ergonom, type:'Enduro bikes', price:4972]

            let mut parsed_fields: HashMap<String, BuckTypes> = HashMap::new();

            for field_kv in fields_kv_pair {
                let field_kv_pair: Vec<&str> = field_kv.splitn(2, ':').collect();
                // >>> [model, Deimos], [brand, Ergonom], [type, 'Enduro bikes'], [price, 4972]

                if let (Some(name), Some(value)) = (field_kv_pair.first(), field_kv_pair.get(1)) {
                    if !is_valid_key(name) {
                        return Err(BuckParserError::InvalidKey(name.to_string()));
                    }

                    let value = get_value_type(value)?;

                    parsed_fields.insert(name.to_string(), value);
                }
            }

            return Ok(BuckQuery::HSet(key.to_string(), parsed_fields));
        }
    }

    Err(BuckParserError::InvalidQueryCommand(query.to_owned()))
}