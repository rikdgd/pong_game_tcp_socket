use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use serde::{Serialize, Deserialize};
use serde_json;
use tungstenite::Message;



#[derive(Serialize, Deserialize, Debug)]
pub struct TcpMessage {
    pub sender: SocketAddr,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub request_type: RequestType,
    pub action: String,
    pub data: String,
    pub sender_id: u32,
}
impl Request {
    pub fn from_tcp_message(tcp_message: TcpMessage) -> Result<Self, Box<dyn Error>> {
        let parsed_message: Request = serde_json::from_str(&tcp_message.data)?;
        Ok(parsed_message)
    }
    
    pub fn from_message(message: Message) -> Result<Self, Box<dyn Error>> {
        let data = match message {
            Message::Text(text) => text,
            Message::Binary(binary) => String::from_utf8_lossy(&binary).to_string(),
            _ => String::from(""),
        };
        
        let parsed_message: Request = serde_json::from_str(&data)?;
        Ok(parsed_message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    Command,
    GameAction,
    StateUpdate,
    Error,
}



#[test]
fn parse_tpc_message_test() {
    // Arrange
    let action = "get_game_state";
    let sender_id: u32 = 12;
    let data = r#"{"user": "Bob Ross"}"#; // Using raw string for JSON data
    let data_escaped = data.replace("\"", "\\\"");
    let message = format!(
        "{{
            \"action\": \"{}\", 
            \"data\": \"{}\", 
            \"request_type\": \"Command\", 
            \"sender_id\": {}
        }}"
            , action, data_escaped, sender_id);

    let tcp_message = TcpMessage {
        sender: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        data: message.clone(),
    };

    //Act
    println!("message: {}", &message);
    let parsed_request = Request::from_tcp_message(tcp_message).expect("failed to parse TCP message.");

    // Assert
    assert_eq!(parsed_request.action, action);
    assert!(matches!(parsed_request.request_type, RequestType::Command));
    assert_eq!(parsed_request.data, data);
}

