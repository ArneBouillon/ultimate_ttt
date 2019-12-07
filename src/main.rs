use ultimate_ttt::gui::GUI;
use ultimate_ttt::ai::mcts::{Node, mcts};
use ultimate_ttt::game::action::Action;

fn main() {
    let gui = GUI::new();
    mcts();

    gui.play_pvp();
}
