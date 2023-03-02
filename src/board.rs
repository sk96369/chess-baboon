use crate::{piece::Piece, chess_move::Move, types::{ChessError}};

pub struct BoardState {
    squares: Vec<Option<Piece>>,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            squares: 
        }
    }

    pub fn next_move(&self, search_depth: u8) -> Move {
        Move::from("next_move")
    }

    pub fn make_move(self, chess_move: Move) -> Result<BoardState, ChessError> {
        let new_state = self.clone();
        let mover = new_state.get_square((chess_move.start_col, chess_move.start_row));
        Ok(new_state)
    }

    fn get_squares(&self, ColRow) {
        self.
}

#[test]
fn legal_move_simple() {
    let start_state = BoardState::from("8/8/8/5p2/4P3/8/8/K3k3 w");
    let new_move = Move::from("e4x1");
    assert_ne!(IllegalMoveError, new_move);
}
