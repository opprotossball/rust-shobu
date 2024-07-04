use crate::shobu_move::Move;

pub const EXACT: usize = 0;
pub const LOWERBOUND: usize = 1;
pub const UPPERBOUND: usize = 2;

pub struct TTEntry {
    pub variation_hash: u64,
    pub eval: f64,
    pub flag: usize,
    pub depth: usize,
    pub best_move: Move
}

impl TTEntry {
    pub fn new(hash: u64, eval: f64, flag: usize, depth: usize, best_move: Move) -> Self {
        TTEntry {
            variation_hash: hash,
            eval: eval,
            flag,
            best_move: best_move,
            depth: depth
        }
    }
}
