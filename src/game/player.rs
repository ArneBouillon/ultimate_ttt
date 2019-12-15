use crate::game::game_result::GameResult;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn next(self) -> Player {
        if self == Player::Player1 { Player::Player2 } else { Player::Player1 }
    }

    pub fn num(self) -> usize {
        if self == Player::Player1 { 1 } else { 2 }
    }

    pub fn wins(self) -> GameResult {
        if self == Player::Player1 { GameResult::Player1Wins } else { GameResult::Player2Wins }
    }
}
