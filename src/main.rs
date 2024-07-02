mod shobu;
mod tests;
mod zobrist;
mod shobu_move;
mod bot;
mod bot_constants;
mod utils;
mod tt_entry;
use crate::bot::ShobuBot;
use crate::zobrist::Zobrist;
use crate::shobu::Shobu;
use crate::shobu::BLACK;
use crate::shobu::WHITE;
use crate::shobu::EMPTY;
use crate::shobu::MARGIN;

use std::io;

fn main() {
    let mut bot = ShobuBot::new();
    // bot.play_game();
    let mut game = Shobu::new();
    let mv = bot.choose_move(&mut game);
}