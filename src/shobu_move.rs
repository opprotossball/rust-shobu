use std::cmp::min;
use std::cmp::max;
use crate::shobu::DIRECTIONS;
use crate::shobu::BLACK;
use crate::shobu::TILES;
pub const DIRECTION_CODES: [&str; 8] = ["U", "UR", "R", "DR", "D", "DL", "L", "UL"]; 

pub struct Move {
    pub board_1: usize,
    pub board_2: usize,
    pub direction: i8,
    pub from_1: usize,
    pub from_2: usize,
    pub double: bool,
}

pub struct MoveExtended {
    pub mv: Move,
    pub push_1: bool,
    pub push_2: bool 
}

pub fn diff(direction: i8, double: bool) -> i8 {
    if double { return 2 * direction; }
    direction
}

pub fn readable_2_internal(tile: usize) -> usize {
    TILES[tile]
}
    
pub fn internal_2_readable(tile: usize) -> usize {
    tile - 2 * ((tile - 7) / 6) - 7
}

impl MoveExtended {

    fn passive_aggressive_board(&self, active_player: i8) -> (usize, usize) {
        if !self.push_1 && !self.push_2 {
            if active_player == BLACK {
                return (min(self.mv.board_1, self.mv.board_2), max(self.mv.board_1, self.mv.board_2));
            } else {
                return (max(self.mv.board_1, self.mv.board_2), min(self.mv.board_1, self.mv.board_2));
            }
        } else if self.push_1 {
            return (self.mv.board_2, self.mv.board_1);
        } else {
            return (self.mv.board_1, self.mv.board_2);
        }
    }

    pub fn new(mv: &Move, push_1: bool, push_2: bool) -> Self {
        MoveExtended {
            mv: mv.deep_copy(),
            push_1: push_1,
            push_2: push_2
        }
    }

    pub fn to_move(&self) -> Move {
        Move {
            board_1: self.mv.board_1,
            board_2: self.mv.board_2,
            direction: self.mv.direction,
            from_1: self.mv.from_1,
            from_2: self.mv.from_2,
            double: self.mv.double
        }
    }

    pub fn to_string(&self, active_player: i8) -> String {
        let mut encoded = String::new();
        if self.mv.double { encoded.push('2'); }
        let direction_id = DIRECTIONS.iter().position(|&x| x == self.mv.direction).unwrap();
        encoded.push_str(DIRECTION_CODES[direction_id]);
        let (passive_board, aggressive_board) = self.passive_aggressive_board(active_player);
        if passive_board % 2 == 0 {
            encoded.push('b');
        } else {
            encoded.push('w');
        }
        let passive_from = internal_2_readable(if passive_board == self.mv.board_1 { self.mv.from_1 } else { self.mv.from_2 });
        encoded.push_str(passive_from.to_string().as_str());
        if self.mv.board_1 + self.mv.board_2 == 3 {
            encoded.push('f');
        } else {
            encoded.push('h');
        }
        let aggressive_from = internal_2_readable(if aggressive_board == self.mv.board_1 { self.mv.from_1 } else { self.mv.from_2 });
        encoded.push_str(aggressive_from.to_string().as_str());
        encoded
    }

}

impl Move {

    // pub fn create_symmetric(&self, color_swap: bool, horizontal_swap: bool) -> Self {
    //     self.deep_copy()
    // }

    pub fn deep_copy(&self) -> Self {
        Move {
            board_1: self.board_1,
            board_2: self.board_2,
            direction: self.direction,
            from_1: self.from_1,
            from_2: self.from_2,
            double: self.double
        }
    }

    pub fn from_string(encoded: &str, active_player: i8) -> Option<Move> {
        let double = encoded.starts_with('2');
        let mut index = if double { 1 } else { 0 };

        let mut direction_chars = String::new();
        while index < encoded.len() && encoded[index..].chars().next()?.is_uppercase() {
            direction_chars.push(encoded[index..].chars().next()?);
            index += 1;
        }
        let direction_id = DIRECTION_CODES.iter().position(|&x| x == &direction_chars).unwrap();
        let direction = DIRECTIONS[direction_id];

        let board_1 = if encoded[index..].starts_with('b') { 1 + active_player } else { 2 + active_player };
        index += 1;

        let from_1_start: usize = index;
        while index < encoded.len() && encoded[index..].chars().next()?.is_digit(10) {
            index += 1;
        }
        let from_1 = encoded[from_1_start..index].parse().ok()?;
        let from_1 = readable_2_internal(from_1);

        let board_2 = if encoded[index..].starts_with('h') { 
            if active_player == BLACK {
                1 - board_1
            } else {
                5 - board_1
            } 
        } else {
            3 - board_1
        } ;
        index += 1;

        let aggressive_from_start = index;
        while index < encoded.len() && encoded[index..].chars().next()?.is_digit(10) {
            index += 1;
        }
        let from_2 = encoded[aggressive_from_start..index].parse().ok()?;
        let from_2 = readable_2_internal(from_2);

        Some(Move {
            board_1: board_1 as usize,
            board_2: board_2 as usize,
            direction: direction,
            from_1: from_1,
            from_2: from_2,
            double: double
        })
    }
}