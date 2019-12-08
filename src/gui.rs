use crate::game::game_state::GameState;
use crate::game::board::Owned;
use crate::game::player::{Player, GameResult};
use std::io::stdin;
use crate::game::action::Action;

pub struct GUI {
    game_state: GameState,
}

impl GUI {
    pub fn new() -> GUI {
        GUI { game_state: GameState::new() }
    }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn game_state_mut(&mut self) -> &mut GameState { &mut self.game_state }

    pub fn play_pvp(mut self) -> Option<GameResult> {
        loop {
            println!("{}", self.display());

            loop {
                let action = self.get_move();
                let current_sub_x = self.game_state.current_sub_x;
                let current_sub_y = self.game_state.current_sub_y;

                if current_sub_x == None || Some(action.sub_x) == current_sub_x && Some(action.sub_y) == current_sub_y {
                    match action.apply(self.game_state_mut()) {
                        None => {},
                        Some(result) => {
                            println!("Result: {}", result.to_string());
                            return Some(result);
                        }
                    }

                    break;
                }

                println!("\nInvalid input!\n");
            }
        }
    }

    pub fn display(&mut self) -> String {
        let current_sub_x = self.game_state.current_sub_x;
        let current_sub_y = self.game_state.current_sub_y;

        let mut string: String = String::new();

        for sub_y in 0..3 {
            string += "\n";

            for y in 0..3 {
                for sub_x in 0..3 {
                    string +=  if current_sub_y == Some(sub_y) &&
                        current_sub_x == Some(sub_x) { "|" } else { " " };

                    for x in 0..3 {
                        string += "|";

                        let player = self.game_state
                            .board
                            .structure()
                            .get(sub_x, sub_y)
                            .structure()
                            .get(x, y)
                            .owner();

                        string += if player == Some(Player::Player1) {
                                    "O"
                                  } else if player == Some(Player::Player2) {
                                    "X"
                                  } else {
                                    " "
                                  }
                    }

                    string += "|";
                    string += if current_sub_y == Some(sub_y) &&
                        current_sub_x == Some(sub_x) { "|" } else { " " }
                }

                string += "\n"
            }
        }

        string
    }

    pub fn get_move(&mut self) -> Action {
        let mut sub_x = String::new();
        let mut sub_y = String::new();
        let mut x = String::new();
        let mut y = String::new();

        println!("Player {}'s move!", self.game_state.current_player.num());

        stdin().read_line(&mut sub_x).unwrap();
        stdin().read_line(&mut sub_y).unwrap();
        stdin().read_line(&mut x).unwrap();
        stdin().read_line(&mut y).unwrap();

        return Action::new(
            sub_x.replace("\n", "").parse::<usize>().unwrap(),
            sub_y.replace("\n", "").parse::<usize>().unwrap(),
            x.replace("\n", "").parse::<usize>().unwrap(),
            y.replace("\n", "").parse::<usize>().unwrap(),
            self.game_state().current_sub_x == None,
        )
    }
}