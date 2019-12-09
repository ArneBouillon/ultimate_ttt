use ultimate_ttt::gui::GUI;
use ultimate_ttt::ai::mcts::mcts;

fn main() {
    let mut gui = GUI::new();

    gui.play_pvc();
}
