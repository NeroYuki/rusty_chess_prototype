// An attribute to hide warnings for unused code.
use std::io;

struct Chessboard {
    board : Array2d,
    turn: i8, //1 is black turn, -1 is white turn
}

fn abs(x : i8) -> u8 {
    if x < 0 { (x * (-1)) as u8 } else { x as u8 }
}

const WHITE_PAWN : i8 = -1;
const WHITE_ROOK : i8 = -2;
const WHITE_KNIGHT : i8 = -3;
const WHITE_BISHOP : i8 = -4;
const WHITE_QUEEN : i8 = -5;
const WHITE_KING : i8 = -6;
const BLACK_PAWN : i8 = 1;
const BLACK_ROOK : i8 = 2;
const BLACK_KNIGHT : i8 = 3;
const BLACK_BISHOP : i8 = 4;
const BLACK_QUEEN : i8 = 5;
const BLACK_KING : i8 = 6;
const EMPTY : i8 = 0;

struct Array2d {
    data: Vec<i8>,
    h: u8,
    w: u8
}

fn char2coord(src: &str) -> (u8, u8) {
    src.to_lowercase();
    //println!("{}", src);
    let mut char_it = src.chars();
    let mut a : (u8, u8) = (0, 0);
    //println!("{:?}", char_it);
    a.0 = (char_it.nth(0).unwrap() as u8) - 97;
    a.1 = 7 - ((char_it.nth(0).unwrap() as u8) - 49);
    a
}


impl Chessboard {
    fn new(b: Array2d) -> Chessboard {
        Chessboard { board: b , turn: -1 }
    }
    fn moving_piece(&mut self, src: &str, dst: &str) -> i8 {
        let mut res = 0;
        let src_coord :(u8, u8) = match self.resolve_pos(src) {
            (1, a, b) => (a, b), //if the square has player's piece
            _ => {
                res = -1; //invalid move
                (0, 0)
            } 
        };
        let dst_coord :(u8, u8) = match self.resolve_pos(dst) {
            (0, a, b) => (a, b), //if the square is empty
            (2, a, b) => (a, b), //if the square has an opponent's piece
            _ => {
                res = -1; //invalid move
                (0, 0)
            } 
        };
        if src_coord.0 == dst_coord.0 && src_coord.1 == dst_coord.1 { res = -1; } //0 length move is not allowed
        if res == 0 {
            //println!("{:?} {:?}", src_coord, dst_coord);
            res = match self.apply_rule(src_coord, dst_coord) {
                1 => 0,
                _ => -3, //illegal move
            }
        }
        res
    }
    fn apply_rule(&mut self, src: (u8, u8), dst: (u8, u8)) -> u8 {
        let mut res : u8 = 0;
        let moving_piece = self.board.get_val(src.0, src.1);
        let distance : (i8, i8) = ((dst.0  as i8 - src.0  as i8) , (dst.1 as i8 - src.1 as i8));
        //println!("{:?} {:?} {:?}", src, dst, distance);
        let mut initial_check : i8 = 0;
        let mut path_stack : Vec<(u8, u8)> = Vec::new();
        match moving_piece {
            BLACK_PAWN => {
                //if first move, allow to go 0 - 2, if not allow 0 - 1, if capture allow 1 - 1 move if no obstacle on move
                if src.0 == 1 && distance == (2, 0) { initial_check = 1; }
                else if distance == (1, 0) { initial_check = 1; }
                if self.board.get_val(dst.0, dst.1) < 0 { initial_check = 0; }
                if (distance == (1 , -1) || distance == (1, 1)) && self.board.get_val(dst.0, dst.1) < 0 { initial_check = 1; } 
                ()
            }, 
            WHITE_PAWN => {
                //same as black pawn but all positive
                if src.0 == 6 && distance == (-2, 0) { initial_check = 1; }
                else if distance == (-1, 0) { initial_check = 1; }
                if self.board.get_val(dst.0, dst.1) > 0 { initial_check = 0; }
                if (distance == (-1 , -1) || distance == (-1, 1)) && self.board.get_val(dst.0, dst.1) > 0 { initial_check = 1; } 
                ()
            },
            BLACK_ROOK | WHITE_ROOK => {
                //allow to go 0 - x or x - 0 if no obstacle 
                if distance.0 == 0 || distance.1 == 0 { initial_check = 1; }
                ()
            },
            BLACK_KNIGHT | WHITE_KNIGHT => {
                //allow to go 1 - 2, 2 - 1, -2 - -1, -1 - -2, 1 - -2, -2 - 1, -1 - 2, 2 - -1
                if abs(distance.0) == 1 && abs(distance.1) == 2 || abs(distance.0) == 2 && abs(distance.1) == 1 {initial_check = 1;}
                ()
            },
            BLACK_BISHOP | WHITE_BISHOP => {
                //allow to go x - x, -x - x, x - -x if no obstacle
                if abs(distance.0) == abs(distance.1) { initial_check = 1; }
                ()
            },
            BLACK_QUEEN | WHITE_QUEEN => {
                //allow to go x - x, -x - x, x - -x, 0, x - 0, 0 - x if no obstacle
                if distance.0 == 0 || distance.1 == 0 { initial_check = 1; }
                if abs(distance.0) == abs(distance.1) { initial_check = 1; }
                ()
            },
            BLACK_KING | WHITE_KING => {
                //allow to go anywhere as long as -1 <= x <= 1 if no obstacle  
                if abs(distance.0) <= 1 && abs(distance.1) <= 1 { initial_check = 1; }
                ()
            },
            _ => (),
        }
        if initial_check == 1 {
            self.push_path(&mut path_stack, src, distance);
            //println!("{:?}", path_stack);
            match self.path_check(&mut path_stack) {
                true => {
                    self.board.set_val(src.0, src.1, 0);
                    self.board.set_val(dst.0, dst.1, moving_piece);
                    res = 1;
                    ()
                }
                false => {
                    res = 0;
                    ()
                }
            }
        }
        res
    }

    //checking if the one position is valid (correct format, if square is empty, if square is opponent's piece or own's piece)
    fn resolve_pos(&self, src : &str) -> (i8, u8, u8) {
        
        //example: a6 => column 0, row 5 => (5, 0)
        let mut res : (i8, u8, u8) = (0, 0, 0);
        if src.len() != 2 {
            res.0 = -1 //Invalid input
        }
        else {
            let pos : (u8, u8) = char2coord(src);
            if pos.0 > 7 || pos.1 > 7 { res.0 = -2; } //out of bound input
            else { 
                let piece_in_pos = self.board.get_val(pos.1, pos.0); 
                if piece_in_pos == 0 {res.0 = 0} //empty square 
                else if piece_in_pos * self.turn < 0 {res.0 = 2} //opponent's piece
                else if piece_in_pos * self.turn > 0 {res.0 = 1} //own piece
                res.1 = pos.1; res.2 = pos.0;
            }
        }
        res
    }
    fn push_path(&self, path :&mut Vec<(u8, u8)>, src: (u8, u8), distance: (i8, i8)) {
        let mut distance_it_left = (abs(distance.0), abs(distance.1));
        let mut current_it_pos = src;
        if distance_it_left.0 > 0 {distance_it_left.0 -= 1;} //ignore the last step check because src pos_resolve already determine if the piece can move there
        if distance_it_left.1 > 0 {distance_it_left.1 -= 1;}
        while distance_it_left.0 > 0 || distance_it_left.1 > 0 {
            if distance_it_left.0 > 0 {
                if distance.0 > 0 {current_it_pos.0 += 1;}
                else {current_it_pos.0 -= 1;}
                distance_it_left.0 -= 1;
            }
            if distance_it_left.1 > 0 {
                if distance.1 > 0 {current_it_pos.1 += 1;}
                else {current_it_pos.1 -= 1;}
                distance_it_left.1 -= 1;
            }
            path.push(current_it_pos);
        }
        
    }
    
    fn path_check(&self, path :&mut Vec<(u8, u8)>) -> bool {
        let mut res = true;
        while !path.is_empty() {
            if self.board.get_val(path.first().unwrap().0, path.first().unwrap().1) != 0 {res = false; break;}
            path.pop();
        }
        res
    }

    fn output(&self) {
        print!("  ");
        for i in 0..self.board.w {
            print!("{} ", (i + 97) as char);
        }
        println!("");
        for i in 0..self.board.h {
            print!("{} ", (8 - i));
            for j in 0..self.board.w {
                match self.board.data[(self.board.h * i + j) as usize] {
                    WHITE_PAWN => { print!("♙ "); () }
                    BLACK_PAWN => { print!("♟ "); () }
                    WHITE_ROOK => { print!("♖ "); () }
                    BLACK_ROOK => { print!("♜ "); () }
                    WHITE_KNIGHT => { print!("♘ "); () }
                    BLACK_KNIGHT => { print!("♞ "); () }
                    WHITE_BISHOP => { print!("♗ "); () }
                    BLACK_BISHOP => { print!("♝ "); () }
                    WHITE_QUEEN => { print!("♕ "); () }
                    BLACK_QUEEN => { print!("♛ "); () }
                    WHITE_KING => { print!("♔ "); () }
                    BLACK_KING => { print!("♚ "); () }
                    _ => { print!("  "); () }
                }
            }
            println!("");
        }
    }
}

impl Array2d {
    fn new(h: u8, w: u8) -> Self {
        let vec_size = h * w;
        let data = vec![0; vec_size as usize];
        Self { data, h, w }
    }
   
    fn get_val(&self, x: u8, y: u8) -> i8 {
        self.data[(self.h * x + y) as usize]
    }
    fn set_val(&mut self, x: u8, y: u8, val: i8) {
        self.data[(self.h * x + y) as usize] = val;
    }
    fn find_val(&mut self, val: i8) -> (bool, u8, u8) {
        let mut res = (false, 0, 0);
        'outer: for i in 0..self.h {
            for j in 0..self.w {
                if self.get_val(i, j) == val {
                    res = (true, i, j);
                    break 'outer;
                }
            }
        }
        res
    }
}

fn main() {
    let mut a : Array2d = Array2d::new(8, 8);
    //as static array
    let board_patt : [[i8; 8]; 8] = [[2, 3, 4, 5, 6, 4, 3, 2],
                                    [1, 1, 1, 1, 1, 1, 1, 1],
                                    [0, 0, 0, 0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0, 0, 0, 0],
                                    [-1,-1,-1,-1,-1,-1,-1,-1],
                                    [-2,-3,-4,-5,-6,-4,-3,-2]];
    //copy board_patt to a
    for i in 0..8 {
        for j in 0..8 {
            a.set_val(i, j, board_patt[i as usize][j as usize]);
        }
    }

    let mut cb : Chessboard = Chessboard::new(a); 
    cb.output();
    let mut inp = String::new();
    loop {
        inp.clear();
        if cb.turn == -1 {println!("White move: ", );}
        else {println!("Black move: ", );}
        match io::stdin().read_line(&mut inp) {
            Ok(inp) => (),
            _ => continue,
        }
        let args : Vec<&str> = inp.split_whitespace().collect();
        if args.len() < 2 {
            println!("Invalid command")
        }
        match cb.moving_piece(args[0], args[1]) {
            0 => (),
            _ => continue,
        }
        cb.turn *= -1;
        cb.output();
        if cb.board.find_val(BLACK_KING).0 == false {println!("WHITE WIN!"); break;}
        if cb.board.find_val(WHITE_KING).0 == false {println!("BLACK WIN!"); break;}
    }
}