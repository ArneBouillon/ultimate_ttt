use std::io::stdin;

use crate::actor::Actor;
use crate::game::action::Action;
use crate::game::board::Owned;
use crate::game::game_result::GameResult;
use crate::game::game_state::GameState;

fn display(game_state: &mut GameState) -> String {
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

pub fn get_move(game_state: &mut GameState) -> Action {
    let mut sub_x = String::new();
    let mut sub_y = String::new();
    let mut x = String::new();
    let mut y = String::new();

    println!("Player {}'s move!", game_state.current_player.num());

    if game_state.current_sub_x.is_none() {
        stdin().read_line(&mut sub_x).unwrap();
        stdin().read_line(&mut sub_y).unwrap();
    } else {
        sub_x = game_state.current_sub_x.unwrap().to_string();
        sub_y = game_state.current_sub_y.unwrap().to_string();
    }
    stdin().read_line(&mut x).unwrap();
    stdin().read_line(&mut y).unwrap();

    Action::new(
        sub_x.replace("\n", "").parse::<usize>().unwrap(),
        sub_y.replace("\n", "").parse::<usize>().unwrap(),
        x.replace("\n", "").parse::<usize>().unwrap(),
        y.replace("\n", "").parse::<usize>().unwrap(),
        game_state.current_sub_x == None,
    )
}

pub struct Human {}

impl Actor for Human {
    fn get_action(&self, game_state: &mut GameState) -> Action {
        get_move(game_state)
    }
}

pub fn play(player1: &mut dyn Actor, player2: &mut dyn Actor) -> GameResult {
    let mut game_state = GameState::new();

    loop {
        println!("{}", display(&mut game_state));

        let action = player1.get_action(&mut game_state);
        if let Some(result) = action.apply(&mut game_state) {
            println!("{}", display(&mut game_state));
            println!("Result: {}", result.to_string());
            return result;
        }

        println!("{}", display(&mut game_state));

        let action = player2.get_action(&mut game_state);
        if let Some(result) = action.apply(&mut game_state) {
            println!("{}", display(&mut game_state));
            println!("Result: {}", result.to_string());
            return result;
        }
    }
}
