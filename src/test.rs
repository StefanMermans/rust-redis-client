use crate::client::Client;

#[test]
fn client_test() {
    let mut client = Client::connect("127.0.0.1", "6379");
    //client.handshake();
    let response = client.send_message_string("*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n".to_string());
    println!("response {}", response);
}

