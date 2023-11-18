use std::{
    io::{self, Error, Read, Write},
    net::TcpStream,
};

use crate::resp::parser;

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
        let response = self.send_message("HELLO 3\r\n".as_bytes()).unwrap();

        parser::parse_resp3(&response);
    }

    pub fn send_message(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
        self.stream.write_all(data)?;

        Ok(self.read()?)
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        // TODO: We need to be able to get larger buffers, probably by reading multiple times and
        // combining the buffers
        let mut buffer = [0; 512];
        let size = self.stream.read(&mut buffer)?;

        Ok(buffer[..size].to_vec())
    }

    // TODO: Error
    pub fn get_string<T: AsRef<str>>(&mut self, key: T) -> Result<String, Error> {
        let key = key.as_ref();
        let message = format!("*2\r\n$3\r\nget\r\n${}\r\n{}\r\n", key.len(), key);
        let response = self.send_message(message.as_bytes())?;

        Ok(parser::parse_string(response).expect("failed to parse string"))
    }
}
