use crate::client::Client;

mod client;
mod resp;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

fn main() {
    let mut client = Client::connect(HOST, PORT);
    client.handshake();

    let response = client.send_message_string("*4\r\n$5\r\nRPUSH\r\n$5\r\nmyKey\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".to_string());
    println!("response\n{}", &response);
}


#[cfg(test)]
mod test;

