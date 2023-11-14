use crate::client::Client;

mod client;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

fn main() {
    let mut client = Client::connect(HOST, PORT);
    client.handshake();
}

