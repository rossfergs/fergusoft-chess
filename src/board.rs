use array_macro::array;
use std::vec::Vec;
use crate::pieces::Piece;
use crate::square::Square;

const WIDTH: usize = 8;

pub struct Board {
    grid: [[Square; WIDTH]; WIDTH],
    //current player is stored at the end of the fen notation
}

impl Board {
    pub fn new() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let tokens: Vec<&str> = fen.split('/').collect();
        //gonna add the piece gen thing here

        let mut new_grid = array![array![Square::new(); WIDTH]; WIDTH];
        let mut new = Board {
            grid: {
                new_grid
            }
        };
        // Sets the square colour :D (Split into separate function for readability)
        new.add_colours();
        return new;
    }

    fn populate(&mut self, input_position: Vec<&str>) {
        (0..WIDTH)
            .for_each(|i| {
                let mut col: usize = 0;
                input_position[i]
                    .chars()
                    .for_each(|c| {
                        if c.is_digit(10) {
                            col += c
                                .to_digit(10)
                                .expect("Error in reading pieces onto board") as usize;
                        } else {
                            let row = 7 - i;
                            let new_piece = Piece::new(c);
                            self.grid[row][col].piece = Some(new_piece);
                            col += 1;
                        }
                    })
            });
    }

    fn add_colours(&mut self) {
        for x in (0..8).step_by(2) {
            (1..WIDTH).step_by(2).for_each(|y| {
                self.grid[x][y].square_colour = self.grid[x][y].clone().change_colour();
            });

            (0..WIDTH).step_by(2).for_each(|y| {
                self.grid[x + 1][y].square_colour = self.grid[x + 1][y].clone().change_colour();
            });
        }
    }

    ///@brief displays each row with new line for each (and some padding)
    pub fn display_board(self) -> String{
        let mut str = String::from("");
        for row in self.grid{
            for square in row {
                if square.piece == None {
                    str.push(square.get_square_colour());
                    str.push_str("  ");
                }
                else {str.push_str(&square.piece.unwrap().to_string());}
            }
            str.push_str("\n");
        }
        str
    }


}