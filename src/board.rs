use array_macro::array;
use crate::pieces::Piece;
use crate::square::Square;

const WIDTH: usize = 8;

pub struct Board {
    grid: [[Square; WIDTH]; WIDTH],
    current_player: char
}

impl Board {
    pub fn new() -> Board {
        let s = Square::new();
        Board {
            grid: array![array![Square::new(); WIDTH]; WIDTH],
            current_player: 'w'
        }

    }

    pub fn display_board(self) -> String{
        let mut str = String::from("");
        for i in self.grid{
            for ii in i {
                str.push(ii.get_square_colour());
                str.push_str("  ");
            }
            str.push_str("\n");
        }
        str
    }

    pub fn add_pieces(self) {
        (1..8usize)
            .step_by(5)
            .for_each(|x| {
                (0..WIDTH)
                    .for_each(|y| {
                    });
            })

    }

    pub fn add_colours(&mut self) {
        /*
        (1..=64usize)
            .step_by(2)
            .for_each(|i| {
                let x: usize = i / WIDTH;
                let y: usize = i % WIDTH;

                self.grid[x][y].square_colour = self.grid[x][y].clone().change_colour();
            });
        */

        (0..8usize)
            .step_by(2)
            .for_each(|x| {
                (1..WIDTH)
                    .step_by(2)
                    .for_each(|y| {
                        self.grid[x][y].square_colour = self.grid[x][y].clone().change_colour();
                    });

                (0..WIDTH)
                    .step_by(2)
                    .for_each(|y| {
                        self.grid[x + 1][y].square_colour = self.grid[x + 1][y].clone().change_colour();
                    });
            })
    }

}