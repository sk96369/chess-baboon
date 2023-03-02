use crate::{types::{ChessError::{self, NotationError}, ColRow}, piece::Piece::{self, Pawn}};
use std::fmt;

pub struct Move {
    piece: Piece,
    move_str: String,
    start_col: Option<i8>,
    start_row: Option<i8>,
    end: Option<ColRow>,
    takes: bool,
}

impl From<String> for Move {
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
        it.skip_while(|c| &c.is_uppercase() || !&c.is_alphanumeric());

        let d = it.next()?;
        let c = it.next()?;
        new_move.end = Some((c - 'a', d - '0'));

        if let Some(c) = it.next() {
            if c == 'x' {
                new_move.takes = true;
                //If the move takes, but nothing precedes x in the notation, an error is returned
                c = it.next()?;
            }
            if c.is_digit(10) {
                new_move.start_row = Some(c - '0' - 1);
                //If the starting row is specified, a column (for pawn) or a piece type specifier
                //can be assumed
                c = it.next()?;
            }
            if c.is_uppercase() {
                new_move.piece = Piece::from(c)?;
            } else {
                if c.is_lowercase() {
                    new_move.start_col = Some(c - 'a');
                    if let Some(c) = it.next() {
                        new_move.piece = Piece::from(c)?;
                    }
                } else {
                    NotationError
                }
            }

        }

    }
}

impl From<&str> for Move {
    
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.move_str)
    }
}

#[test]
fn correct_notation() {
    assert_ne!(Move::from("f4f5"), NotationError);
    assert_ne!(Move::from("f5xg6"), NotationError);
    assert_ne!(Move::from("Rb1b7"), NotationError);
}

#[test]
fn correct_short_notation() {
    assert_ne!(Move::from("f5"), NotationError);
    assert_ne!(Move::from("fxg6"), NotationError);
    assert_ne!(Move::from("Rc2"), NotationError);
    assert_ne!(Move::from("c1=Q"), NotationError);
}

#[test]
fn incorrect_notation() {
    assert_eq!(NotationError, Move::from("y4f5"));
    assert_eq!(NotationError, Move::from("f5xxg6"));
    assert_eq!(NotationError, Move::from("Rb1b9"));
    assert_eq!(NotationError, Move::from("Cb1b8"));
    assert_eq!(NotationError, Move::from("8"));
    assert_eq!(NotationError, Move::from("b"));
    assert_eq!(NotationError, Move::from("Q"));
}
