use std::{
    fmt::format,
    io::{self, Error, Read, Write},
    net::TcpStream,
};

use crate::resp::{parser, utils, definitions};

pub struct Client {
    stream: TcpStream,
}

fn get_stream(host: &str, port: &str) -> TcpStream {
    TcpStream::connect(format!("{}:{}", host, port)).unwrap()
}

impl Client {
    pub fn connect(host: &str, port: &str) -> Self {
        let stream = get_stream(host, port);

        Client { stream }
    }

    pub fn handshake(&mut self) {
        self.write("HELLO 3\r\n".as_bytes()).unwrap();
        let response = self.read().unwrap();

        parser::parse_resp3(&response);
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        self.stream.write_all(data)?;

        Ok(())
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        // TODO: We need to be able to get larger buffers, probably by reading multiple times and
        // combining the buffers
        let mut buffer = [0; 512];
        let size = self.stream.read(&mut buffer)?;

        Ok(buffer[..size].to_vec())
    }

    // TODO: Error
    pub fn get_string<T: AsRef<str>>(&mut self, key: T) -> Result<Option<String>, Error> {
        let key = key.as_ref();
        let message = format!("*2\r\n$3\r\nget\r\n${}\r\n{}\r\n", key.len(), key);
        self.write(message.as_bytes())?;
        let response = self.read().unwrap();

        Ok(parser::parse_string(response).expect("failed to parse string"))
    }

    pub fn put_bulk_string<T: AsRef<str>>(&mut self, key: T, value: Vec<u8>) -> Result<(), Error>{
        let key = key.as_ref();
        let message = format!(
            "*3\r\n$3\r\nset\r\n${}\r\n{}\r\n${}\r\n",
            key.len(),
            key,
            value.len()
        )
        .into_bytes();

        self.write(&message)?;
        self.write(&value)?;
        self.write(definitions::END)?;

        let response = self.read().unwrap();
        let parsed = parser::parse_string(response).unwrap().unwrap();

        println!("Parsed {}", parsed);

        Ok(())
    }
}
