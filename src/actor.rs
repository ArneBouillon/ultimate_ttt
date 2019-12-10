use crate::game::game_state::GameState;
use crate::game::action::Action;

pub trait Actor {
    fn get_action(&self, game_state: &mut GameState) -> Action;
}
