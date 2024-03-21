mod messaging;
mod game_logic;

use std::error::Error;
use messaging::requests::Request;

use std::net::TcpListener;
use std::thread;
use tungstenite::accept;
use tungstenite::protocol::Message;



fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in server.incoming() {
        thread::spawn(move || {
            let mut socket = accept(stream.unwrap()).unwrap();
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
