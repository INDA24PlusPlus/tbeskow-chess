mod lib;
use {lib::ChessBoard, std::io::{self, prelude::*}};

fn main() {
    let mut board = ChessBoard::new();
    loop{
        board.print_board();
        let moves = board.get_moves();
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
