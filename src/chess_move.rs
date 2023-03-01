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
            move_str: String::new(),
            start_col: None,
            start_row: None,
            end: None,
            takes: false,
        });
        let mut it = value.chars();
        if let Some(c) = it.next() {
            

                

    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        Move {
            move_str: value.to_string(),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.move_str)
    }
}
