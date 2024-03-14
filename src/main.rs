mod messaging;
mod game_logic;

use std::{error::Error, io::Read};
use messaging::requests::{TcpMessage, Request};

use std::net::TcpListener;
use std::thread;
use tungstenite::{accept, WebSocket};
use tungstenite::protocol::Message;



fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind("127.0.0.1:8080")?;
    let mut running = true;
    
    for stream in server.incoming() {
        thread::spawn(move || {
            let mut socket = accept(stream.unwrap()).unwrap();
            loop {
                let message = socket.read().unwrap();
                
                if message.is_binary() || message.is_text() {
                    parse_message(message);
                }
            }
        });
    }
    
    Ok(())
}

fn handle_message(message: TcpMessage) -> Result<(), Box<dyn Error>> {
    let request = Request::from_tcp_message(message)?;
    
    println!("{:#?}", request);
    
    Ok(())
}

// TODO: Should replace the handle_message function since the new type of 'Message' is used.
fn parse_message(message: Message) -> String {
    todo!()
}

