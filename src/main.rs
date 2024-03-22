mod messaging;
mod game_logic;
mod websocket;
mod controllers;

#[macro_use] extern crate rocket;

use controllers::game_controller::start_game_socket;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        start_game_socket,
    ])
}
