use crate::{types::{ChessError::{self, NotationError}, ColRow}, piece::Piece::{self, Pawn}};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub move_str: String,
    pub start_col: Option<i8>,
    pub start_row: Option<i8>,
    pub end: Option<ColRow>,
    pub takes: bool,
}

impl Move {
    fn from(value: String) -> Result<Self, ChessError> {
        let mut new_move = Move {
            piece: Pawn,
            move_str: value,
            start_col: None,
            start_row: None,
            end: None,
            takes: false,
        };

        let mut it = value.chars().rev();
        it.skip_while(|c| c.is_uppercase() || !&c.is_alphanumeric());

        let d = it.next().ok_or(NotationError)?;
        let c = it.next().ok_or(NotationError)?;
        new_move.end = Some((c as i8 - 'a' as i8, d as i8 - '0' as i8));

        if let Some(c) = it.next() {
            if c == 'x' {
                new_move.takes = true;
                //If the move takes, but nothing precedes x in the notation, an error is returned
                c = match it.next() {
                    Some(c) => c,
                    None => return Err(NotationError),
                }
            }
            if c.is_digit(10) {
                new_move.start_row = Some(c as i8 - '0' as i8 - 1);
                //If the starting row is specified, a column (for pawn) or a piece type specifier
                //can be assumed
                c = match it.next() {
                    Some(c) => c,
                    None => return Err(NotationError),
                };
            }
            if c.is_uppercase() {
                new_move.piece = match Piece::char_to_piece(c) {
                    Ok(p) => p,
                    Err(e) => return Err(e),
                }
            } else {
                if c.is_lowercase() {
                    new_move.start_col = Some(c as i8 - 'a' as i8);
                    if let Some(c) = it.next() {
                        new_move.piece = match Piece::char_to_piece(c) {
                            Ok(p) => p,
                            Err(e) => return Err(e),
                        }
                    }
                } else {
                    return Err(NotationError)
                }
            }
        }
        Ok(new_move)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.move_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_notation() {
        assert_ne!(Move::from("f4f5".to_string()).unwrap(), Move {
            piece: Pawn,
            move_str: "f4f5".to_string(),
            start_col: Some(5),
            start_row: Some(3),
            takes: false,
            end: Some((5, 4))
        });
        assert_ne!(Move::from("f5xg6".to_string()).unwrap(), Move {
            piece: Pawn,
            move_str: "f5xg6".to_string(),
            start_col: Some(5),
            start_row: Some(4),
            takes: false,
            end: Some((6, 5))
        });
        assert_ne!(Move::from("Rb1b7".to_string()).unwrap(), Move {
            piece: Piece::Rook,
            move_str: "Rb1b7".to_string(),
            start_col: Some(1),
            start_row: Some(0),
            takes: false,
            end: Some((1, 6))
        });
    }

    #[test]
    fn correct_short_notation() {
        assert_eq!(Move::from("f5".to_string()).unwrap(), Move {
            piece: Pawn,
            move_str: "f5".to_string(),
            start_col: None,
            start_row: None,
            takes: false,
            end: Some((5, 4))
        });
        assert_eq!(Move::from("fxg6".to_string()).unwrap(), Move {
            piece: Pawn,
            move_str: "f5".to_string(),
            start_col: Some(5),
            start_row: None,
            takes: false,
            end: Some((6, 5))
        });
        assert_eq!(Move::from("Rc2".to_string()).unwrap(), Move {
            piece: Piece::Rook,
            move_str: "Rc2".to_string(),
            start_col: None,
            start_row: None,
            takes: false,
            end: Some((5, 5))
        });
        assert_eq!(Move::from("c1=Q".to_string()).unwrap(), Move {
            piece: Pawn,
            move_str: "f5".to_string(),
            start_col: None,
            start_row: None,
            takes: false,
            end: Some((5, 5))
        });
    }

    #[test]
    fn incorrect_notation() {
        assert_eq!(Err(NotationError), Move::from("y4f5".to_string()));
        assert_eq!(Err(NotationError), Move::from("f5xxg6".to_string()));
        assert_eq!(Err(NotationError), Move::from("Rb1b9".to_string()));
        assert_eq!(Err(NotationError), Move::from("Cb1b8".to_string()));
        assert_eq!(Err(NotationError), Move::from("8".to_string()));
        assert_eq!(Err(NotationError), Move::from("b".to_string()));
        assert_eq!(Err(NotationError), Move::from("Q".to_string()));
    }
}
