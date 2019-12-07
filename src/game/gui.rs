use crate::game::game_state::GameState;

pub struct GUI {
    game_state: GameState,
}

impl GUI {
    pub fn new() -> GUI {
        GUI { game_state: GameState::new() }
    }

    pub fn display(&self) -> String {
        let string: String = "".to_string();

        



        string
    }
}
