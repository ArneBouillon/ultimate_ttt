use crate::game::player::Player;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameResult {
    Player1Wins,
    Player2Wins,
    Draw,
}

impl GameResult {
    pub fn to_string(self) -> String {
        match self {
            GameResult::Player1Wins => "Player 1 wins!".to_string(),
            GameResult::Player2Wins => "Player 2 wins!".to_string(),
            GameResult::Draw        => "Draw!".to_string(),
        }
    }

    pub fn score(self, player: Player) -> f32 {
        match (self, player) {
            (GameResult::Player1Wins, Player::Player1) => 1.,
            (GameResult::Player1Wins, Player::Player2) => 0.,
            (GameResult::Player2Wins, Player::Player1) => 0.,
            (GameResult::Player2Wins, Player::Player2) => 1.,
            (GameResult::Draw,        _              ) => 0.5,
        }
    }
}
