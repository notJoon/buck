use crate::types::BuckTypes;

use super::{query::BuckQuery, errors::BuckParserError};

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

    // TODO: what if string value contains UpperCase? how to handle this?
    match value.to_lowercase().as_str() {
        "true" => return Ok(BuckTypes::Boolean(true)),
        "false" => return Ok(BuckTypes::Boolean(false)),
        // string value must be wrapped in double quotes
        _ => {
            if value.starts_with('"') && value.ends_with('"') {
                return Ok(BuckTypes::String(value[1..value.len()-1].to_string()));
            }
        }
    }

    Ok(BuckTypes::Unknown(value.to_string()))
}

pub fn parse_query(query: &str) -> BuckParserResult {
    let parts: Vec<&str> = query.splitn(3, ' ').collect();

    match parts.get(0) {
        Some(&"GET") => {
            if let Some(key) = parts.get(1) {
                return Ok(BuckQuery::Get(key.to_string()));
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        },
        Some(&"SET") => {
            if let (Some(key), Some(value)) = (parts.get(1), parts.get(2)) {
                let buck_type = get_value_type(value);

                return Ok(BuckQuery::Set(key.to_string(), buck_type?))
            }

            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        },
        _ => Err(BuckParserError::InvalidQueryCommand(query.to_string()))
    }
}