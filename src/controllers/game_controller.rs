use crate::websocket::start_websocket_server;


#[get("/game/start_game")]
pub fn start_game_socket() -> String {
    let mut return_message = String::from("succes");
    
    start_websocket_server().unwrap_or_else(|op| {
        return_message = String::from("Failed to start websocket server");
        println!("Error when starting websocket server: {}", op);
    });
    
    return_message
}
