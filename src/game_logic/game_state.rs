use serde::{Serialize, Deserialize, Deserializer};



#[derive(Serialize, Deserialize, Debug)]
pub struct PongGameState {
    pub player_1_position: u16,
    pub player_2_position: u16,
    pub ball_x: PongBall,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PongBall {
    pub x: u16,
    pub y: u16,
    pub speed_x: u16,
    pub speed_y: u16,
}
