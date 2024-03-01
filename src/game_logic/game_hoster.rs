use super::game_state::{PongGameState, GameModifiers, PongBall};



pub trait GameHoster {
    fn start_game(&mut self);
    fn end_game(&self);
    fn update_game_state(&mut self);
}

pub struct PongGameHoster {
    pub game_state: PongGameState,
}
impl GameHoster for PongGameHoster {
    fn start_game(&mut self) {
        loop {
            self.update_game_state();
            break;
        }
    }

    fn end_game(&self) {
        todo!()
    }

    fn update_game_state(&mut self) {
        let player_1_x = 20;
        let player_2_x = 580;
        let screen_height = 400;
        
        let new_ball_x = self.game_state.ball.x + self.game_state.ball.speed_x;
        let new_ball_y = self.game_state.ball.y + self.game_state.ball.speed_y;
        
        if new_ball_x <= player_1_x || new_ball_x >= player_2_x {
            self.game_state.ball.speed_x = self.game_state.ball.speed_x * -1;
        } 
        
        if self.game_state.ball.y <= 0 || self.game_state.ball.y >= screen_height {
            self.game_state.ball.speed_y = self.game_state.ball.speed_y * -1;
        }
        
        self.game_state = PongGameState {
            player_1_position: self.game_state.player_1_position,
            player_2_position: self.game_state.player_2_position,
            ball: PongBall {
                x: self.game_state.ball.x + self.game_state.ball.speed_x,
                y: self.game_state.ball.y + self.game_state.ball.speed_y,
                speed_x: self.game_state.ball.speed_x,
                speed_y: self.game_state.ball.speed_y,
            },
            modifiers: self.game_state.modifiers.clone(),
        };
    }
}
impl PongGameHoster {
    pub fn new(game_state: PongGameState) -> Self {
        PongGameHoster { game_state: game_state }
    }
}
