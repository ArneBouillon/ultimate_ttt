#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Action {
    sub_x: usize,
    sub_y: usize,
    x: usize,
    y: usize,
}

impl Action {
    pub fn new(sub_x: usize, sub_y: usize, x: usize, y: usize) -> Action {
        Action {
            sub_x,
            sub_y,
            x,
            y,
        }
    }
}
