use crate::game::action::Action;
use crate::game::game_state::GameState;

pub trait Actor {
    fn get_action(&self, game_state: &mut GameState) -> Action;
}
