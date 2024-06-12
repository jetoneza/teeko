
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
    history: Vec<PieceDropCommand>,
    history_pos: usize,
}

pub fn make_blank_board() -> [[BoardPiece; 5]; 5] {
    [[BoardPiece::None; 5]; 5]
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: make_blank_board(),
            current_player: BoardPiece::Red,
            pieces_dropped: [0, 0],
            history: Vec::new(),
            history_pos: 0,
        }
    }

    pub fn handle_click(&mut self, row: usize, col: usize) {
        let command = PieceDropCommand {
            row,
            col,
            player: self.current_player,
        };

        if !command.is_valid(self) {
            return;
        }

        if self.history.len() > 0 {
            let elements_to_clear = self.history.len() - (self.history_pos + 1);

            for _ in 0..elements_to_clear {
                self.history.pop();
            }
        }

        command.perform(self);

        self.history.push(command);
        self.history_pos = self.history.len() - 1;
    }

    pub fn redo_action(&mut self) {
        if (self.history_pos + 1) >= self.history.len() {
            return;
        }

        self.history_pos += 1;

        let command = self.history[self.history_pos].copy();
        command.perform(self);
    }

    pub fn undo_action(&mut self) {
        if self.history.len() == 0 {
            return;
        }

        let command = self.history[self.history_pos].copy();
        command.undo(self);

        if self.history_pos == 0 {
            return;
        }

        self.history_pos -= 1;
    }

    fn index_of_piece(&self, piece: BoardPiece) -> usize {
        (piece == BoardPiece::Red).try_into().unwrap()
    }
}

struct PieceDropCommand {
    pub row: usize,
    pub col: usize,
    pub player: BoardPiece,
}

impl PieceDropCommand {
    pub fn perform(&self, game: &mut GameState) {
        game.pieces_dropped[game.index_of_piece(self.player)] += 1;
        game.board[self.row][self.col] = self.player;

        game.current_player = if game.current_player == BoardPiece::Red {
            BoardPiece::Black
        } else {
            BoardPiece::Red
        };
    }

    pub fn undo(&self, game: &mut GameState) {
        if game.pieces_dropped[game.index_of_piece(self.player)] == 0 {
            return;
        }

        game.pieces_dropped[game.index_of_piece(self.player)] -= 1;
        game.board[self.row][self.col] = BoardPiece::None;

        game.current_player = self.player;
    }

    pub fn is_valid(&self, game: &mut GameState) -> bool {
        if self.row > 4 || self.col > 4 {
            return false;
        }

        if game.pieces_dropped[game.index_of_piece(self.player)] >= 4 {
            return false;
        }

        if game.board[self.row][self.col] != BoardPiece::None {
            return false;
        }

        return true;
    }

    pub fn copy(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            player: self.player,
        }
    }
}
