use crate::game::player::Player::Player1;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn next(self) -> Player {
        if self == Player1 { Player::Player2 } else { Player1 }
    }
}
