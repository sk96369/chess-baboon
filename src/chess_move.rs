pub struct Move {
    piece: Piece,
    move_str: String,
    start_col: Option<i8>,
    start_row: Option<i8>,
    end: Option<ColRow>,
    takes: bool,
}

impl From<String> for Move {
    fn from(value: String) -> Result<Self, NotationError> {
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

        if let Some(d) = it.next() {
            if let Some(c) = it.next() {
                new_move.end = Some((c - 'a', d - '0'));
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
    Move::from("f4f5")?;
    Move::from("f5xg6")?;
    Move::from("Rb1b7")?;
}

#[test]
fn correct_short_notation() {
    Move::from("f5")?;
    Move::from("fxg6")?;
    Move::from("Rc2")?;
    Move::from("c1=Q")?;
}

#[test]
fn incorrect_notation() {
    Move::from("y4f5")?;
    Move::from("f5xxg6")?;
    Move::from("Rb1b9")?;
    Move::from("Cb1b8")?;
    Move::from("8")?;
    Move::from("b")?;
    Move::from("Q")?;
}
