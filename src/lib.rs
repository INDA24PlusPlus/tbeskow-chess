pub struct ChessBoard{
    board: [[char; 8];8],
    white_move: bool,
    board_size: usize,
}

impl ChessBoard{

    pub fn new() ->Self{
        // let initial_board: [[char; 8]; 8] = [
        //     ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        //     ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        //     ['.', '.', '.', '.', '.', '.', '.', '.'],
        //     ['.', '.', '.', '.', '.', '.', '.', '.'],
        //     ['.', '.', '.', '.', '.', '.', '.', '.'],
        //     ['.', '.', '.', '.', '.', '.', '.', '.'],
        //     ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        //     ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        // ];
        let initial_board: [[char; 8]; 8] = [
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'O', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'R', '.', 'r', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
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

    fn my_color(&self, r:usize, c:usize) -> bool{
        return (self.white_move ^ (self.board[r][c]>='a')) == true;
    }

    pub fn get_moves(&self) ->  Vec<String>{
        let mut moves: Vec<String> = Vec::new();

        for r in 0..self.board_size{
            for c in 0..self.board_size{
                if self.board[r][c] == '.' {continue;}
                if !self.my_color(r, c) {continue;}
                let mut piece: char = self.board[r][c];
                piece = piece.to_lowercase().next().unwrap_or(piece);
                // kan göra finare men vem bryr sig
                if piece == 'q' {moves.extend(self.get_queen_moves(r as i32, c as i32))};
                if piece == 'r' {moves.extend(self.get_rook_moves(r as i32, c as i32))};
                if piece == 'b' {moves.extend(self.get_bishop_moves(r as i32, c as i32))};
            }
        }


        return moves
    }

    fn get_moves_direction(&self, x: i32, y: i32, dirs: Vec<(i32, i32)>) -> Vec<String> {
        let mut moves: Vec<String> = Vec::new();

        for dir in dirs {
            for i in 1..self.board_size {
                let new_x = x + dir.0 * (i as i32);
                let new_y = y + dir.1 * (i as i32);

                if new_x < 0 || new_x >= self.board_size as i32 || new_y < 0 || new_y >= self.board_size as i32 {break;}

                if self.board[new_x as usize][new_y as usize] != '.' {
                    if !self.my_color(new_x as usize, new_y as usize) {
                        moves.push(format!("{}{}{}{}", x, y, new_x, new_y));
                    }
                    break; 
                }

                moves.push(format!("{}{}{}{}", x, y, new_x, new_y));
            }
        }

        return moves
    }

    fn get_rook_moves(&self, x: i32, y: i32) -> Vec<String> {
        let dirs: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        return self.get_moves_direction(x, y, dirs);
    }

    fn get_bishop_moves(&self, x:i32, y:i32) -> Vec<String>{ // can lower i32
        let dirs: Vec<(i32, i32)> = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];
        return self.get_moves_direction(x, y, dirs);
    }
    
    fn get_queen_moves(&self, x:i32, y:i32) -> Vec<String>{
        let mut moves = Vec::new();
        moves.extend(self.get_rook_moves(x, y));
        moves.extend(self.get_bishop_moves(x, y));
        return moves
    }

    pub fn make_move(&self, played_move: String){
        // remove illegal moves
        // make move
    }


}