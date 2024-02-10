use std::fmt::{Display, Formatter};

/*
P - Pawn
K - King

 */

#[derive(Clone, PartialEq)]
pub struct Piece {
    colour: String,
    piece: String,
}

impl Piece {
    pub fn new(new_colour: String, new_piece: String) -> Piece {
        Piece {
            colour: new_colour,
            piece: new_piece,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.colour, self.piece)
    }
}