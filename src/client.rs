use std::{
    io::{Read, Write},
    net::TcpStream,
};

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
        let response = self.send_message_string("HELLO 3\r\n".to_string());

        println!("===REDIS===\n{}\n===END REDIS===", response,);
    }

    pub fn send_message(&mut self, data: &[u8]) -> Vec<u8> {
        self.stream.write_all(data).unwrap();

        // TODO: We need to be able to get larger buffers, probably by reading multiple times and
        // combining the buffers
        let mut buffer = [0; 512];
        let size = self.stream.read(&mut buffer).unwrap();

        buffer[..size].to_vec()
    }

    pub fn send_message_string(&mut self, data: String) -> String {
        let response = self.send_message(data.as_bytes());

        String::from_utf8_lossy(&response).to_string()
    }
}
