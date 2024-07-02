pub const EXACT: usize = 0;
pub const LOWERBOUND: usize = 1;
pub const UPPERBOUND: usize = 2;

pub struct TTEntry {
    pub hash: u64,
    pub eval: f64,
    pub flag: usize,
    pub best_move: usize,
    pub depth: usize 
}

impl TTEntry {
    pub fn new(hash: u64, eval: f64, flag: usize, best_move: usize, depth: usize) -> Self {
        TTEntry {
            hash: hash,
            eval: eval,
            flag,
            best_move: best_move,
            depth: depth
        }
    }
}
