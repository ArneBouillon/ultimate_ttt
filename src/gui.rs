use crate::game::game_state::GameState;
use crate::game::board::Owned;
use crate::game::player::GameResult;
use std::io::stdin;
use crate::game::action::Action;
use crate::ai::mcts::mcts;

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

    pub fn play_pvp(&mut self) -> GameResult {
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
                            return result;
                        }
                    }

                    break;
                }

                println!("\nInvalid input!\n");
            }
        }
    }

    pub fn play_pvc(&mut self) -> GameResult {
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
                            return result;
                        }
                    }

                    break;
                }

                println!("\nInvalid input!\n");
            }

            println!("{}", self.display());

            let ai_action = mcts(self.game_state_mut(), 4);
            match ai_action.apply(self.game_state_mut()) {
                None => {},
                Some(result) => {
                    println!("Result: {}", result.to_string());
                    return result;
                }
            }
        }
    }

    pub fn display(&mut self) -> String {
        GUI::display_static(self.game_state_mut())
    }

    pub fn display_static(game_state: &mut GameState) -> String {
        let current_sub_x = game_state.current_sub_x;
        let current_sub_y = game_state.current_sub_y;

        let mut string: String = String::new();

        for sub_y in 0..3 {
            string += "\n";

            for y in 0..3 {
                for sub_x in 0..3 {
                    string +=  if current_sub_y == Some(sub_y) &&
                        current_sub_x == Some(sub_x) { "|" } else { " " };

                    for x in 0..3 {
                        string += "|";

                        let result = game_state
                            .board
                            .structure()
                            .get(sub_x, sub_y)
                            .structure()
                            .get(x, y)
                            .result();

                        string += if result == Some(GameResult::Player1Wins) {
                            "O"
                        } else if result == Some(GameResult::Player2Wins) {
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
