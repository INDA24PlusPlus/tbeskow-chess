mod lib;
use {lib::{ChessBoard, GameState}, std::io::{self, prelude::*}};
// Ggez
fn main() {
    let mut board = ChessBoard::new();
    loop{
        let gamestate:GameState = board.current_gamestate();
        if gamestate == GameState::Checkmate {println!("Checkmate");}
        if gamestate == GameState::Draw {println!("Draw");}
        board.print_board();
        let moves = board.get_moves();
        println!("{}", moves.len());
        for mo in moves{
            println!("{}", mo);
        }
        let mut lines = io::stdin().lock().lines();

        if let Some(Ok(line)) = lines.next(){
            let inp: Vec<&str> = line.split_whitespace().collect();
            let played_move: String = inp[0].to_string();
            board.make_move(played_move);
        }
    }
}
