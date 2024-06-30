use crate::shobu::*;
use crate::bot_constants::*;
use crate::shobu_move::Move;
use std::cmp::min;
use std::cmp::max;

const INF: f64 = 1_000_000_000.0;

pub struct ShobuBot {
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
            tt_size: TT_SIZE
        }
    }

    pub fn choose_move(&mut self, position: &mut Shobu) -> Move {
        let (_, move_id) = self.negamax(position, 3, -INF, INF, position.active_player);
        position.get_legal_moves()[move_id].deep_copy()
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

    fn negamax(&mut self, position: &mut Shobu, depth: usize, alpha: f64, beta: f64, active_player: i8) -> (f64, usize) {
        let mut alpha_new = alpha;
        if position.winner != 0 {
            return ((position.winner * position.active_player) as f64 * (INF + depth as f64), 0);
        }
        if depth == 0 {
            return (position.active_player as f64 * self.eval(position), 0);
        }
        let mut best_eval = -INF;
        let mut best_move = 0;
        for (i, mv) in position.get_legal_moves().into_iter().enumerate() {
            position.make_move(&mv);
            let eval = -self.negamax(position, depth - 1, -beta, -alpha_new, -active_player).0;
            position.undo_move();
            if eval > best_eval {
                best_eval = eval;
                best_move = i;
            }
            alpha_new = f64::max(alpha, best_eval);
            if alpha_new >= beta { break; };
        }
        (best_eval, best_move)
    }
}