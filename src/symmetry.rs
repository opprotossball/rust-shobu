use crate::{shobu::Shobu, tt_entry::{self, TTEntry}};

pub fn opposite_color_board(board_id: usize) -> usize {
    match board_id {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        _ => panic!("Invalid board index!")
    }
}

pub fn direction_flipped(direction: i8) -> i8 {
    match direction {
        -6 => -6,
        -5 => -7,
        1 => -1,
        7 => 5,
        6 => 6,
        5 => 7,
        -1 => 1,
        -7 => -5,
        _ => panic!("Invalid direction give!")
    }
}

pub fn tile_flipped(tile: usize) -> usize {
    match tile % 6 {
        1 => tile + 3,
        2 => tile + 1,
        3 => tile - 1,
        4 => tile - 3,
        _ => panic!("Invalid tile given!")
    }
}

pub fn transposition_symmetries(position: &Shobu, tt_entry: &TTEntry) -> Option<(bool, bool)> {
    for color_swap in [false, true] {
        for horizontal_swap in [false, true] {
            if position.get_symmetry_hash(color_swap, horizontal_swap) == tt_entry.variation_hash {
                return Some((color_swap, horizontal_swap));
            }
        }
    }
    None
    //panic!("TTEntry instance do not have matching hash!");
}