mod shobu;
mod tests;
mod shobu_move;
mod bot;
mod bot_constants;
mod utils;
mod tt_entry;
pub mod symmetry;

use crate::bot::ShobuBot;
use crate::shobu::Shobu;

fn main() {
    let mut bot = ShobuBot::new();
    bot.play_game();
    // let mut game = Shobu::new();
    // let mv = bot.choose_move(&mut game);
}