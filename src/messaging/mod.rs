use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use serde::{Serialize, Deserialize};
use serde_json;



#[derive(Serialize, Deserialize, Debug)]
pub struct TcpMessage {
    pub sender: SocketAddr,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub action: String,
    pub request_type: RequestType,
    pub data: String,
}
impl Request {
    pub fn from_tcp_message(tcp_message: TcpMessage) -> Result<Self, Box<dyn Error>> {
        let parsed_message: Request = serde_json::from_str(&tcp_message.message)?;
        Ok(parsed_message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    Command,
    StateUpdate,
    Error,
}



#[test]
fn parse_tpc_message_test() {
    // Arrange
    let action = "get_game_state";
    let data = r#"{"user": "Bob Ross"}"#; // Using raw string for JSON data
    let data_escaped = data.replace("\"", "\\\"");
    let message = format!("{{\"action\": \"{}\", \"data\": \"{}\", \"request_type\": \"Command\"}}", action, data_escaped);

    let tcp_message = TcpMessage {
        sender: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        message: message.clone(),
    };

    //Act
    println!("message: {}", &message);
    let parsed_request = Request::from_tcp_message(tcp_message).unwrap();

    // Assert
    assert_eq!(parsed_request.action, action);
    assert!(matches!(parsed_request.request_type, RequestType::Command));
    assert_eq!(parsed_request.data, data);
}

