use crate::{piece::Piece, chess_move::Move, types::{ColRow, ChessError}};

pub struct BoardState {
    squares: Vec<Option<Piece>>,
}

impl BoardState {
    pub fn new() -> Self {
        let mut start_state = BoardState {
            squares: vec![],
        };
        start_state.push_back(
    }

    pub fn next_move(&self, search_depth: u8) -> Move {
        
    }

    pub fn make_move(self, chess_move: Move) -> Result<BoardState, ChessError> {
        let new_state = self.clone();
        let mover = new_state.get_square((chess_move.start_col, chess_move.start_row));
        Ok(new_state)
    }

    fn get_squares(&self, loc: ColRow) -> Vec<&Option<Piece>> {
        self.
}
