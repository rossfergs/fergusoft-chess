mod pieces;
mod board;
mod square;

use crate::board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board.display_board());
}
