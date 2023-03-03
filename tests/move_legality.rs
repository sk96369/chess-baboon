#[test]
fn legal_move_simple() {
    let start_state = BoardState::from("8/8/8/5p2/4P3/8/8/K3k3 w");
    let new_move = Move::from("e4x1".to_string());
    assert_ne!(Err(IllegalMoveError), new_move);
}
