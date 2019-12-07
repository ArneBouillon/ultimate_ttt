#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn next(self) -> Player {
        if self == Player::Player1 { Player::Player2 } else { Player::Player1 }
    }

    pub fn num(self) -> usize { if self == Player::Player1 { 1 } else { 2 } }
}
