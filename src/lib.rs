pub struct ChessBoard{
    board: [[char; 8];8],
    white_move: bool,
    board_size: usize,
}

impl ChessBoard{

    pub fn new() ->Self{
        let initial_board: [[char; 8]; 8] = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ];

        ChessBoard{
            board: initial_board,
            white_move: true,
            board_size: 8, // does not work over 9 (because of move representation)
        }
    }


    pub fn print_board(&self){
        for row in &self.board{
            for square in row{
                print!("{}",square);
            }
            println!();
        }
    }

    pub fn get_moves(&self) ->  Vec<String>{
        let mut moves: Vec<String> = Vec::new();




        return moves
    }

    pub fn make_move(&self, played_move: String){
        // remove illegal moves
        // make move
    }


}