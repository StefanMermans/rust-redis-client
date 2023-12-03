use super::parser;
use super::{definitions, parser::parse_len};
use std::{borrow::BorrowMut, default, string::ParseError};

pub struct Parser {
    data: Data,
}

struct Data {
    resp3: Vec<u8>,
    index: usize,
}

struct StringParser<'a> {
    data: &'a mut Data,
}

struct LengthParser<'a> {
    data: &'a mut Data,
}

struct NewLineParser<'a, T> {
    data: &'a mut Data,
    value: T,
}

impl<'a, T> NewLineParser<'a, T> {
    pub fn new(data: &'a mut Data, value: T) -> Self {
        Self { data, value }
    }
}

impl<'a> LengthParser<'a> {
    pub fn new(data: &'a mut Data) -> Self {
        Self { data }
    }

    pub fn parse(&mut self) -> Result<Option<usize>, parser::ParseError> {
        // TODO: implement
        if let Some((len, offset)) = parse_len(&mut self.data.resp3[self.data.index..])? {
            self.data.index += 2;

            return Ok(Some(len));
        }

        return Ok(None);
    }
}

impl<'a> StringParser<'a> {
    pub fn new(data: &'a mut Data) -> Self {
        Self { data }
    }

    pub fn parse(&mut self) -> Result<Option<String>, parser::ParseError> {
        let len = { LengthParser::new(self.data).parse()? };

        if let Some(len) = len {
            println!("Len: {}", &len);

            let string = &self.data.resp3[self.data.index + 2..self.data.index + len + 2];

            return Ok(Some(String::from_utf8(string.to_vec()).unwrap()));
        }

        Ok(None)
    }
}

impl Parser {
    pub fn new(resp3: Vec<u8>) -> Self {
        Self {
            data: Data { resp3, index: 0 },
        }
    }

    pub fn parse(&mut self) {
        // TODO: handle the element doesn't exist
        let byte = self.data.resp3.get(self.data.index).unwrap();
        self.data.index += 1;

        let string = match *byte {
            definitions::BULK_STRING => StringParser::new(&mut self.data).parse(),
            default => todo!("Error + more definitions"),
        };

        println!("string {}", string.unwrap().unwrap());
    }
}
