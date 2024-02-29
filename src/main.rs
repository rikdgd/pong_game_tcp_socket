mod messaging;
mod game_logic;

use std::{error::Error, io::Read, net};

use messaging::request_types::{TcpMessage, Request};



fn main() -> Result<(), Box<dyn Error>> {
    let listener = net::TcpListener::bind("127.0.0.1:8080")?;
    let mut running = true;
    
    while running {
        let (mut stream, addr) = listener.accept()?;
        let mut content_buffer: Vec<u8> = Vec::new();
        stream.read_to_end(&mut content_buffer)?;
        let message_content = String::from_utf8(content_buffer.clone())?;
        
        let received_request = TcpMessage {
            sender: addr,
            message: message_content,
        };
        
        handle_message(received_request).unwrap_or_else(|op| {
            println!("{:#?}", op);
            running = false;
        });
    }
    
    return Ok(());
}

fn handle_message(message: TcpMessage) -> Result<(), Box<dyn Error>> {
    let request = Request::from_tcp_message(message)?;
    
    println!("{:#?}", request);
    
    return Ok(());
}

