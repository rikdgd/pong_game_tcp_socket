use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct PongGameState {
    pub is_running: bool,
    pub player_1_position: u16,
    pub player_2_position: u16,
    pub ball: PongBall,
    pub modifiers: GameModifiers,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PongBall {
    pub x: i32,
    pub y: i32,
    pub speed_x: i32,
    pub speed_y: i32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameModifiers {
    None, 
    IncrementSpeed,
    EnableGravity,
    MultipleBalls,
    DecreasingBat,
    FieldRotate,
}
