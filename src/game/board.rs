use crate::game::player::GameResult;

pub trait Owned {
    fn result(&self) -> Option<GameResult>;

    fn set_result(&mut self, owner: Option<GameResult>);
}

pub struct Board {
    pub structure: BoardStructure<SubBoard>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            structure: <BoardStructure<SubBoard>>::new(),
        }
    }

    pub fn structure(&self) -> &BoardStructure<SubBoard> {
        &self.structure
    }

    pub fn structure_mut(&mut self) -> &mut BoardStructure<SubBoard> {
        &mut self.structure
    }

    pub fn get(&self, x: usize, y: usize) -> &SubBoard {
        self.structure().get(x, y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut SubBoard {
        self.structure_mut().get_mut(x, y)
    }

    pub fn make_move(&mut self, result: Option<GameResult>, sub_x: usize, sub_y: usize, x: usize, y: usize) -> (Option<usize>, Option<usize>, Option<GameResult>) {
        self.get_mut(sub_x, sub_y)
            .structure_mut()
            .set_result_at(x.clone(), y.clone(), result);

        let mut game_result: Option<GameResult> = None;

        if result != None {
            match self.get_mut(sub_x, sub_y).structure().check_result(x, y) {
                None => {},
                Some(result) => {
                    self.get_mut(sub_x, sub_y).set_result(Some(result));

                    match self.structure().check_result(sub_x, sub_y) {
                        None => {},
                        Some(result) => game_result = Some(result),
                    }
                }
            }
        }
        match self.get(x, y).result() {
            None => (Some(x), Some(y), game_result),
            Some(_) => (None, None, game_result),
        }
    }
}

#[derive(Copy, Clone)]
pub struct SubBoard {
    pub structure: BoardStructure<Square>,
    result: Option<GameResult>,
}

impl SubBoard {
    pub fn new() -> SubBoard {
        SubBoard { structure: <BoardStructure<Square>>::new(), result: None }
    }

    pub fn structure(&self) -> &BoardStructure<Square> {
        &self.structure
    }

    pub fn structure_mut(&mut self) -> &mut BoardStructure<Square> {
        &mut self.structure
    }

    pub fn get(&self, x: usize, y: usize) -> &Square {
        self.structure().get(x, y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Square {
        self.structure_mut().get_mut(x, y)
    }
}

impl Owned for SubBoard {
    fn result(&self) -> Option<GameResult> {
        self.result
    }

    fn set_result(&mut self, result: Option<GameResult>) {
        self.result = result;
    }
}

#[derive(Copy, Clone)]
pub struct BoardStructure<T: Owned> {
    pub items: [T; 9],
}

impl<T: Owned> BoardStructure<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.items[3 * y + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.items[3 * y + x]
    }

    pub fn set_result_at(&mut self, x: usize, y: usize, result: Option<GameResult>) {
        self.items.get_mut(3 * y + x).unwrap().set_result(result);
    }

    pub fn check_result(&self, last_x: usize, last_y: usize) -> Option<GameResult> {
        if ((last_x == 1) as usize + (last_y == 1) as usize) != 1 {
            if last_x == last_y &&
                self.get(0, 0).result() == self.get(1, 1).result() &&
                self.get(0, 0).result() == self.get(2, 2).result() {

                return self.get(0, 0).result();
            }

            if last_x == 2 - last_y &&
                self.get(0, 2).result() == self.get(1, 1).result() &&
                self.get(0, 2).result() == self.get(2, 0).result() {

                return self.get(0, 2).result();
            }
        }

        if self.get(last_x, 0).result() == self.get(last_x, 1).result() &&
            self.get(last_x, 0).result() == self.get(last_x, 2).result() {

            return self.get(last_x, 0).result();
        }

        if self.get(0, last_y).result() == self.get(1, last_y).result() &&
            self.get(0, last_y).result() == self.get(2, last_y).result() {

            return self.get(0, last_y).result();
        }

        if self.items.iter().any(|item| item.result() == None) {
            None
        } else {
            Some(GameResult::Draw)
        }
    }
}

impl BoardStructure<SubBoard> {
    pub fn new() -> BoardStructure<SubBoard> {
        BoardStructure {
            items: [SubBoard::new(); 9]
        }
    }
}

impl BoardStructure<Square> {
    pub fn new() -> BoardStructure<Square> {
        BoardStructure {
            items: [Square::new(); 9]
        }
    }
}

#[derive(Copy, Clone)]
pub struct Square {
    result: Option<GameResult>,
}

impl Square {
    pub fn new() -> Square {
        Square { result: None }
    }
}

impl Owned for Square {
    fn result(&self) -> Option<GameResult> {
        self.result
    }

    fn set_result(&mut self, result: Option<GameResult>) {
        self.result = result;
    }
}
