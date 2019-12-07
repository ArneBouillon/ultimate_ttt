use crate::game::game_state::GameState;
use crate::game::board::Owned;
use crate::game::player::Player;
use std::io::stdin;

pub struct GUI {
    game_state: GameState,
}

impl GUI {
    pub fn new() -> GUI {
        GUI { game_state: GameState::new() }
    }

    pub fn game_state(&mut self) -> &mut GameState { &mut self.game_state }

    pub fn play_pvp(mut self) -> Option<Player> {
        loop {
            println!("{}", self.display());

            loop {
                let (sub_x, sub_y, x, y) = self.get_move();
                let current_sub_x = self.game_state.current_sub_x;
                let current_sub_y = self.game_state.current_sub_y;

                if current_sub_x == None || Some(sub_x) == current_sub_x && Some(sub_y) == current_sub_y {
                    match self.game_state().make_move(sub_x, sub_y, x, y) {
                        None => {},
                        Some(player) => return Some(player)
                    }

                    break
                }

                println!("\nInvalid input!\n")
            }
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
                            .get(major_x, major_y)
                            .structure()
                            .get(minor_x, minor_y)
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
