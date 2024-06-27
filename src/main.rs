mod shobu;
mod tests;
use crate::shobu::Shobu;
use crate::shobu::BLACK;
use crate::shobu::WHITE;
use crate::shobu::EMPTY;
use crate::shobu::MARGIN;

fn main() {
    let mut game = Shobu::new();
    println!("{}", game.to_string());
}
