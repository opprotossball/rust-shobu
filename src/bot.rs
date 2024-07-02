use crate::shobu::*;
use crate::bot_constants::*;
use crate::shobu_move::Move;
use crate::shobu_move::MoveExtended;
use crate::tt_entry::TTEntry;
use rustc_hash::FxHashMap;
use std::io;
use std::iter::zip;
use crate::utils;
use crate::tt_entry::*;

pub struct ShobuBot {
    tt: FxHashMap<u64, TTEntry>,
    max_depth: usize,
    psts: [[f64; 36]; 2],
    tt_size: usize,
    negamax_calls: usize
}

impl ShobuBot {
    pub fn new() -> Self {
        ShobuBot {
            max_depth: MAX_DEPTH,
            psts: PSTS,
            tt_size: TT_SIZE,
            tt: FxHashMap::default(),
            negamax_calls: 0
        }
    }

    pub fn play_game(&mut self) {
        let stdin = io::stdin();
        loop {
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
            position.make_move(&moves[i].mv).unwrap();
            let eval = -self.negamax(position, self.max_depth-1, -INF, INF, position.active_player);
            position.undo_move();
            if eval > best_eval {
                best_eval = eval;
                best_move = i;
            }
        }
        moves[best_move].mv.deep_copy()
    }

    fn mobility_score(&self, active_player: i8, position: &Shobu) -> f64 {
        let (m1, m2) = 
            if active_player == BLACK
                {(position.available_passive_directions(0, -1), position.available_passive_directions(1, -1))}
            else 
                {(position.available_passive_directions(2, 1), position.available_passive_directions(3, 1))};
        self.board_mobility_score(m1) + self.board_mobility_score(m2)
    }

    fn board_mobility_score(&self, mobility: usize) -> f64 {
        if mobility < 7 {return -10.0 + 0.7 * (mobility as f64);}
        else if mobility < 13 {return 0.7 * mobility as f64;}
        else {return 8.4 + 0.5 * (mobility as f64 - 12.0)};
    }

    fn eval(&self, position: &Shobu) -> f64 {
        let mut eval = 0.0;
        // black pieces
        for piece_list in position.pieces[0] {
            let mut material = 0;
            for tile in piece_list {
                if tile == NOT_ON_BOARD { continue; }
                eval -= self.psts[0][tile];
                material += 1;
            }
            eval -= MATERIAL[material];
        }
        eval -= self.mobility_score(BLACK, position);
        // white pieces
        for piece_list in position.pieces[1] {
            let mut material = 0;
            for tile in piece_list {
                if tile == NOT_ON_BOARD { continue; }
                eval += self.psts[1][tile];
                material += 1;
            }
            eval += MATERIAL[material];
        }
        eval += self.mobility_score(WHITE, position);
        eval
    }

    fn moves_ordered(&mut self, position: &mut Shobu) -> Vec<MoveExtended> {
        let mut moves = position.get_legal_moves();
        moves.sort_by_key(|x| if x.push_1 || x.push_2 {0} else {1} );
        moves
    }

    fn moves_order_tt_lookup(&mut self, position: &mut Shobu, depth: usize) -> Vec<MoveExtended> {
        let mut move_evals = Vec::new();
        let mut moves = position.get_legal_moves();
        for mv in &moves {
            let mut eval = if mv.push_1 || mv.push_2 {-1000.0 + 1.0} else {-1000.0};
            if depth < self.max_depth - 1 {
                _ = position.make_move(&mv.mv);
                match self.tt.get(&position.hash) {
                    Some(entry) => eval += entry.eval,
                    None => ()
                }
                position.undo_move();
            }
            move_evals.push(eval);
        }
        let mut combined: Vec<_> = zip(moves, move_evals).collect();
        combined.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let (moves, _): (Vec<_>, Vec<_>) = combined.into_iter().unzip();
        moves
    }
    
    fn get_transposition(&mut self, position: &Shobu, depth: usize) -> Option<&TTEntry> {
        match self.tt.get(&position.hash) {
            Some(entry) => {
                if entry.depth >= depth && position.hash == entry.hash {
                    return Some(entry)
                }
            },
            None => ()
        }
        None
    }

    fn negamax(&mut self, position: &mut Shobu, depth: usize, alpha_prev: f64, beta_prev: f64, active_player: i8) -> f64 {
        let mut alpha = alpha_prev;
        let mut beta = beta_prev;
        self.negamax_calls += 1;
        match self.get_transposition(position, depth) {
            Some(entry) => {
                match entry.flag {
                    EXACT => return entry.eval,
                    LOWERBOUND => alpha = f64::max(alpha, entry.eval),
                    UPPERBOUND => beta = f64::min(beta, entry.eval),
                    _ => ()
                }
                if alpha >= beta
                { 
                    return entry.eval; 
                }
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
        for (i, mv) in self.moves_ordered(position).into_iter().enumerate() {
            _ = position.make_move(&mv.mv);
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
        let hash = position.hash; 
        let new_entry = TTEntry::new(hash, best_eval, flag, best_move, depth);
        self.tt.insert(hash, new_entry);

        best_eval
    }
}