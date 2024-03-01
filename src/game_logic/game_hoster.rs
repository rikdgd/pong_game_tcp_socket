use super::game_state::{PongGameState, PongBall};



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
        let mut is_running = true;
        while is_running {
            self.update_game_state();
            is_running = false;
        }
    }

    fn end_game(&self) {
        todo!()
    }

    /// Updates the state of the game to the next frame
    fn update_game_state(&mut self) {
        let player_1_x = 20;
        let player_2_x = 580;
        let screen_height = 400;
        
        let new_ball_x = self.game_state.ball.x + self.game_state.ball.speed_x;
        let new_ball_y = self.game_state.ball.y + self.game_state.ball.speed_y;
        let mut new_ball_x_speed = 0;
        let mut new_ball_y_speed = 0;
        
        if new_ball_x <= player_1_x || new_ball_x >= player_2_x {
            new_ball_x_speed = self.game_state.ball.speed_x * -1;
        } 
        
        if self.game_state.ball.y <= 0 || self.game_state.ball.y >= screen_height {
            new_ball_y_speed = self.game_state.ball.speed_y * -1;
        }
        
        
        self.game_state = PongGameState {
            is_running: self.game_state.is_running,
            player_1_position: self.game_state.player_1_position,
            player_2_position: self.game_state.player_2_position,
            ball: PongBall {
                x: new_ball_x,
                y: new_ball_y,
                speed_x: new_ball_x_speed,
                speed_y: new_ball_y_speed,
            },
            modifiers: self.game_state.modifiers.clone(),
        };
    }
}
impl PongGameHoster {
    pub fn _new(game_state: PongGameState) -> Self {
        PongGameHoster { game_state: game_state }
    }
}
