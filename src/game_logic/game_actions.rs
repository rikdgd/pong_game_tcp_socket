use std::collections::HashMap;
use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::messaging::requests::{Request, RequestType};



#[derive(Serialize, Deserialize, Debug)]
pub struct StartGameActionData {
    pub player_1_id: u32,
    pub player_2_id: u32,
    pub modifiers: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndGameActionData {
    pub player_1_id: u32,
    pub player_2_id: u32,
    pub winner_id: u32,
}


pub enum GameAction {
    StartGame(StartGameActionData),
    EndGame(EndGameActionData),
}

impl GameAction {
    pub fn from_request(request: Request) -> Result<GameAction, Box<dyn Error>> {
        if let RequestType::GameAction = request.request_type {
            let action = &request.action;
            
            let game_action: Result<GameAction, Box<dyn Error>> = match action.as_str() {
                "start_game" => {
                    let start_game_action_data: StartGameActionData = serde_json::from_str(&request.data)?;
                    Ok(GameAction::StartGame(start_game_action_data))
                },
                "end_game" => {
                    let end_game_action_data: EndGameActionData = serde_json::from_str(&request.data)?;
                    Ok(GameAction::EndGame(end_game_action_data))
                },
                _ => {
                    panic!("Tried to parse GameAction from request with wrong action")
                }
            };
           
           return game_action;
        }
        
        panic!("Tried to parse GameAction from request with wrong request_type");
    }
    
    pub fn execute(self) {
        todo!()
    }
}
