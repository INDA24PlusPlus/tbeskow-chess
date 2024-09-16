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
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'O', '.', 'o', '.', '.'],
            ['.', '.', '.', '.', 'P', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        ChessBoard{
            board: initial_board,
            white_move: true,
            board_size: 8, // does not work over 9 (because of move representation)
        }
    }


    pub fn print_board(&self) {
        print!("   ");
        for col in 0..self.board[0].len(){
            print!("{} ", (col as u8 + b'a') as char);
        }
        println!();

        for row_index in 0..self.board.len(){
            print!("{}  ", 8-row_index);

            for square in self.board[row_index] {
                print!("{} ", square);
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
                if piece == 'q' {moves.extend(self.get_queen_moves(c as i32, r as i32))};
                if piece == 'r' {moves.extend(self.get_rook_moves(c as i32, r as i32))};
                if piece == 'b' {moves.extend(self.get_bishop_moves(c as i32, r as i32))};

                if piece == 'n' {moves.extend(self.get_knight_moves(c as i32, r as i32))};
                if piece == 'k' {moves.extend(self.get_king_moves(c as i32, r as i32))};
                if piece == 'p' {moves.extend(self.get_pawn_moves(c as i32, r as i32))};
            }
        }


        return moves
    }

    fn coordinate_to_move(&self, x: i32, y: i32, new_x: i32, new_y: i32) -> String{
        return format!("{}{}{}{}", ((x as u8)+b'a') as char, 8-y, ((new_x as u8)+b'a') as char, 8-new_y);
    }

    fn get_moves_direction(&self, x: i32, y: i32, dirs: Vec<(i32, i32)>, distance: i32) -> Vec<String> {
        let mut moves: Vec<String> = Vec::new();
        
        for dir in dirs {
            for i in 1..(distance+1) {
                let new_x = x + dir.0 * (i as i32);
                let new_y = y + dir.1 * (i as i32);

                if new_x < 0 || new_x >= self.board_size as i32 || new_y < 0 || new_y >= self.board_size as i32 {break;}
                
                if self.board[new_y as usize][new_x as usize] != '.' {
                    if !self.my_color( new_y as usize, new_x as usize) {
                        moves.push(self.coordinate_to_move(x, y, new_x, new_y));
                    }
                    break; 
                }
                moves.push(self.coordinate_to_move(x, y, new_x, new_y));
            }
        }

        return moves
    }

    fn get_rook_moves(&self, x: i32, y: i32) -> Vec<String> {
        let dirs: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        return self.get_moves_direction(x, y, dirs, self.board_size as i32);
    }

    fn get_bishop_moves(&self, x:i32, y:i32) -> Vec<String>{ // can lower i32
        let dirs: Vec<(i32, i32)> = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];
        return self.get_moves_direction(x, y, dirs, self.board_size as i32);
    }
    
    fn get_queen_moves(&self, x:i32, y:i32) -> Vec<String>{
        let mut moves = Vec::new();
        moves.extend(self.get_rook_moves(x, y));
        moves.extend(self.get_bishop_moves(x, y));
        return moves
    }
    
    fn get_knight_moves(&self, x:i32, y:i32) -> Vec<String>{
        let dirs: Vec<(i32, i32)> = vec![(1, 2), (-1, 2), (1, -2), (-1, -2), (2, 1), (-2, 1), (2, -1), (-2, -1)];
        return self.get_moves_direction(x, y, dirs, 1);
    }
    
    fn get_king_moves(&self, x:i32, y:i32) -> Vec<String>{
        let dirs: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];
        let mut moves: Vec<String> = self.get_moves_direction(x, y, dirs, 1);

        // add castle

        return moves
    }

    fn get_pawn_moves(&self, x:i32, y:i32) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        let mut forward: i32 = -1;
        if !self.white_move {forward = 1;}
        if self.board[(y+forward) as usize][x as usize] == '.'{
            if (self.white_move && y == 1) || (!self.white_move && y == 6){
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "n")); // can make more clean
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "b"));
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "q"));
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "r"));

            } else{
                moves.push(self.coordinate_to_move(x, y, x, y+forward));
                if ((self.white_move && y == 6) || (!self.white_move && y == 1)) && self.board[(y+forward*2) as usize][x as usize] == '.'{
                    moves.push(self.coordinate_to_move(x, y, x, y+forward*2));
                }
            }
        }
        for side_x in [x-1, x+1]{
            if self.board[(y+forward) as usize][side_x as usize] == '.'{continue;}
            if self.my_color((y+forward) as usize, side_x as usize) {continue;}
            if (self.white_move && y == 1) || (!self.white_move && y == 6){
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, side_x, y+forward), "n")); // can make more clean
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, side_x, y+forward), "b"));
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, side_x, y+forward), "q"));
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, side_x, y+forward), "r"));

            } else{
                moves.push(self.coordinate_to_move(x, y, side_x, y+forward));
            }
        }

        return moves;
    }
    

    pub fn make_move(&self, played_move: String){
        // remove illegal moves
        // make move
    }


}