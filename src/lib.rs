pub struct ChessBoard{
    board_size: usize,

    pub white_move: bool,
    move_number: usize,

    // make rollbackable if undo
    pub board: Vec<[[char; 8];8]>,
    long_castle: Vec<[bool; 2]>, // black, white
    short_castle: Vec<[bool; 2]>,
    en_passant: Vec<i32>,
}

#[derive(PartialEq)]
pub enum GameState{
    InProgress,
    Draw, 
    Checkmate
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
            board_size: 8, // does not work over 9 (because of move representation)

            white_move: true,
            move_number: 0,

            board: vec![initial_board],
            long_castle: vec![[true, true]], // black, white
            short_castle: vec![[true, true]],
            en_passant: vec![-2],
        }
        
    }


    pub fn print_board(&self) {
        print!("   ");
        for col in 0..self.board[self.move_number][0].len(){
            print!("{} ", (col as u8 + b'a') as char);
        }
        println!();

        for row_index in 0..self.board[self.move_number].len(){
            print!("{}  ", 8-row_index);

            for square in self.board[self.move_number][row_index] {
                print!("{} ", square);
            }
            println!();
        }
    }

    fn find_king(&self) -> (i32, i32){
        for r in 0..self.board_size{
            for c in 0..self.board_size{
                if self.board[self.move_number][r][c] == '.' {continue;}
                if self.my_color(r, c) {continue;}
                let mut piece: char = self.board[self.move_number][r][c];
                piece = piece.to_lowercase().next().unwrap_or(piece);
                
                if piece == 'k' {
                    return (c as i32, r as i32);   
                }
            }
        }
        return (-1, -1);
    }

    fn is_check(&mut self, opponent: bool) -> bool{
        let (king_x, king_y) = self.find_king();
        if !opponent{self.unsafe_make_move(self.coordinate_to_move(king_x, king_y, king_x, king_y));}
        let (king_x, king_y) = self.find_king();
        let opp_moves: Vec<String> = self.get_all_moves();
        for opp_move in opp_moves{
            let (x, y, new_x, new_y) = self.move_to_coordinate(&opp_move);
            if new_x == king_x && new_y == king_y{
                let mut piece: char = self.board[self.move_number][y as usize][x as usize]; // tror det är detta håll
                piece = piece.to_lowercase().next().unwrap_or(piece);
                if piece == 'p' && new_x == x {continue;}
                if !opponent{self.undo_move();}
                return true;
            }
        }
        if !opponent{self.undo_move();}
        return false;
    }

    pub fn current_gamestate(&mut self) -> GameState{
        let moves: Vec<String> = self.get_moves();
        println!("gs len: {}", moves.len());
        if moves.len()==0 {
            if self.is_check(false){return GameState::Checkmate}
            else {return GameState::Draw}
        }else{return GameState::InProgress;}
    }

    fn my_color(&self, r:usize, c:usize) -> bool{
        return (self.white_move ^ (self.board[self.move_number][r][c]>='a')) == true;
    }

    fn get_all_moves(&self) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();

        for r in 0..self.board_size{
            for c in 0..self.board_size{
                if self.board[self.move_number][r][c] == '.' {continue;}
                if !self.my_color(r, c) {continue;}
                let mut piece: char = self.board[self.move_number][r][c];
                piece = piece.to_lowercase().next().unwrap_or(piece);
                
                if piece == 'q' {moves.extend(self.get_queen_moves(c as i32, r as i32))};
                if piece == 'r' {moves.extend(self.get_rook_moves(c as i32, r as i32))};
                if piece == 'b' {moves.extend(self.get_bishop_moves(c as i32, r as i32))};
                if piece == 'n' {moves.extend(self.get_knight_moves(c as i32, r as i32))};
                if piece == 'k' {moves.extend(self.get_king_moves(c as i32, r as i32))};
                if piece == 'p' {moves.extend(self.get_pawn_moves(c as i32, r as i32))};
            }
        }
        return moves;
    }

    pub fn get_moves(&mut self) ->  Vec<String>{
        let all_moves: Vec<String> = self.get_all_moves();
        let mut moves: Vec<String> = Vec::new();

        let mut long_castle_possible: bool = false;
        let mut short_castle_possible: bool = false;
        for played_move in &all_moves{
            self.unsafe_make_move(played_move.clone());

            let (king_x, king_y) = self.find_king();
            let mut is_possible: bool = !self.is_check(true);
            let (x, y, new_x, new_y) = self.move_to_coordinate(&played_move);
            if (king_x == new_x && king_y == new_y) && ((new_y-y==2 && !short_castle_possible)||(new_y-y==-2 && !long_castle_possible)) {is_possible = false;}
            if king_x == new_x && king_y == new_y && x == new_x && new_y-y==1 && is_possible {short_castle_possible = true}
            if king_x == new_x && king_y == new_y && x == new_x && new_y-y==-1 && is_possible {long_castle_possible = true}
            if is_possible{moves.push(played_move.clone());}

            self.undo_move();
        }

        return moves;
    }

    fn coordinate_to_move(&self, x: i32, y: i32, new_x: i32, new_y: i32) -> String{
        return format!("{}{}{}{}", ((x as u8)+b'a') as char, 8-y, ((new_x as u8)+b'a') as char, 8-new_y);
    }

    fn move_to_coordinate(&self, move_str: &str) -> (i32, i32, i32, i32) {
        let chars: Vec<char> = move_str.chars().collect();
        
        let x = (chars[0] as u8 - b'a') as i32;
        let y = 8 - chars[1].to_digit(10).unwrap() as i32;
        let new_x = (chars[2] as u8 - b'a') as i32;
        let new_y = 8 - chars[3].to_digit(10).unwrap() as i32;
        (x, y, new_x, new_y)
    }
    

    fn get_moves_direction(&self, x: i32, y: i32, dirs: Vec<(i32, i32)>, distance: i32) -> Vec<String> {
        let mut moves: Vec<String> = Vec::new();
        
        for dir in dirs {
            for i in 1..(distance+1) {
                let new_x = x + dir.0 * (i as i32);
                let new_y = y + dir.1 * (i as i32);

                if new_x < 0 || new_x >= self.board_size as i32 || new_y < 0 || new_y >= self.board_size as i32 {break;}
                
                if self.board[self.move_number][new_y as usize][new_x as usize] != '.' {
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

        if self.short_castle[self.move_number][self.white_move as usize]{
            if self.board[self.move_number][y as usize][(x+1) as usize] == '.' && self.board[self.move_number][y as usize][(x+2) as usize] == '.'{
                moves.push(self.coordinate_to_move(x, y, x+2, y));
            }
        }
        if self.long_castle[self.move_number][self.white_move as usize]{
            if self.board[self.move_number][y as usize][(x-1) as usize] == '.' && self.board[self.move_number][y as usize][(x-2) as usize] == '.'{
                moves.push(self.coordinate_to_move(x, y, x-2, y));
            }
        }
        return moves
    }

    fn get_pawn_moves(&self, x:i32, y:i32) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        let mut forward: i32 = -1;
        if !self.white_move {forward = 1;}
        if self.board[self.move_number][(y+forward) as usize][x as usize] == '.'{
            if (self.white_move && y == 1) || (!self.white_move && y == 6){
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "n")); // can make more clean
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "b"));
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "q"));
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, x, y+forward), "r"));

            } else{
                moves.push(self.coordinate_to_move(x, y, x, y+forward));
                if ((self.white_move && y == 6) || (!self.white_move && y == 1)) && self.board[self.move_number][(y+forward*2) as usize][x as usize] == '.'{
                    moves.push(self.coordinate_to_move(x, y, x, y+forward*2));
                }
            }
        }
        for side_x in [x-1, x+1]{
            if side_x < 0 || side_x >= 8 {continue;}
            if side_x == self.en_passant[self.move_number] && ((self.white_move && y == 3) || (!self.white_move && y == 4)){
                println!("{} {} {}", side_x,  self.en_passant[self.move_number], self.move_number);
                moves.push(format!("{}{}", self.coordinate_to_move(x, y, side_x, y+forward), 'e'));
            }
            if self.board[self.move_number][(y+forward) as usize][side_x as usize] == '.'{continue;}
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
    
    pub fn undo_move(&mut self){
        if self.move_number == 0{
            panic!("Error: Tried to undo move 0");
        }
        self.white_move = !self.white_move;
        self.move_number -= 1;

        self.board.pop(); // unefficient
        self.long_castle.pop();
        self.short_castle.pop();
        self.en_passant.pop();
    }

    pub fn unsafe_make_move(&mut self, played_move: String){
        let (x, y, new_x, new_y) = self.move_to_coordinate(&played_move);

        // unefficient
        if let Some(last_board) = self.board.last().cloned(){
            self.board.push(last_board);
        }
        if let Some(last_long_castle) = self.long_castle.last().cloned(){
            self.long_castle.push(last_long_castle);
        }
        if let Some(last_short_castle) = self.short_castle.last().cloned(){
            self.short_castle.push(last_short_castle);
        }
        
        self.move_number += 1;
        let mut piece: char = self.board[self.move_number][y as usize][x as usize]; // tror det är detta håll
        piece = piece.to_lowercase().next().unwrap_or(piece);
        if piece == 'k'{self.long_castle[self.move_number][self.white_move as usize] = false; self.short_castle[self.move_number][self.white_move as usize] = false;}
        if piece == 'r' && x == 0{self.long_castle[self.move_number][self.white_move as usize] = false;}
        if piece == 'r' && x == 7{self.short_castle[self.move_number][self.white_move as usize] = false;}

        
        if piece == 'p' && (y-new_y).abs() == 2{
            self.en_passant.push(x);  
        }else{
            self.en_passant.push(-2);
        }
        

        self.board[self.move_number][new_y as usize][new_x as usize] = self.board[self.move_number][y as usize][x as usize];  
        self.board[self.move_number][y as usize][x as usize] = '.' as char;


        if played_move.len() == 5{
            if let Some(char_at_index) = played_move.chars().nth(4) {
                if char_at_index == 'e'{
                    self.board[self.move_number][y as usize][new_x as usize] = '.' as char;
                }else{
                    self.board[self.move_number][new_y as usize][new_x as usize] = char_at_index;
                    if self.white_move{
                        self.board[self.move_number][new_y as usize][new_x as usize] = ((self.board[self.move_number][new_y as usize][new_x as usize] as u8)+32) as char
                    }
                }
            } 
        }

        
        self.white_move = !self.white_move;
    }

    pub fn make_move(&mut self, played_move: String){
        let moves: Vec<String> = self.get_moves();
        if !moves.contains(&played_move){
            panic!("Error: Move was not playable");
        }
        self.unsafe_make_move(played_move);
    }


}