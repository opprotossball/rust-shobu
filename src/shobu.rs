use std::hash;

use crate::shobu_move::{self, internal_2_readable, Move, MoveExtended};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub const BLACK: i8 = -1;
pub const WHITE: i8 = 1;
pub const EMPTY: i8 = 0;
pub const MARGIN: i8 = i8::MIN;
pub const NOT_ON_BOARD: usize = 0;
pub const TILES: [usize; 16] = [
    7, 8, 9, 10,
    13, 14, 15, 16,
    19, 20, 21, 22,
    25, 26, 27, 28
];
pub const DIRECTIONS: [i8; 8] = [-6, -5, 1, 7, 6, 5, -1, -7];
const SHIFTS: [[[usize; 8]; 2]; 2] = [
    [[3, 5, 7, 11, 13, 17, 19, 23], [7, 11, 3, 5, 19, 23, 13, 17]],
    [[5, 3, 11, 7, 17, 13, 23, 19], [11, 7, 5, 3, 23, 19, 17, 13]]
];
const ZOBRIST_TILES: [usize; 16] = [0, 1, 1, 0, 2, 3, 3, 2, 4, 5, 5, 4, 6, 7, 7, 6];

pub struct Shobu {
    pub active_player: i8,
    pub winner: i8,
    pub boards: [[i8; 36]; 4],
    pub pieces: [[[usize; 4]; 4]; 2],
    pub history: Vec<(Move, usize, usize)>,
    pub hashes: [[u64; 2]; 2],
    piece_hash_vals: [[u64; 8]; 2],
    black_active_hash: u64,
}

fn occupied(val: i8) -> bool {
    val == BLACK || val == WHITE
}

impl Shobu {
    pub fn new() -> Self {
        let mut rand = StdRng::seed_from_u64(2137);
        let mut new = Self {
            active_player: BLACK,
            winner: 0,
            boards: [[MARGIN; 36]; 4],
            pieces: [[[NOT_ON_BOARD; 4]; 4]; 2],
            history: Vec::new(),
            piece_hash_vals: rand.gen(),
            black_active_hash: rand.gen(),
            hashes: [[0; 2]; 2]
        };
        new.init();
        new.init_hashes();
        new
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), String> {
        if self.winner != 0 { return Err("Game is over!".to_string()); }
        let _ = self.validate_and_extend(&mv)?;
        self.make_move_unsafe(mv);
        return Ok(());
    }

    pub fn get_hash(&self) -> u64 {
        let mut hash = if self.active_player == BLACK {self.black_active_hash} else {0};
        for color_swap in 0..=1 {
            for horizontal_swap in 0..=1 {
                hash ^= self.hashes[color_swap][horizontal_swap];
            }
        }
        hash
    }

    pub fn get_symmetry_hash(&self, color_swap: bool, horizontal_swap: bool) -> u64 {
        if self.active_player == BLACK {
            self.hashes[color_swap as usize][horizontal_swap as usize] ^ self.black_active_hash
        } else {
            self.hashes[color_swap as usize][horizontal_swap as usize]
        }
    }

    pub fn make_move_unsafe(&mut self, mv: &Move) {
        let pushed_from_1 = self.move_on_board_unsafe(mv.board_1, mv.direction, mv.from_1, mv.double);
        let pushed_from_2 = self.move_on_board_unsafe(mv.board_2, mv.direction, mv.from_2, mv.double);
        self.active_player = -self.active_player;
        // add to history
        if pushed_from_1 != NOT_ON_BOARD {
            self.history.push((mv.deep_copy(), mv.board_1, pushed_from_1))
        } else {
            // if push occured on board_2 or neither move was push
            self.history.push((mv.deep_copy(),  mv.board_2, pushed_from_2))
        }
    }

    pub fn available_passive_directions(&self, board_id: usize, active_player: i8) -> usize {
        let player_id = if active_player == BLACK {0} else {1};
        let mut dir_count = 0;
        for double in [1, 2] {
            for direction in DIRECTIONS {
                let diff = direction * double;
                for piece in self.pieces[player_id][board_id] {
                    if piece == NOT_ON_BOARD {continue;}
                    if piece as i8 + diff > 0 && piece as i8 + diff < 36 && self.boards[board_id][(piece as i8 + diff) as usize] == EMPTY && self.boards[board_id][(piece as i8 + direction) as usize] == EMPTY {
                        dir_count += 1;
                        break;
                    }
                }
            }
        }
        dir_count
    }

    pub fn undo_move(&mut self) {
        self.winner = 0;
        self.active_player = -self.active_player;
        let (mv, pushed_board, pushed_from) = self.history.pop().unwrap();
        let diff = shobu_move::diff(mv.direction, mv.double);
        // undo moves
        self.boards[mv.board_1][mv.from_1] = self.active_player;
        let to_1 = (mv.from_1 as i8 + diff) as usize;
        self.boards[mv.board_1][to_1] = EMPTY;
        self.update_piece_check_winner(self.active_player, mv.board_1, (mv.from_1 as i8 + diff) as usize, mv.from_1);
        self.boards[mv.board_2][mv.from_2] = self.active_player;
        let to_2 = (mv.from_2 as i8 + diff) as usize;
        self.boards[mv.board_2][to_2] = EMPTY;
        self.update_piece_check_winner(self.active_player, mv.board_2, (mv.from_2 as i8 + diff) as usize, mv.from_2);
        // undo push
        if pushed_from != NOT_ON_BOARD {
            self.update_hashes(-self.active_player, pushed_board, pushed_from);
            self.boards[pushed_board][pushed_from] = -self.active_player;
            let pushed_to = if pushed_board == mv.board_1 {
                (mv.from_1 as i8 + diff + mv.direction) as usize
            } else {
                (mv.from_2 as i8 + diff + mv.direction) as usize
            };
            if self.boards[pushed_board][pushed_to] == MARGIN {
                self.update_piece_check_winner(-self.active_player, pushed_board, NOT_ON_BOARD, pushed_from);
            } else {
                self.boards[pushed_board][pushed_to] = EMPTY;
                self.update_piece_check_winner(-self.active_player, pushed_board, pushed_to, pushed_from);
                self.update_hashes(-self.active_player, pushed_board, pushed_to);
            }
        }
        self.update_hashes(self.active_player, mv.board_1, mv.from_1);
        self.update_hashes(self.active_player, mv.board_1, to_1);
        self.update_hashes(self.active_player, mv.board_2, mv.from_2);
        self.update_hashes(self.active_player, mv.board_2, to_2);
    }

    fn move_on_board_unsafe(&mut self, board_id: usize, direction: i8, from: usize, double: bool) -> usize {
        let diff = shobu_move::diff(direction, double);
        let to = (from as i8 + diff) as usize;
        let board = &mut self.boards[board_id];
        let mut pushed_from = NOT_ON_BOARD;
        if occupied(board[to]) { pushed_from = to }
        // push
        let jump_over = (from as i8 + direction) as usize;
        if  double && occupied(board[jump_over]) {
            pushed_from = jump_over;
            board[jump_over] = EMPTY;
        }
        board[from] = EMPTY;
        board[to] = self.active_player;
        if pushed_from != NOT_ON_BOARD {
            let mut pushed_to = (to as i8 + direction) as usize;
            if board[pushed_to] == MARGIN {
                pushed_to = NOT_ON_BOARD;
            } else {
                board[pushed_to] = -self.active_player;
            }
            self.update_piece_check_winner(-self.active_player, board_id, pushed_from, pushed_to);
            // update hashes for pushed player
            self.update_hashes(-self.active_player, board_id, pushed_from);
            self.update_hashes(-self.active_player, board_id, pushed_to);
        }
        self.update_piece_check_winner(self.active_player, board_id, from, to);
        // update hashes for active player
        self.update_hashes(self.active_player, board_id, from);
        self.update_hashes(self.active_player, board_id, to);
        pushed_from
    }

    fn update_piece_check_winner(&mut self, player: i8, board_id: usize, from: usize, to: usize) {
        let pieces = &mut self.pieces[if player == BLACK {0} else {1}][board_id];
        // for winner check
        let mut piece_count = 0;
        // for debug purposes
        let mut found = false;
        for i in 0..pieces.len() {
            if pieces[i] == from && !found{ 
                found = true;
                pieces[i] = to;
            }
            if pieces[i] != NOT_ON_BOARD { piece_count += 1; }
        }
        if piece_count == 0 { self.winner = -player }
        if !found { 
            panic!("Updating position of piece not in list!") 
        }
    }

    pub fn validate_and_extend(&self, mv: &Move) -> Result<MoveExtended, String> {
        let error_msg = "Move is invalid!";
        let board_sum = mv.board_1 + mv.board_2;
        // boards have the same color
        if board_sum % 2 == 0 { return Err(error_msg.to_string()); }
        // both boards are on opponent's side
        if self.active_player == BLACK && board_sum > 3 { return Err(error_msg.to_string()); }
        if self.active_player == WHITE && board_sum < 3 { return Err(error_msg.to_string()); }
        let (legal_1, push_1) = self.is_legal_and_push(mv.board_1, mv.direction, mv.from_1, mv.double);
        if !legal_1 { return Err(error_msg.to_string()); }
        let (legal_2, push_2) = self.is_legal_and_push(mv.board_2, mv.direction, mv.from_2, mv.double);
        if !legal_2 { return Err(error_msg.to_string()); }
        // 2 aggressive moves
        if push_1 && push_2 { return Err(error_msg.to_string()); }
        // aggressive move on home board
        if self.active_player == BLACK && board_sum != 1 {
            if (mv.board_1 < 2 && push_1) || (mv.board_2 < 2 && push_2) { return Err(error_msg.to_string()); }
        } else if self.active_player == WHITE && board_sum != 5 {
            if (mv.board_1 > 1 && push_1) || (mv.board_2 > 1 && push_2) { return Err(error_msg.to_string()); }
        }
        Ok(MoveExtended::new(mv, push_1, push_2))
    }

    pub fn get_legal_moves(&self) -> Vec<MoveExtended> {
        let mut res = Vec::new();
        self.moves_for_board_pair(&mut res, 0, 3);
        self.moves_for_board_pair(&mut res, 1, 2);
        if self.active_player == BLACK {
           self.moves_for_board_pair(&mut res, 0, 1);
        } else {
            self.moves_for_board_pair(&mut res, 2, 3);
        }
        res
    }

    fn moves_for_board_pair(&self, out: &mut Vec<MoveExtended>, board_1: usize, board_2: usize) {
        for direction in DIRECTIONS {
            for double in [false, true] {
                for piece_1 in self.pieces[if self.active_player == BLACK { 0 } else { 1 }][board_1] {
                    if piece_1 == NOT_ON_BOARD { continue; }
                    let (legal_1, push_1) = self.is_legal_and_push(board_1, direction, piece_1, double);
                    if !legal_1 { continue; }
                    for piece_2 in self.pieces[if self.active_player == BLACK { 0 } else { 1 }][board_2] {
                        if piece_2 == NOT_ON_BOARD { continue; }
                        let (legal_2, push_2) = self.is_legal_and_push(board_2, direction, piece_2, double);
                        if !legal_2 { continue; }
                        // 2 aggressive moves
                        if push_1 && push_2 { continue; }
                        // aggressive move on home board
                        let board_sum = board_1 + board_2;
                        if self.active_player == BLACK && board_sum != 1 {
                            if (board_1 < 2 && push_1) || (board_2 < 2 && push_2) { continue; }
                        } else if self.active_player == WHITE && board_sum != 5 {
                            if (board_1 > 1 && push_1) || (board_2 > 1 && push_2) { continue; }
                        }
                        let mv: Move = Move {
                            board_1: board_1,
                            board_2: board_2,
                            direction: direction,
                            from_1: piece_1,
                            from_2: piece_2,
                            double: double,
                        };
                        let res: MoveExtended = MoveExtended {
                            mv: mv,
                            push_1: push_1,
                            push_2: push_2
                        };
                        out.push(res);
                    }
                }
            }
        }
    }

    fn is_legal_and_push(&self, board_id: usize, direction: i8, from: usize, double: bool) -> (bool, bool) {
        let board: [i8; 36] = self.boards[board_id];
        // invalid stone color
        if board[from] != self.active_player { return (false, false); }
        let diff = shobu_move::diff(direction, double);
        let to = (from as i8 + diff) as usize;
        // goes out of board
        if to >= board.len() || board[to] == MARGIN { return (false, false); }
        let mut pieces_on_path = 0;
        // check target tile
        if occupied(board[to]) {
            if board[to] == self.active_player { return (false, false); }
            pieces_on_path += 1;
        }
        // if double check tile on path
        let next = i8::saturating_add(from as i8, direction) as usize;
        if double && occupied(board[next]) {
            if board[next] == self.active_player { return (false, false); }
            pieces_on_path += 1;
        }
        // double push
        if pieces_on_path > 1 { return (false, false); }
        // push blocked
        if pieces_on_path > 0 && occupied(board[i8::saturating_add(to as i8, direction) as usize]) { return (false, false); }
        (true, pieces_on_path > 0)
    }

    pub fn to_string(&self) -> String
    {
        let mut pos_chars: Vec<char> = Vec::new();
        if self.active_player == BLACK { pos_chars.push('b'); }
        else { pos_chars.push('w'); };
        for board in self.boards.iter() {
            pos_chars.push(' ');
            for tile in TILES {
                match board[tile] {
                    BLACK => pos_chars.push('b'),
                    WHITE => pos_chars.push('w'),
                    _ => pos_chars.push('_')
                }
            }
        }; 
        pos_chars.into_iter().collect()
    }

    pub fn from_string(string: &str) -> Self {
        let mut rand = StdRng::seed_from_u64(2137);
        let mut new = Self {
            active_player: BLACK,
            winner: 0,
            boards: [[MARGIN; 36]; 4],
            pieces: [[[0; 4]; 4]; 2],
            history: Vec::new(),
            piece_hash_vals: rand.gen(),
            black_active_hash: rand.gen(),
            hashes: [[0; 2]; 2]
        };
        let pos = string.split(" ");
        for (i, part) in pos.into_iter().enumerate() {
            if i == 0 {
                if part == "b" { new.active_player = -1 }
                else { new.active_player = 1 }
            } else {
                let mut black_added = 0;
                let mut white_added = 0;
                for (j, val) in part.chars().enumerate() {
                    let piece = match val {
                        'b' => { 
                            new.pieces[0][i - 1][black_added] = TILES[j];
                            black_added += 1; 
                            BLACK 
                        },
                        'w' => { 
                            new.pieces[1][i - 1][white_added] = TILES[j];
                            white_added += 1; 
                            WHITE 
                        },
                        _ => EMPTY
                    };
                    new.boards[i - 1][TILES[j]] = piece;
                }
            }
        };
        new.init_hashes();
        new
    }
    
    fn update_hashes(&mut self, player: i8, board_id: usize, tile: usize) {
        if tile == NOT_ON_BOARD { return; }
        let tile_id = internal_2_readable(tile);
        let part_id: usize = 2 * board_id + (tile % 4) / 2;
        let player_id = if player == BLACK {0} else {1};
        for color_swap in 0..=1 {
            for horizontal_swap in 0..=1 {
                self.hashes[color_swap][horizontal_swap] 
                    ^= self.piece_hash_vals[player_id][ZOBRIST_TILES[tile_id]] 
                    << SHIFTS[color_swap][horizontal_swap][part_id];
            }
        }
    }

    fn init(&mut self) {
        for (i, board) in self.boards.iter_mut().enumerate() {
            let mut white_added: usize = 0;
            let mut black_added: usize = 0;
            for tile in TILES.iter() {
                if tile <= &10 
                { 
                    board[*tile] = WHITE; 
                    self.pieces[1][i][white_added] = *tile;
                    white_added += 1;
                }
                else if tile >= &25 
                { 
                    board[*tile] = BLACK; 
                    self.pieces[0][i][black_added] = *tile;
                    black_added += 1;
                }
                else 
                { 
                    board[*tile] = EMPTY; 
                }
            }
        }
    }

    fn calculate_symmetry_hash(&self, color_swap: bool, horizontal_swap: bool) -> u64 {
        let mut hash = if self.active_player == BLACK {self.black_active_hash} else {0};
        for part_hash in self.parts_hash(&SHIFTS[horizontal_swap as usize][color_swap as usize]) {
            hash ^= part_hash;
        }
        hash
    }

    fn init_hashes(&mut self) {
        for color_swap in [false, true] {
            for horizontal_swap in [false, true] {
                self.hashes[color_swap as usize][horizontal_swap as usize] ^= self.calculate_symmetry_hash(color_swap, horizontal_swap);
            }
        }
    }

    fn parts_hash(&self, shifts: &[usize; 8]) -> [u64; 8] {
        let mut parts = [0; 8];
        for (i, board) in self.boards.into_iter().enumerate() {
            for j in 0..16 {
                let part_id: usize = 2 * i + (j % 4) / 2;
                if board[TILES[j]] == BLACK {
                    parts[part_id] ^= self.piece_hash_vals[0][ZOBRIST_TILES[j]] << shifts[part_id];
                } else if board[TILES[j]] == WHITE {
                    parts[part_id] ^= self.piece_hash_vals[1][ZOBRIST_TILES[j]] << shifts[part_id];
                }
            }
        }
        parts
    }
}