use ultimate_ttt::gui::{GUI, Human};
use ultimate_ttt::ai::mcts::MCTSActor;

fn main() {
    let mut gui = GUI::new();
    gui.play(&mut Human{}, &mut MCTSActor::new(10000));
}
