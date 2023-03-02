use crate::types::ChessError;

pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl From for Piece {
    fn from(c: char) -> Result<Self, ChessError> {
        match c {
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'R' => Ok(Self::Rook),
            'B' => Ok(Self::Bishop),
            'N' => Ok(Self::Knight),
            _ => ChessError::NotationError,
        }
    }
}
