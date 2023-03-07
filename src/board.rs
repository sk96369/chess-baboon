use crate::{piece::Piece, chess_move::Move, types::{ColRow, ChessError}};

pub struct BoardState {
    squares: Vec<Option<Piece>>,
}

impl Clone for BoardState {
    fn clone() -> Self {

    }
}

impl BoardState {
    pub fn new() -> Self {
        let mut start_state = BoardState {
            squares: vec![],
        };
        start_state.push(
    }

    pub fn next_move(&self, search_depth: u8) -> Move {

    }

    pub fn make_move(self, chess_move: Move) -> Result<BoardState, ChessError> {
        let mover_candidates = new_state.get_squares((chess_move.start_col, chess_move.start_row));

        //Clone the state if the move is legal
        let new_state = self.clone();
        Ok(new_state)
    }

    fn get_squares(&self, col: Option<i8>, row: Option<i8>) -> Vec<&Option<Piece>> {
        self.
    }
}
