use super::player::Player;

pub trait Owned {
    fn owner(&self) -> Option<Player>;
    fn set_owner(&mut self, owner: Option<Player>);
}

pub struct Board {
    pub structure: BoardStructure<SubBoard>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            structure: <BoardStructure<SubBoard>>::new()
        }
    }

    pub fn structure(&self) -> &BoardStructure<SubBoard> {
        &self.structure
    }

    pub fn structure_mut(&mut self) -> &mut BoardStructure<SubBoard> {
        &mut self.structure
    }

    pub fn make_move(&mut self, player: Player, sub_x: usize, sub_y: usize, x: usize, y: usize) -> (Option<usize>, Option<usize>, Option<Player>) {
        self.structure_mut()
            .get_mut(sub_x, sub_y)
            .structure_mut()
            .set_owner_at(x.clone(), y.clone(), Some(player));

        let mut winning_player: Option<Player> = None;
        match self.structure_mut().get_mut(sub_x, sub_y).structure().check_winner(x, y) {
            None => {},
            Some(player) => {
                self.structure_mut().get_mut(sub_x, sub_y).set_owner(Some(player));

                match self.structure.check_winner(sub_x, sub_y) {
                    None => {},
                    Some(player) => winning_player = Some(player),
                };
            }
        };

        match self.structure().get(x, y).owner() {
            None => (Some(x), Some(y), winning_player),
            Some(_) => (None, None, winning_player),
        }
    }
}

pub struct SubBoard {
    pub structure: BoardStructure<Square>,
    owner: Option<Player>,
}

impl SubBoard {
    pub fn new() -> SubBoard {
        SubBoard { structure: <BoardStructure<Square>>::new(), owner: None }
    }

    pub fn structure(&self) -> &BoardStructure<Square> {
        &self.structure
    }

    pub fn structure_mut(&mut self) -> &mut BoardStructure<Square> {
        &mut self.structure
    }
}

impl Owned for SubBoard {
    fn owner(&self) -> Option<Player> {
        self.owner
    }

    fn set_owner(&mut self, owner: Option<Player>) {
        self.owner = owner;
    }
}

pub struct BoardStructure<T: Owned> {
    items: [T; 9],
}

impl<T: Owned> BoardStructure<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.items[3 * y + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.items[3 * y + x]
    }

    pub fn set_owner_at(&mut self, x: usize, y: usize, owner: Option<Player>) {
        self.items.get_mut(3 * y + x).unwrap().set_owner(owner);
    }

    pub fn check_winner(&self, last_x: usize, last_y: usize) -> Option<Player> {
        if ((last_x == 1) as usize + (last_y == 1) as usize) != 1 {
            if last_x == last_y &&
                self.get(0, 0).owner() == self.get(1, 1).owner() &&
                self.get(0, 0).owner() == self.get(2, 2).owner() {

                return self.get(0, 0).owner();
            }

            if last_x == 2 - last_y &&
                self.get(0, 2).owner() == self.get(1, 1).owner() &&
                self.get(0, 2).owner() == self.get(2, 0).owner() {

                return self.get(0, 2).owner();
            }
        }

        if self.get(last_x, 0).owner() == self.get(last_x, 1).owner() &&
            self.get(last_x, 0).owner() == self.get(last_x, 2).owner() {

            return self.get(last_x, 0).owner();
        }

        if self.get(0, last_y).owner() == self.get(1, last_y).owner() &&
            self.get(0, last_y).owner() == self.get(2, last_y).owner() {

            return self.get(0, last_y).owner();
        }

        return None;
    }
}

impl BoardStructure<SubBoard> {
    pub fn new() -> BoardStructure<SubBoard> {
        BoardStructure {
            items: [SubBoard::new(), SubBoard::new(), SubBoard::new(), SubBoard::new(), SubBoard::new(), SubBoard::new(), SubBoard::new(), SubBoard::new(), SubBoard::new()]
        }
    }
}

impl BoardStructure<Square> {
    pub fn new() -> BoardStructure<Square> {
        BoardStructure {
            items: [Square::new(), Square::new(), Square::new(), Square::new(), Square::new(), Square::new(), Square::new(), Square::new(), Square::new()]
        }
    }
}

pub struct Square {
    owner: Option<Player>,
}

impl Square {
    pub fn new() -> Square {
        Square { owner: None }
    }
}

impl Owned for Square {
    fn owner(&self) -> Option<Player> {
        self.owner
    }

    fn set_owner(&mut self, owner: Option<Player>) {
        self.owner = owner;
    }
}
