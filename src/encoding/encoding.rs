use std::collections::HashSet;

use crate::types::{types::BuckTypes, sets::{BuckSets, Setable, EqFloat}};

use super::errors::EncodingError;

pub fn encode_boolean(typ: bool) -> u8 {
    match typ {
        true => 0x01,
        false => 0x00,
    }
}

pub fn decode_boolean(byte: u8) -> Result<bool, EncodingError> {
    match byte {
        0x01 => Ok(true),
        0x00 => Ok(false),
        _ => Err(EncodingError::InternalError(format!("Invalid boolean value: {}", byte))),
    }
}

pub fn take_boolean(bytes: &mut &[u8]) -> Result<bool, EncodingError> {
    take_byte(bytes)
        .map_or(
            Err(EncodingError::InternalError("Unexpected end of bytes".into())), 
            decode_boolean,
        )
}

/// Encodes a byte vector. 0x00 is escaped as 0x00 0xff, and 0x00 0x00 is used as a terminator.
/// See: https://activesphere.com/blog/2018/08/17/order-preserving-serialization
pub fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(bytes.len() + 2);
    encoded.extend(
        bytes
            .iter()
            .flat_map(|b| match b {
                0x00 => vec![0x00, 0xff],
                _ => vec![*b],
            })
            .chain(vec![0x00, 0x00]),
    );

    encoded
}

/// Takes a single byte from a byte slice and shorten it
pub fn take_byte(bytes: &mut &[u8]) -> Option<u8> {
    if bytes.is_empty() {
        return None;
    }

    let byte = bytes[0];
    *bytes = &bytes[1..];

    Some(byte)
}

/// Decode byte vectors from a slice and shortens the slice
pub fn takes_bytes(bytes: &mut &[u8]) -> Result<Vec<u8>, EncodingError> {
    let mut decoded = Vec::with_capacity(bytes.len() / 2);
    let mut iter = bytes.iter().enumerate();

    let taken = loop {
        match iter.next().map(|(_, b)| b) {
            Some(0x00) => match iter.next() {
                Some((i, 0x00)) => break i + 1,    // 0x00 0x00 -> terminator
                Some((_, 0xff)) => decoded.push(0x00),     // 0x00 0xff is escape sequence for 0x00
                Some((_, b)) => Err(EncodingError::UnexpectedEndOf(format!("Invalid byte escape {}", b)))?,
                None => Err(EncodingError::UnexpectedEndOf(format!{"Unexpected end of bytes"}))?,
            }
            Some(b) => decoded.push(*b),
            None => Err(EncodingError::UnexpectedEndOf(format!{"Unexpected end of bytes"}))?,
        }
    };

    *bytes = &bytes[taken..];
    Ok(decoded)
}

pub fn encode_integer(n: i64) -> [u8; 8] {
    let mut bytes = n.to_be_bytes();

    // flip left-most bit in the first byte (sign bit)
    bytes[0] ^= 0x80;

    bytes
}

pub fn decode_integer(mut bytes: [u8; 8]) -> i64 {
    bytes[0] ^= 0x80;

    i64::from_be_bytes(bytes)
}

pub fn take_integer(bytes: &mut &[u8]) -> Result<i64, EncodingError> {
    if bytes.len() < 8 {
        return Err(EncodingError::InternalError(format!("Unable to decode integer from {} bytes", bytes.len())));
    }

    let n = decode_integer(bytes[..8].try_into().unwrap());
    *bytes = &bytes[8..];

    Ok(n)
}

/// Encodes an float(f64) value. Uses an big-endian, and flip sign bit to 1 if 0, otherwise flip all bits.
/// preserves the natural numerical ordering, with NaN at the end.
pub fn encode_float(n: f64) -> [u8; 8] {
    let mut bytes = n.to_be_bytes();

    if bytes[0] >> 7 & 1 == 0 {
        bytes[0] ^= 1 << 7;
    } else {
        bytes.iter_mut().for_each(|b| *b = !*b);
    }

    bytes
}

pub fn decode_float(mut bytes: [u8; 8]) -> f64 {
    if bytes[0] >> 7 & 1 == 1 {
        bytes[0] ^= 1 << 7;
    } else {
        bytes.iter_mut().for_each(|b| *b = !*b);
    }

    f64::from_be_bytes(bytes)
}

pub fn take_float<T: From<f64>>(bytes: &mut &[u8]) -> Result<T, EncodingError> {
    if bytes.len() < 8 {
        return Err(EncodingError::InternalError(format!("Unable to decode float from {} bytes", bytes.len())));
    }

    let n = decode_float(bytes[..8].try_into().unwrap());
    *bytes = &bytes[8..];

    Ok(T::from(n))
}

pub fn encode_string(string: &str) -> Vec<u8> {
    encode_bytes(string.as_bytes())
}

pub fn take_string(bytes: &mut &[u8]) -> Result<String, EncodingError> {
    takes_bytes(bytes)
        .and_then(|bytes| String::from_utf8(bytes)
        .map_err(|e| EncodingError::InternalError(format!("Invalid UTF-8 string: {}", e))))
}

pub fn encode_length(len: usize) -> [u8; 4] {
    (len as u32).to_be_bytes()
}

pub fn encode_set(set: &BuckSets) -> Vec<u8> {
    let mut encoded = Vec::new();

    // encode the length of the set
    encoded.extend(encode_length(set.data.len()));

    // encode each item in the set
    for item in &set.data {
        match item {
            Setable::String(s) => {
                encoded.push(0x01);
                encoded.extend(encode_string(s));
            }
            Setable::Boolean(b) => {
                encoded.push(0x02);
                encoded.push(encode_boolean(*b));
            }
            Setable::Integer(i) => {
                encoded.push(0x03);
                encoded.extend(encode_integer(*i));
            }
            Setable::Float(f) => {
                encoded.push(0x04);
                encoded.extend(encode_float(f.0));
            }
            Setable::Empty => {
                encoded.push(0x05);
            }
        }
    }

    encoded
}

pub fn take_set(bytes: &mut &[u8]) -> Result<BuckSets, EncodingError> {
    if bytes.len() < 4 {
        return Err(EncodingError::InternalError(format!("Unable to decode set from {} bytes", bytes.len())));
    }

    let len = take_integer(bytes)? as usize;
    let mut data = HashSet::new();

    for _ in 0..len {
        match bytes[0] {
            0x01 => {
                *bytes = &bytes[1..];
                let b = take_boolean(bytes)?;
                data.insert(Setable::Boolean(b));
            }
            0x02 => {
                *bytes = &bytes[1..];
                let s = take_string(bytes)?;
                data.insert(Setable::String(s));
            }
            0x03 => {
                *bytes = &bytes[1..];
                let i = take_integer(bytes)?;
                data.insert(Setable::Integer(i));
            }
            0x04 => {
                *bytes = &bytes[1..];
                let ef: EqFloat = take_float(bytes)?;
                data.insert(Setable::Float(ef));
            }
            0x05 => {
                *bytes = &bytes[1..];
                data.insert(Setable::Empty);
            }
            _ => {
                return Err(EncodingError::InternalError(format!("Invalid set type: {}", bytes[0])));
            }
        }
    }

    Ok(BuckSets { data })
}

pub fn encode_type(typ: &BuckTypes) -> Vec<u8> {
    match typ {
        BuckTypes::Boolean(b) => vec![0x01, encode_boolean(*b)],
        BuckTypes::Float(f) => vec![&[0x02][..], &encode_float(*f)].concat(),
        BuckTypes::Integer(i) => vec![&[0x03][..], &encode_integer(*i)].concat(),
        BuckTypes::String(s) => vec![&[0x04][..], &encode_string(s)].concat(),
        BuckTypes::Sets(s) => vec![&[0x05][..], &encode_set(s)].concat(),
        _ => unimplemented!("Encoding for type {:?} is not implemented", typ),
    }
}