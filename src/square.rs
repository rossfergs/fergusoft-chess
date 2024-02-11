
use crate::pieces::Piece;

#[derive(Copy, Clone)]
pub struct Square {
    pub piece: Option<Piece>,
    pub square_colour: char
}


impl Square {
    pub fn new() -> Square {
        Square {
            piece: None,
            square_colour: 'w'
        }
    }

    // pub fn set_piece(mut self, new_piece: Option<Piece>) {
    //     self.piece = new_piece;
    // }

    pub fn change_colour(mut self) -> char {
        if self.square_colour == 'w' {
            return 'b';
        }
        return 'w';
    }

    pub fn check_empty(self) {
        match self.piece {
            Some(x) => println!("{x}"),
            None => println!("None.")
        }
    }
    pub fn get_square_colour(self) -> char{
        self.square_colour
    }
}
