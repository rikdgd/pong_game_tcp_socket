use crate::messaging::requests::Request;
use serde::{Serialize, Deserialize};
use serde_json;



pub trait GameAction {
    fn execute(&self);
    fn from_request(request: Request) -> Self;
}

pub struct StartGameAction {
    pub player_1_id: u32,
    pub player_2_id: u32,
    pub modifiers: String,
}
impl GameAction for StartGameAction {
    fn execute(&self) {
        todo!()
    }

    fn from_request(request: Request) -> Self {
        if request.action != "start_game" {
            panic!("Invalid request for StartGameAction");
        }
        
        todo!()
    }
}


pub struct EndGameAction {
    pub player_1_id: u32,
    pub player_2_id: u32,
    pub winner_id: u32,
}
impl GameAction for EndGameAction {
    fn execute(&self) {
        todo!()
    }

    fn from_request(request: Request) -> Self {
        if request.action != "end_game" {
            panic!("Invalid request for StartGameAction");
        }
        
        todo!()
    }
}
