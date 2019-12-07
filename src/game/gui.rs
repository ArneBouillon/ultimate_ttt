use crate::game::game_state::GameState;
use crate::game::board::Owned;
use crate::game::player::Player::{Player1, Player2};
use std::io::stdin;

pub struct GUI {
    game_state: GameState,
}

impl GUI {
    pub fn new() -> GUI {
        GUI { game_state: GameState::new() }
    }

    pub fn game_state(&mut self) -> &mut GameState { &mut self.game_state }

    pub fn play(mut self) {
        loop {
            println!("{}", self.display());

            let (sub_x, sub_y, x, y) = self.get_move();

            self.game_state().make_move_full_board(sub_x, sub_y, x, y)
        }
    }

    pub fn display(&mut self) -> String {
        let current_sub_x = self.game_state.current_sub_x;
        let current_sub_y = self.game_state.current_sub_y;

        let mut string: String = String::new();

        for major_y in 0..3 {
            string += "\n";

            for minor_y in 0..3 {
                for major_x in 0..3 {
                    string +=  if current_sub_y == Some(major_y) &&
                        current_sub_x == Some(major_x) { "|" } else { " " };

                    for minor_x in 0..3 {
                        string += "|";

                        let player = self.game_state
                            .board
                            .structure()
                            .get_mut(major_x, major_y)
                            .structure()
                            .get_mut(minor_x, minor_y)
                            .owner();

                        string += if player == Some(Player1) { "O" } else if player == Some(Player2) { "X" } else { " " }
                    }

                    string += "|";
                    string += if current_sub_y == Some(major_y) &&
                        current_sub_x == Some(major_x) { "|" } else { " " }
                }

                string += "\n"
            }
        }

        string
    }

    pub fn get_move(&mut self) -> (usize, usize, usize, usize) {
        let mut sub_x = String::new();
        let mut sub_y = String::new();
        let mut x = String::new();
        let mut y = String::new();

        println!("Player {}'s move!", self.game_state.current_player.num());

        stdin().read_line(&mut sub_x).unwrap();
        stdin().read_line(&mut sub_y).unwrap();
        stdin().read_line(&mut x).unwrap();
        stdin().read_line(&mut y).unwrap();

        return (
            sub_x.replace("\n", "").parse::<usize>().unwrap(),
            sub_y.replace("\n", "").parse::<usize>().unwrap(),
            x.replace("\n", "").parse::<usize>().unwrap(),
            y.replace("\n", "").parse::<usize>().unwrap(),
        )
    }
}
