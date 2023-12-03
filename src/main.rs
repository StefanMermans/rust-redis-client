use crate::{client::Client, resp::state_machine};

mod client;
mod resp;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

fn main() {
    let mut client = Client::connect(HOST, PORT);
    client.handshake();

    client.put_bulk_string("myKey", "myValue".into());


    return;
    for byte in b"-1" {
        println!("byte: {}", *byte);
    }

    let resp3 = "$11\r\nHello World".as_bytes().to_vec();

    let mut parser = state_machine::Parser::new(resp3);



    parser.parse();

return;

    // let encoded = encoder::encode_string("test value");
    //println!("{}", utils::to_debug_string(&encoded).unwrap());
    //let response = client.send_message("*4\r\n$5\r\nRPUSH\r\n$5\r\nmyKey\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".as_bytes());
    // let response_debug = utils::to_debug_string(&response).unwrap();
    //println!("response\n{}", &response_debug);
    let my_value = client.get_string("myKey").unwrap().unwrap();

    println!("{}", my_value);

    if let Some(other) = client.get_string("not_a_key").unwrap() {
        println!("Other: {}", other);
    } else {
        println!("Key not set");
    }

    client.put_bulk_string("My bulk string", "My bulk value!".into());
}


#[cfg(test)]
mod test;

