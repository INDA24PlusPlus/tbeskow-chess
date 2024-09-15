mod lib;
use lib::ChessBoard;

fn main() {
    let mut board = ChessBoard::new();
    board.print_board();
    let moves = board.get_moves();

}
