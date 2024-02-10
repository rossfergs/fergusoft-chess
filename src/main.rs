mod pieces;
mod board;
mod square;

use crate::board::Board;
use crate::square::Square;

fn main() {
    println!("Chess :)");

    let square = Square::new();
    let mut board = Board::new();
    board.add_colours();
    println!("{}", board.display_board());
}
