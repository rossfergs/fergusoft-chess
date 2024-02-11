mod pieces;
mod board;
mod square;

use crate::board::Board;
use crate::square::Square;

fn main() {
    println!("Chess :)");
    let mut board = Board::new();
    println!("{}", board.display_board());
}
