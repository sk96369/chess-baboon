use std::{error::Error, fmt};

pub type ColRow = (i8, i8);

#[derive(Debug, Clone, PartialEq)]
pub enum ChessError {
    NotationError,
    IllegalMoveError,
    //Player's responsibility to make sure the moves are unambiguous
    //AmbiguousMoveError,
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self {
            NotationError => "invalid algebraic move notation",
            IllegalMoveError => "illegal move",
            //Player's responsibility to make sure the moves are unambiguous
            //AmbiguousMoveError => "ambiguous move notation",
        };
        write!(f, "{}", msg)
    }
}
