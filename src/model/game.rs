#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece {
    None,
    Red,
    Black,
}

pub struct GameState {
    pub board: [[BoardPiece; 5]; 5],
}

pub fn make_blank_board() -> [[BoardPiece; 5]; 5] {
    [[BoardPiece::None; 5]; 5]
}

impl GameState {
    pub fn jumble_board(&mut self) {
        self.board[1][0] = BoardPiece::Red;
        self.board[2][0] = BoardPiece::Black;
    }

    pub fn print_board(&self) {
        let mut label: String;

        for row in 0..5 {
            for col in 0..5 {
                match self.board[row][col] {
                    BoardPiece::None => {
                        label = "-".to_string();
                    }
                    BoardPiece::Red => {
                        label = "R".to_string();
                    }
                    BoardPiece::Black => {
                        label = "B".to_string();
                    }
                };

                print!("{}", label);
            }
            println!();
        }
        println!();
    }

    pub fn handle_click(&mut self, row: usize, col: usize) {
        if row > 4 || col > 4 {
            return;
        }

        self.board[row][col] = BoardPiece::Red;
    }
}
