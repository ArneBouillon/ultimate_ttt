use ultimate_ttt::gui::GUI;
use ultimate_ttt::ai::mcts::mcts;

fn main() {
    let gui = GUI::new();
    mcts();

    gui.play_pvp();
}
