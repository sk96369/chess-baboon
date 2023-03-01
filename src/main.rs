use std::io::{stdin, stdout, Write};
use chessb::{types::Move, board::BoardState};

fn main() {
    let search_depth = 5;
    let mut turn = 1;

    let mut board = BoardState::new();
    let ai_move = board.next_move(search_depth);
    println!("Computer move for turn {turn}: {}", ai_move);
    let res = board.make_move(ai_move);
    match res {
        Ok(new_state) => {
            board = new_state;
            turn += 1;
        },
        Err(e) => println!("Invalid move"),
    }
    let mut player_move_str = String::new();
    print!("Make a move:");
    let _=stdout().flush();
    stdin().read_line(&mut player_move_str).expect("Not a valid input");
    player_move_str = player_move_str.chars()
        .filter(|s| !s.is_whitespace())
        .collect();

    let player_move = Move::from(player_move_str);
    let res = board.make_move(player_move);
}
