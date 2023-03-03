use crate::types::ChessError;

#[derive(Debug, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub fn char_to_piece(c: char) -> Result<Self, ChessError> {
        let res = match c {
            'K' => Ok(Piece::King),
            'Q' => Ok(Piece::Queen),
            'R' => Ok(Piece::Rook),
            'B' => Ok(Piece::Bishop),
            'N' => Ok(Piece::Knight),
            _ => Err(ChessError::NotationError),
        };
        res
    }
}
