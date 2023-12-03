use std::{collections::HashMap, default, usize};

use super::{definitions, utils};

#[derive(Debug)]
pub struct ParseError {
    message: String,
    index: usize,
}


impl ParseError {
    pub fn new(message: String, index: usize) -> Self {
        Self { message, index }
    }
}

pub fn parse_resp3(resp3: &Vec<u8>) -> Result<HashMap<String, Vec<u8>>, ParseError> {
    let debug_string = utils::to_debug_string(resp3).unwrap();

    parse_map(resp3)
}

pub fn parse_map(resp3: &Vec<u8>) -> Result<HashMap<String, Vec<u8>>, ParseError> {
    let map = HashMap::new();
    let mut identified = false;

    for byte in resp3 {
        if !identified && *byte != definitions::MAP {
            return Err(ParseError::new(format!("Expected identifier \"{}\"", definitions::MAP), 0))
        }
        identified = true;

        //byte
    }

    Ok(map)
}

pub fn parse_string(resp3: Vec<u8>) -> Result<Option<String>, ParseError> {
    if resp3.len() == 0 {
        return Err(ParseError::new("Empty resp string".to_string(), 0));
    }

    match resp3[0] {
        // TODO: maybe check if \r\n is present like expected.
        definitions::SIMPLE_STRING => parse_simple_string(&resp3[1..]),
        definitions::BULK_STRING => parse_bulk_string(&resp3[1..]),
        default => Err(ParseError::new(format!("unsupported identifier \"{}\"", resp3[0]), 0)),
    }
}

fn parse_bulk_string(resp3: &[u8]) -> Result<Option<String>, ParseError> {
    if let Some((len, offset)) = parse_len(resp3)? {
        let result = &resp3[offset..len + offset];

        let result_string = String::from_utf8(result.to_vec()).unwrap();

        return Ok(Some(result_string));
    }

    Ok(None)
}

fn parse_simple_string(resp3: &[u8]) -> Result<Option<String>, ParseError> {
    if resp3.len() == 0 {
        return Err(ParseError::new("Empty resp string".to_string(), 0));
    }

    let mut string = String::new();

    for (index, byte) in resp3.iter().enumerate() {
        if *byte == definitions::END[0] {
            if resp3.len() < index + 1 {
                return Err(ParseError::new("Expected \\n".into(), index))
            }

            if resp3[index + 1] != definitions::END[1] {
                return Err(ParseError::new("Expected \\n".into(), index));
            }

            break;
        }

        string.push(*byte as char);
    }

    Ok(Some(string))
}

pub fn parse_len(resp3: &[u8]) -> Result<Option<(usize, usize)>, ParseError> {
    let mut len: usize = 0;
    // Start offset at 2 to account for the \r\n characters
    let mut offset: usize = 2;

    for byte in resp3 {
        if *byte == definitions::END[0] {
            break;
        }

        // Check if byte is - (for null values)
        if *byte == 45 {
            return Ok(None);
        }

        if *byte > 57 || *byte < 48 {
            return Err(ParseError::new(format!("Byte '{}' ouf of range", byte).to_string(), 0));
        }

        len *= 10;
        // This -48 doesn't make sense to me.
        // TODO: figure out the correct way to convert a utf-8 encoded char to a number
        len += (byte - 48) as usize;
        offset += 1;
    }

    Ok(Some((len, offset)))
}
