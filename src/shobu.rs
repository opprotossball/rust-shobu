use std::usize;

pub const BLACK: i8 = -1;
pub const WHITE: i8 = 1;
pub const EMPTY: i8 = 0;
pub const MARGIN: i8 = i8::MIN;
pub const TILES: [usize; 16] = [
    7, 8, 9, 10,
    13, 14, 15, 16,
    19, 20, 21, 22,
    25, 26, 27, 28
];

#[derive(Debug)]
pub struct Shobu {
    pub active_player: i8,
    pub winner: i8,
    pub boards: [[i8; 36]; 4],
    pub pieces: [[[usize; 4]; 4]; 2],
}

impl Shobu {
    pub fn new() -> Self {
        let mut new = Self {
            active_player: -1,
            winner: 0,
            boards: [[MARGIN; 36]; 4],
            pieces: [[[0; 4]; 4]; 2] 
        };
        new.init();
        return new;
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
            active_player: -1,
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