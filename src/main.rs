use ultimate_ttt::gui::{GUI, Human};
use ultimate_ttt::ai::mcts::MCTSActor;

fn main() {
    let mut gui = GUI::new();
    gui.play(&mut MCTSActor::new(1000), &mut MCTSActor::new(1000));
}
