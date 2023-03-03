use std::ops::Deref;

use crate::types::ChessError;

#[derive(PartialEq, Debug)]
pub struct Piece {
    piece_type: PieceType,
    // -1 = black, 1 = white. Helps with pawn movement math
    team: i8,
}

impl Deref for Piece {
    type Target = PieceType;

    fn deref(&self) -> &Self::Target {
        &self.piece_type
    }
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn from(c: char) -> Result<Self, ChessError> {
        let res = match c {
            'K' => Ok(PieceType::King),
            'Q' => Ok(PieceType::Queen),
            'R' => Ok(PieceType::Rook),
            'B' => Ok(PieceType::Bishop),
            'N' => Ok(PieceType::Knight),
            _ => Err(ChessError::NotationError),
        };
        res
    }
}
