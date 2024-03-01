use super::game_state;



pub trait GameHoster {
    fn start_game();
    fn end_game();
    fn get_next_frame();
}

pub struct PongGameHoster {
    pub game_state: game_state::PongGameState,
}
impl GameHoster for PongGameHoster {
    fn start_game() {
        todo!()
    }

    fn end_game() {
        todo!()
    }

    fn get_next_frame() {
        todo!()
    }
}
impl PongGameHoster {
    pub fn new(game_state: game_state::PongGameState) -> Self {
        PongGameHoster { game_state: game_state }
    }
}
