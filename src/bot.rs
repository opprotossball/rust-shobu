use crate::shobu::*;
use crate::bot_constants::*;
use crate::shobu_move::Move;
use crate::tt_entry;
use crate::tt_entry::TTEntry;
use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::io;
use crate::utils;
use crate::tt_entry::*;

const INF: f64 = 1_000_000_000.0;
const WIN_EVAL: f64 = 1_000_000.0;

pub struct ShobuBot {
    tt: HashMap<u64, TTEntry>,
    max_depth: usize,
    piece_value: f64,
    psts: [[f64; 36]; 2],
    tt_size: usize 
}

impl ShobuBot {
    pub fn new() -> Self {
        ShobuBot {
            max_depth: MAX_DEPTH,
            piece_value: PIECE_VALUE,
            psts: PSTS,
            tt_size: TT_SIZE,
            tt: HashMap::with_capacity(TT_SIZE)
        }
    }

    pub fn play_game(&mut self) {
        let stdin = io::stdin();
        while true {
            let _ = utils::input(&stdin);
            let position = utils::input(&stdin);
            let mut game = Shobu::from_string(&position);
            let mv = self.choose_move(&mut game);
            println!("{}", game.validate_and_extend(&mv).unwrap().to_string(game.active_player));
        };
    }

    pub fn choose_move(&mut self, position: &mut Shobu) -> Move {
        let moves = position.get_legal_moves();
        let mut best_move = 0;
        let mut best_eval = -INF;
        for i in 0..moves.len() {
            position.make_move(&moves[i]).unwrap();
            let eval = -self.negamax(position, self.max_depth-1, -INF, INF, position.active_player);
            position.undo_move();
            if eval > best_eval {
                best_eval = eval;
                best_move = i;
            }
        }
        moves[best_move].deep_copy()
    }

    fn eval(&self, position: &Shobu) -> f64 {
        let mut eval = 0.0;
        // black pieces
        for piece_list in position.pieces[0] {
            for tile in piece_list {
                if tile == NOT_ON_BOARD { continue; }
                eval -= self.piece_value;
                eval -= self.psts[0][tile];
            }
        }
        // white pieces
        for piece_list in position.pieces[1] {
            for tile in piece_list {
                if tile == NOT_ON_BOARD { continue; }
                eval += self.piece_value;
                eval += self.psts[1][tile];
            }
        }
        eval
    }

    // (entry, is_color_swap, is_horizontal_swap)
    fn get_transposition(&self, position: &Shobu) -> Option<(&TTEntry, bool, bool)> {
        for color_swap in [false, true] {
            for horizontal_swap in [false, true] {
                match self.tt.get(&position.get_hash(color_swap, horizontal_swap)) {
                    Some(entry) => return Some((entry, color_swap, horizontal_swap)),
                    None => ()
                }
            }
        }
        None
    }

    fn negamax(&mut self, position: &mut Shobu, depth: usize, alpha_prev: f64, beta_prev: f64, active_player: i8) -> f64 {
        let mut alpha = alpha_prev;
        let mut beta = beta_prev;

        match self.get_transposition(position) {
            Some((entry, color_swap, horizontal_swap)) => 'found: {
                if entry.depth < depth { break 'found; }
                match entry.flag {
                    EXACT => return entry.eval,
                    LOWERBOUND => alpha = f64::max(alpha, entry.eval),
                    UPPERBOUND => beta = f64::min(beta, entry.eval),
                    _ => ()
                }
                if alpha >= beta { return entry.eval; }
            },
            None => ()
        }

        if position.winner != 0 {
            return (position.winner * position.active_player) as f64 * (WIN_EVAL + depth as f64);
        }
        if depth == 0 {
            return position.active_player as f64 * self.eval(position);
        }
        let mut best_eval: f64 = -INF;
        let mut best_move = 0;
        for (i, mv) in position.get_legal_moves().into_iter().enumerate() {
            position.make_move(&mv);
            let eval = -self.negamax(position, depth - 1, -beta, -alpha, -active_player);
            position.undo_move();
            if eval > best_eval {
                best_eval = eval;
                best_move = i;
            }
            alpha = f64::max(alpha, best_eval);
            if alpha >= beta { break; };
        }

        let flag = if best_eval <= alpha_prev { UPPERBOUND }
            else if best_eval >= beta { LOWERBOUND }
            else { EXACT };
        let hash = position.get_hash(false, false); 
        let new_entry = TTEntry::new(hash, best_eval, flag, best_move, depth);
        self.tt.insert(hash, new_entry);

        best_eval
    }
}