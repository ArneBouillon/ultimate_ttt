use super::board::Board;
use super::player::Player;

pub struct GameState {
    board: Board,
    current_player: Player,
    current_sub_x: Option<usize>,
    current_sub_y: Option<usize>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: Board::new(),
            current_player: Player::Player1,
            current_sub_x: None,
            current_sub_y: None,
        }
    }

    fn current_player(&self) -> Player {
        self.current_player.clone()
    }

    fn make_move(mut self, x: usize, y: usize) {
        let (sub_x, sub_y) = (self.current_sub_x.unwrap(), self.current_sub_y.unwrap());

        self.make_move_full_board(sub_x, sub_y, x, y);
    }

    fn make_move_full_board(mut self, sub_x: usize, sub_y: usize, x: usize, y: usize) {
        let current_player = self.current_player();

        let (new_x, new_y) = self.board.make_move(current_player,
                                                  sub_x,
                                                  sub_y,
                                                  x.clone(),
                                                  y.clone());

        self.current_sub_x = new_x;
        self.current_sub_y = new_y;

        self.current_player = self.current_player.next();
    }
}
