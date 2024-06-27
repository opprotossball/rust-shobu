use crate::shobu_move::{self, Move};
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

#[derive(Debug)]
pub struct Shobu {
    pub active_player: i8,
    pub winner: i8,
    pub boards: [[i8; 36]; 4],
    pub pieces: [[[usize; 4]; 4]; 2],
}

fn occupied(val: i8) -> bool {
    val == BLACK || val == WHITE
}

impl Shobu {
    pub fn new() -> Self {
        let mut new = Self {
            active_player: BLACK,
            winner: 0,
            boards: [[MARGIN; 36]; 4],
            pieces: [[[NOT_ON_BOARD; 4]; 4]; 2] 
        };
        new.init();
        new
    }

    pub fn is_legal(&self, mv: Move) -> bool {
        let board_sum = mv.board_1 + mv.board_2;
        // boards have the same color
        if board_sum % 2 == 0 { return false; }
        // both boards are on opponent's side
        if self.active_player == BLACK && board_sum > 3 { return false; }
        if self.active_player == WHITE && board_sum < 3 { return false; }
        let (legal_1, push_1) = self.is_legal_and_push(mv.board_1, mv.direction, mv.from_1, mv.double);
        if !legal_1 { return false; }
        let (legal_2, push_2) = self.is_legal_and_push(mv.board_2, mv.direction, mv.from_2, mv.double);
        if !legal_2 { return false; }
        // 2 aggressive moves
        if push_1 && push_2 { return false; }
        // aggressive move on home board
        if self.active_player == BLACK && board_sum != 1 {
            if (mv.board_1 < 2 && push_1) || (mv.board_2 < 2 && push_2) { return false; }
        } else if self.active_player == WHITE && board_sum != 5 {
            if (mv.board_1 > 1 && push_1) || (mv.board_2 > 1 && push_2) { return false; }
        }
        true
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
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

    fn moves_for_board_pair(&self, out: &mut Vec<Move>, board_1: usize, board_2: usize) {
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
                        let res = Move {
                            board_1: board_1,
                            board_2: board_2,
                            direction: direction,
                            from_1: piece_1,
                            from_2: piece_2,
                            double: double
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
            if board[to] == self.active_player { println!("{}", to); return (false, false); }
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
        let mut new = Self {
            active_player: BLACK,
            winner: 0,
            boards: [[MARGIN; 36]; 4],
            pieces: [[[0; 4]; 4]; 2] 
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
        new
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
}