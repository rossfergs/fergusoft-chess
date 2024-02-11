mod pieces;
mod board;
mod square;

use crate::board::Board;

fn main() {
    println!("Chess :)");
    println!();
    let mut board = Board::new();
    println!("{}", board.display_board());
}
