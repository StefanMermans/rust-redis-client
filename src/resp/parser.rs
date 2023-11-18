use std::{collections::HashMap, default, usize};

use super::{definitions, utils};

#[derive(Debug)]
pub struct ParseError {
    // TODO: add error specif info
}

pub fn parse_resp3(resp3: &Vec<u8>) -> Result<HashMap<String, Vec<u8>>, ParseError> {
    let debug_string = utils::to_debug_string(resp3).unwrap();
    println!("parsed: {}", &debug_string);

    parse_map(resp3)
}

pub fn parse_map(resp3: &Vec<u8>) -> Result<HashMap<String, Vec<u8>>, ParseError> {
    let map = HashMap::new();
    let mut identified = false;

    for byte in resp3 {
        if !identified && *byte != definitions::MAP {
            return Err(ParseError {});
        }
        identified = true;

        //byte
    }

    Ok(map)
}

pub fn parse_string(resp3: Vec<u8>) -> Result<String, ParseError> {
    let debug_string = utils::to_debug_string(&resp3).unwrap();
    println!("parse string: \"{}\"", debug_string);

    if resp3.len() == 0 {
        println!("len 0");
        return Err(ParseError {});
    }

    match resp3[0] {
        // TODO: maybe check if \r\n is present like expected.
        definitions::SIMPLE_STRING => parse_simple_string(&resp3[1..]),
        definitions::BULK_STRING => parse_bulk_string(&resp3[1..]),
        _ => Err(ParseError {}),
    }
}

fn parse_bulk_string(resp3: &[u8]) -> Result<String, ParseError> {
    let debug = utils::to_debug_string(&resp3.to_vec()).unwrap();
    println!("bulk string: \"{}\"", debug);
    let (len, offset) = parse_len(resp3)?;
    println!("Len: {}, offset: {}", len, offset);

    let result = &resp3[offset..len+offset];

    Ok(String::from_utf8(result.to_vec()).unwrap())
}

fn parse_simple_string(resp3: &[u8]) -> Result<String, ParseError> {
    if resp3.len() == 0 {
        return Err(ParseError {});
    }

    let debug = utils::to_debug_string(&resp3.to_vec()).unwrap();
    println!("Simple string: {}", debug);
    Ok("test".to_string())
}

fn parse_len(resp3: &[u8]) -> Result<(usize, usize), ParseError> {
    let mut len: usize = 0;
    // Start offset at 2 to account for the \r\n characters
    let mut offset: usize = 2;

    for byte in resp3 {
        if *byte == b'\r' {
            break;
        }

        if *byte > 57 || *byte < 48 {
            println!("number out of range");

            return Err(ParseError {  });
        }

        len *= 10;
        // This -48 doesn't make sense to me.
        // TODO: figure out the correct way to convert a utf-8 encoded char to a number
        len += (byte - 48) as usize;
        offset += 1;
    }

    Ok((len, offset))
}
