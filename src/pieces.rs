use std::fmt::{Display, Formatter};


#[derive(Clone, PartialEq)]
pub struct Piece {
    //no piece colour cus we use upper case and lower case fo rthe colour
    piece: String,
}

impl Piece {
    pub fn new( new_piece: String) -> Piece {
        Piece {
            piece: new_piece,
        }
    }
}


impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.piece)
    }
}