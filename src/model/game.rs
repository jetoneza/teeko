#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece {
    Red,
    Black,
    None,
}

pub struct GameState {
    pub board: [[BoardPiece; 5]; 5],
    pub current_player: BoardPiece,
    pub pieces_dropped: [i32; 2],
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

        if self.pieces_dropped[self.index_of_piece(self.current_player)] >= 4 {
            return;
        }

        if self.board[row][col] != BoardPiece::None {
            return;
        }

        self.board[row][col] = self.current_player;

        self.next_turn();
    }

    fn next_turn(&mut self) {
        self.pieces_dropped[self.index_of_piece(self.current_player)] += 1;

        self.current_player = if self.current_player == BoardPiece::Red {
            BoardPiece::Black
        } else {
            BoardPiece::Red
        };
    }

    fn index_of_piece(&self, piece: BoardPiece) -> usize {
        (piece == BoardPiece::Red).try_into().unwrap()
    }
}
