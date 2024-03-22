use std::error::Error;
use std::net::TcpListener;
use std::thread;
use tungstenite::accept;
use tungstenite::protocol::Message;

use crate::messaging::requests::Request;




pub fn start_websocket_server() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in server.incoming() {
        thread::spawn(move || {
            let mut socket = accept(stream.unwrap()).expect("Unable to create socket.");
            loop {
                let message = socket.read().expect("Failed to read from socket.");
                
                if message.is_binary() || message.is_text() {
                    handle_message(message).expect("Failed to handle incomming message.");
                }
            }
        });
    }
    
    Ok(())
}

fn handle_message(message: Message) -> Result<(), Box<dyn Error>> {
    let request = Request::from_message(message)?;
    
    println!("{:#?}", request);
    
    Ok(())
}
