use super::board::{Board, Owned};
use super::player::Player;
use super::action::Action;
use crate::game::player::GameResult;

use rand::seq::SliceRandom;

#[derive(Clone)]
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
        self.current_player
    }

    pub fn make_move(&mut self, sub_x: usize, sub_y: usize, x: usize, y: usize) -> Option<GameResult> {
        let current_player = self.current_player();

        let (new_x, new_y, result) = self.board_mut().make_move(Some(current_player.wins()),
                                                                sub_x,
                                                                sub_y,
                                                                x.clone(),
                                                                y.clone());

        self.current_sub_x = new_x;
        self.current_sub_y = new_y;
        self.current_player = self.current_player().next();

        result
    }

    pub fn play_randomly(&mut self) -> GameResult {
        let actions = self.possible_actions();
        let action = actions.choose(&mut rand::thread_rng()).unwrap();

        let action_result = action.apply(self);
        let result = match action_result {
            None => self.play_randomly(),
            Some(action_result) => action_result,
        };
        action.unapply(self);

        result
    }

    pub fn possible_actions(&self) -> Vec<Action> {
        match self.current_sub_x {
            Some(_) => {
                let (sub_x, sub_y) = (self.current_sub_x.unwrap(), self.current_sub_y.unwrap());
                let sub_board_items = self.board().get(sub_x, sub_y).structure().items;
                let mut vec = Vec::with_capacity(9);

                for i in 0..9 {
                    if sub_board_items[i].result().is_none() {
                        vec.push(Action::new(sub_x, sub_y, i % 3, i / 3, false));
                    }
                }

                vec
            },
            None => {
                let mut vec = Vec::with_capacity(81);

                for i in 0..9 {
                    let sub_board = self.board().structure().items[i];

                    if !self.board().structure().items[i].result().is_none() {
                        continue;
                    }

                    let (sub_x, sub_y) = (i % 3, i / 3);
                    for j in 0..9 {
                        if sub_board.structure().items[j].result().is_none() {
                            vec.push(Action::new(sub_x, sub_y, j % 3, j / 3, true));
                        }
                    }
                }

                vec
            },
        }
    }
}
