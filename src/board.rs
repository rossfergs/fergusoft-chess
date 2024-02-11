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
            let mut grid = array![array![Square::new(); WIDTH]; WIDTH];
            //sets the square colour :D
            for x in (0..8).step_by(2) {
                for y in (1..WIDTH).step_by(2) {
                    grid[x][y].square_colour = grid[x][y].clone().change_colour();
                }

                for y in (0..WIDTH).step_by(2) {
                    grid[x + 1][y].square_colour = grid[x + 1][y].clone().change_colour();
                }
            }



            
            let tokens: Vec<&str> = fen.split('/').collect();
            for i in 0..8{
                let mut col = 0;
                for char in tokens[i].chars(){
                    if char.is_digit(10){
                        col += char.to_digit(10).unwrap() as usize;
                    }else{
                        let row = 7 - i;
                        let new_piece = Piece::new(char);
                        grid[row][col].piece = Some(new_piece);
                        col+= 1;
                    }
                    
                }
            }
            //gonna add the piece gen thing here

            Board {
                grid
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
                else {str.push_str(&square.piece.unwrap().to_string()); str.push_str("  ");}
            }
            str.push_str("\n");
        }
        str
    }


}