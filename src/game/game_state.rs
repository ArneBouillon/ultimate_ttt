use super::board::{Board, Owned};
use super::player::Player;
use super::action::Action;
use crate::game::player::GameResult;

use rand::seq::SliceRandom;
use crate::gui::GUI;

pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub current_sub_x: Option<usize>,
    pub current_sub_y: Option<usize>,
    pub moves: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: Board::new(),
            current_player: Player::Player1,
            current_sub_x: None,
            current_sub_y: None,
            moves: 0,
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
        self.moves += 1;

        if self.moves == 81 && result == None {
            Some(GameResult::Draw)
        } else {
            result
        }
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
                self.possible_actions_sub(sub_x, sub_y, false)
            },
            None => {
                (0..3).flat_map(|sub_x| {
                    let v: Vec<Action> = (0..3).flat_map(|sub_y|
                        if self.board().get(sub_x, sub_y).result() == None {
                            self.possible_actions_sub(sub_x, sub_y, true)
                        } else {
                            Vec::new()
                        }
                    ).collect();

                    v
                }).collect()
            },
        }
    }

    fn possible_actions_sub(&self, sub_x: usize, sub_y: usize, full_board: bool) -> Vec<Action> {
        let sub_board = self.board()
            .structure()
            .get(sub_x, sub_y);

        (0..3).flat_map(|x| {
            let v: Vec<Action> = (0..3).filter_map(|y|
                match sub_board.get(x, y).result() {
                    None => Some(Action::new(sub_x, sub_y, x, y, full_board)),
                    Some(_) => None,
                }
            ).collect();

            v
        }).collect()
    }
}
