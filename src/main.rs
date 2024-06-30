mod shobu;
mod tests;
mod zobrist;
mod shobu_move;
mod bot;
mod bot_constants;
use crate::zobrist::Zobrist;
use crate::shobu::Shobu;
use crate::shobu::BLACK;
use crate::shobu::WHITE;
use crate::shobu::EMPTY;
use crate::shobu::MARGIN;

fn main() {
    let mut game = Shobu::new();
    let moves = game.get_legal_moves();
    println!("{}", moves.len())
}
