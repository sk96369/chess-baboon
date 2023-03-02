use crate::types::ChessError;

pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    fn from(item: char) -> Result<Self, ChessError> {
        match item {
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'R' => Ok(Self::Rook),
            'B' => Ok(Self::Bishop),
            'N' => Ok(Self::Knight),
            _ => Err(ChessError::NotationError),
        }
    }
}
