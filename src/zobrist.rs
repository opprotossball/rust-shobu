use std::iter::zip;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::shobu::{Shobu, BLACK, TILES, WHITE};

const SHIFTS: [usize; 8] = [3, 5, 7, 11, 13, 17, 19, 23];
const ZOBRIST_TILES: [usize; 16] = [0, 1, 1, 0, 2, 3, 3, 2, 4, 5, 5, 4, 6, 7, 7, 6];
pub struct Zobrist {
    pub piece_vals: [[u64; 8]; 2],
    pub black_to_go: u64
}

impl Zobrist {
    
    pub fn get_hash(&self, position: &Shobu, color_swap: bool, horizontal_swap: bool) -> u64 {
        let mut shifts = SHIFTS.clone();
        if horizontal_swap {
            for i in 0..4 {
                shifts.swap(2 * i, 2 * i + 1)
            }
        }
        if color_swap {
            for i in 0..8 {
                if i % 4 < 2 {
                    shifts.swap(i, i + 2);
                }
            }
        }
        let mut hash = if position.active_player == BLACK {self.black_to_go} else {0};
        for (part_hash, shift) in zip(self.parts_hash(position), shifts) {
            hash ^= part_hash << shift; 
        }
        hash
    }

    pub fn new() -> Self  {
        let mut rand = StdRng::seed_from_u64(2137);
        Zobrist {
            piece_vals: rand.gen(),
            black_to_go: rand.gen()
        }
    }

    fn parts_hash(&self, position: &Shobu) -> [u64; 8] {
        let mut parts = [0; 8];
        for (i, board) in position.boards.into_iter().enumerate() {
            for j in 0..16 {
                let part_id: usize = 2 * i + (j % 4) / 2;
                if board[TILES[j]] == BLACK {
                    parts[part_id] ^= self.piece_vals[0][ZOBRIST_TILES[j]];
                } else if board[TILES[j]] == WHITE {
                    parts[part_id] ^= self.piece_vals[1][ZOBRIST_TILES[j]];
                }
            }
        }
        parts
    }
}