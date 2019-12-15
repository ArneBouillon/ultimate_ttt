use ultimate_ttt::ai::mcts::MCTSActor;
use ultimate_ttt::gui;

fn main() {
    gui::play(&mut MCTSActor::new(1000), &mut gui::Human{});
}
