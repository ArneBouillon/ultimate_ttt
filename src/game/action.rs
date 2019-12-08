use crate::game::game_state::GameState;
use crate::game::player::GameResult;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub sub_x: usize,
    pub sub_y: usize,
    pub x: usize,
    pub y: usize,
    pub full_board: bool,
}

impl Action {
    pub fn new(sub_x: usize, sub_y: usize, x: usize, y: usize, full_board: bool) -> Action {
        Action {
            sub_x,
            sub_y,
            x,
            y,
            full_board,
        }
    }

    pub fn apply(&self, game_state: &mut GameState) -> Option<GameResult> {
        game_state.make_move(self.sub_x, self.sub_y, self.x, self.y)
    }

    pub fn unapply(&self, game_state: &mut GameState) {

    }
}
