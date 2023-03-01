

pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    PawnStart,
    Pawn,
}

impl From for Piece {
    fn from(c: char) -> Result<Self, NotationError> {
        match c {
            'K' => King,
            'Q' => Queen,
            'R' => Rook,
            'B' => Bishop,
            'N' => Knight,
            _ => NotationError,
        }
    }
}
