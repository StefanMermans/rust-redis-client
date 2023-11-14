use crate::client::Client;

mod client;
mod resp;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

fn main() {
    let mut client = Client::connect(HOST, PORT);
    client.handshake();

    let response = client.send_message_string("EVAL \"return { double = tonumber(ARGV[1]) }\" 0 15.1e0\r\n".to_string());
    println!("response\n{}", &response);
}
