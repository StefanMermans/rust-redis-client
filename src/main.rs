use crate::client::Client;
use crate::resp::*;

mod client;
mod resp;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

fn main() {
    let mut client = Client::connect(HOST, PORT);
    // client.handshake();

    // let encoded = encoder::encode_string("test value");
    //println!("{}", utils::to_debug_string(&encoded).unwrap());
    //let response = client.send_message("*4\r\n$5\r\nRPUSH\r\n$5\r\nmyKey\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".as_bytes());
    // let response_debug = utils::to_debug_string(&response).unwrap();
    //println!("response\n{}", &response_debug);
    let myValue = client.get_string("myKey").unwrap();

    println!("{}", myValue);
}


#[cfg(test)]
mod test;

