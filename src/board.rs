use crate::types::{Move, ChessError};

pub struct BoardState {}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
        }
    }

    pub fn next_move(&self, search_depth: u8) -> Move {
        Move::from("next_move")
    }

    pub fn make_move(self, chess_move: Move) -> Result<BoardState, ChessError> {
        let new_state = self.clone();
        let start_loc = 
        let mover = new_state.get_square(
        Ok(new_state)
    }

    fn get_square(&self, 
}

#[test]
fn illegal_move_simple() {
    let start_state = BoardState::from("8
    let new_move = Move::from("f5f1");
    assert_eq!(IllegalMoveError, 
