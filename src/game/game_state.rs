use super::board::{Board, Owned};
use super::player::Player;
use super::action::Action;
use crate::game::game_result::GameResult;

use rand::seq::SliceRandom;
use rand::Rng;

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
        self.current_player = current_player.next();

        result
    }

    pub fn play_randomly(&self) -> GameResult {
        let mut actions = self.initialize_actions();
        let mut new_game_state = self.clone();

        loop {
            let action = match new_game_state.current_sub_x {
                None => {
                    let possible_actions = new_game_state.possible_actions();
                    let action = possible_actions.choose(&mut rand::thread_rng()).unwrap();
                    actions[3 * action.sub_y + action.sub_x].retain(|ac| ac.x != action.x || ac.y != action.y);
                    action.clone()
                },
                Some(sub_x) => {
                    let sub_y = new_game_state.current_sub_y.unwrap();
                    let random_num = rand::thread_rng().gen_range(0, actions[3 * sub_y + sub_x].len());
                    actions[3 * sub_y + sub_x].remove(random_num).clone()
                },
            };

            let action_result = action.apply(&mut new_game_state);
            if let Some(result) = action_result {
                return result;
            }
        }
    }

    pub fn initialize_actions(&self) -> [Vec<Action>; 9] {
        [
            (0..9).filter_map(|i| {
                match self.board.structure().items[0].structure().items[i].result() {
                    None => Some(Action::new(
                        0,
                        0,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[1].structure().items[i].result() {
                    None => Some(Action::new(
                        1,
                        0,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[2].structure().items[i].result() {
                    None => Some(Action::new(
                        2,
                        0,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[3].structure().items[i].result() {
                    None => Some(Action::new(
                        0,
                        1,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[4].structure().items[i].result() {
                    None => Some(Action::new(
                        1,
                        1,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[5].structure().items[i].result() {
                    None => Some(Action::new(
                        2,
                        1,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[6].structure().items[i].result() {
                    None => Some(Action::new(
                        0,
                        2,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[7].structure().items[i].result() {
                    None => Some(Action::new(
                        1,
                        2,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
            (0..9).filter_map(|i| {
                match self.board.structure().items[8].structure().items[i].result() {
                    None => Some(Action::new(
                        2,
                        2,
                        i % 3,
                        i / 3,
                        false,
                    )),
                    Some(_) => None,
                }
            }).collect(),
        ]
    }

    pub fn possible_actions(&self) -> Vec<Action> {
        match self.current_sub_x {
            Some(sub_x) => {
                let sub_y = self.current_sub_y.unwrap();
                let sub_board_items = &self.board().get(sub_x, sub_y).structure().items;
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

                let board_items = &self.board().structure().items;
                for i in 0..9 {
                    let sub_board = &board_items[i];

                    if sub_board.result().is_some() {
                        continue;
                    }

                    let sub_board_items = &sub_board.structure().items;
                    let (sub_x, sub_y) = (i % 3, i / 3);
                    for j in 0..9 {
                        if sub_board_items[j].result().is_none() {
                            vec.push(Action::new(sub_x, sub_y, j % 3, j / 3, true));
                        }
                    }
                }

                vec
            },
        }
    }
}
