use std::ops::Deref;
use phf::phf_map;

use crate::types::ChessError;

static PIECETYPES: phf::Map<&'static str, PieceType> = phf_map! {
    'K' => PieceType::King,
    'k' => PieceType::King,
    'Q' => PieceType::Queen,
    'q' => PieceType::Queen,
    'R' => PieceType::Rook,
    'r' => PieceType::Rook,
    'B' => PieceType::Bishop,
    'b' => PieceType::Bishop,
    'N' => PieceType::Knight,
    'n' => PieceType::Knight,
};


#[derive(PartialEq, Debug)]
pub struct Piece {
    piece_type: PieceType,
    // -1 = black, 1 = white. Helps with pawn movement math
    team: i8,
}

impl Piece {
    //used for reading positions, where each piece has a team
    fn from(c: char) -> Result<Self, ChessError> {
        if let Some(piece_type) = PIECETYPES.get(c) {
            Ok(Piece {
                piece_type: piece_type.clone(),
                team: match c.is_uppercase() {
                    true => 1,
                    false => -1,
                },
            })
        } else {
            Err(ChessError)
        }
    }
}




impl Deref for Piece {
    type Target = PieceType;

    fn deref(&self) -> &Self::Target {
        &self.piece_type
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    //used for reading moves, where the piece's team is implied
    pub fn from(c: char) -> Result<Self, ChessError> {
        let res = match c {
                        _ => Err(ChessError::NotationError),
        };
        res
    }
}
