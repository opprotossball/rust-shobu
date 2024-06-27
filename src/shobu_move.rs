pub struct Move {
    pub board_1: usize,
    pub board_2: usize,
    pub direction: i8,
    pub from_1: usize,
    pub from_2: usize,
    pub double: bool
}

pub fn diff(direction: i8, double: bool) -> i8 {
    if double { return 2 * direction; }
    direction
}

impl Move {
    
}