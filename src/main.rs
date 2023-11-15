use crate::client::Client;

mod client;
mod resp;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

fn main() {
    let mut client = Client::connect(HOST, PORT);
    client.handshake();

    let response = client.send_message_string("*3\r\n$3\r\nSET\r\n$5\r\nmyKey\r\n$7\r\nmyValue\r\r".to_string());
    println!("response\n{}", &response);
    let response = client.send_message_string("*2\r\n$3\r\nGET\r\n$5\r\nmyKey\r\n".to_string());
    println!("response\n{}", &response);
}


#[cfg(test)]
mod test;

