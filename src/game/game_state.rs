use super::board::{Board, Owned};
use super::player::Player;
use super::action::Action;

pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub current_sub_x: Option<usize>,
    pub current_sub_y: Option<usize>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: Board::new(),
            current_player: Player::Player1,
            current_sub_x: None,
            current_sub_y: None,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player.clone()
    }

    pub fn make_move(&mut self, sub_x: usize, sub_y: usize, x: usize, y: usize) -> Option<Player> {
        let current_player = self.current_player();

        let (new_x, new_y, winning_player) = self.board_mut().make_move(current_player,
                                                                       sub_x,
                                                                       sub_y,
                                                                       x.clone(),
                                                                       y.clone());

        self.current_sub_x = new_x;
        self.current_sub_y = new_y;
        self.current_player = self.current_player.next();

        winning_player
    }

    pub fn possible_actions(&self) -> Vec<Action> {
        match self.current_sub_x {
            Some(_) => {
                let (sub_x, sub_y) = (self.current_sub_x.unwrap(), self.current_sub_y.unwrap());
                self.possible_actions_sub(sub_x, sub_y)
            },
            None => {
                (0..3).flat_map(|sub_x| {
                    let v: Vec<Action> = (0..3).flat_map(|sub_y|
                        self.possible_actions_sub(sub_x, sub_y)
                    ).collect();

                    v
                }).collect()
            },
        }
    }

    fn possible_actions_sub(&self, sub_x: usize, sub_y: usize) -> Vec<Action> {
        let sub_board = self.board()
            .structure()
            .get(sub_x, sub_y);

        (0..3).flat_map(|x| {
            let v: Vec<Action> = (0..3).filter_map(|y|
                match sub_board.structure().get(x, y).owner() {
                    None => Some(Action::new(sub_x, sub_y, x, y)),
                    Some(_) => None,
                }
            ).collect();

            v
        }).collect()
    }
}
