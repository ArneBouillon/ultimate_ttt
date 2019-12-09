use crate::game::game_state::GameState;
use crate::game::player::GameResult;
use crate::game::board::Owned;

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
        game_state.board_mut().make_move(None, self.sub_x, self.sub_y, self.x, self.y);

        if self.full_board {
            game_state.current_sub_x = None;
            game_state.current_sub_y = None;
        } else {
            game_state.current_sub_x = Some(self.sub_x);
            game_state.current_sub_y = Some(self.sub_y);
        };

        game_state.board_mut().get_mut(self.sub_x, self.sub_y).set_result(None);

        game_state.moves -= 1;
    }
}
