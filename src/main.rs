use rand::seq::SliceRandom;
use rand::thread_rng;

use std::cmp;

// types

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
    Empty,
}

type Square = i32; 
type Move = i32;
type Side = Color;
type Piece = Color;

// constants

const FILE_SIZE: i32 = 19;
const RANK_SIZE: i32 = 19;
const SQUARE_SIZE: i32 = (FILE_SIZE + 4) * (FILE_SIZE + 4 * 2 ) + 4;

const EVAL_INF: i32 = FILE_SIZE * RANK_SIZE * 100;
const MOVE_NONE: Move = -1;
const SCORE_NONE: i32 = -EVAL_INF - 1;

const ENDCHECK: [[i32; 4]; 20] = [ [-4, -3, -2, -1],
                                   [-3, -2, -1,  1],
                                   [-2, -1,  1,  2],
                                   [-1,  1,  2,  3],
                                   [ 1,  2,  3,  4],

                                   [1 * (-FILE_SIZE - 4), 2 * (-FILE_SIZE - 4), 3 * (-FILE_SIZE - 4), 4 * (-FILE_SIZE - 4)],
                                   [1 * (-FILE_SIZE - 4), 2 * (-FILE_SIZE - 4), 3 * (-FILE_SIZE - 4), 1 * ( FILE_SIZE + 4)],
                                   [1 * (-FILE_SIZE - 4), 2 * (-FILE_SIZE - 4), 1 * ( FILE_SIZE + 4), 2 * ( FILE_SIZE + 4)],
                                   [1 * (-FILE_SIZE - 4), 1 * ( FILE_SIZE + 4), 2 * ( FILE_SIZE + 4), 3 * ( FILE_SIZE + 4)],
                                   [1 * ( FILE_SIZE + 4), 2 * ( FILE_SIZE + 4), 3 * ( FILE_SIZE + 4), 4 * ( FILE_SIZE + 4)],

                                   [1 * (-FILE_SIZE - 5), 2 * (-FILE_SIZE - 5), 3 * (-FILE_SIZE - 5), 4 * (-FILE_SIZE - 5)],
                                   [1 * (-FILE_SIZE - 5), 2 * (-FILE_SIZE - 5), 3 * (-FILE_SIZE - 5), 1 * ( FILE_SIZE + 5)],
                                   [1 * (-FILE_SIZE - 5), 2 * (-FILE_SIZE - 5), 1 * ( FILE_SIZE + 5), 2 * ( FILE_SIZE + 5)],
                                   [1 * (-FILE_SIZE - 5), 1 * ( FILE_SIZE + 5), 2 * ( FILE_SIZE + 5), 3 * ( FILE_SIZE + 5)],
                                   [1 * ( FILE_SIZE + 5), 2 * ( FILE_SIZE + 5), 3 * ( FILE_SIZE + 5), 4 * ( FILE_SIZE + 5)],

                                   [1 * (-FILE_SIZE - 3), 2 * (-FILE_SIZE - 3), 3 * (-FILE_SIZE - 3), 4 * (-FILE_SIZE - 3)],
                                   [1 * (-FILE_SIZE - 3), 2 * (-FILE_SIZE - 3), 3 * (-FILE_SIZE - 3), 1 * ( FILE_SIZE + 3)],
                                   [1 * (-FILE_SIZE - 3), 2 * (-FILE_SIZE - 3), 1 * ( FILE_SIZE + 3), 2 * ( FILE_SIZE + 3)],
                                   [1 * (-FILE_SIZE - 3), 1 * ( FILE_SIZE + 3), 2 * ( FILE_SIZE + 3), 3 * ( FILE_SIZE + 3)],
                                   [1 * ( FILE_SIZE + 3), 2 * ( FILE_SIZE + 3), 3 * ( FILE_SIZE + 3), 4 * ( FILE_SIZE + 3)] ];

const PATTERNFILE4: [i32; 5] = [1, 2, 3, 4, 5];
const PATTERNRANK4: [i32; 5] = [1 * (FILE_SIZE + 4), 2 * (FILE_SIZE + 4), 3 * (FILE_SIZE + 4), 4 * (FILE_SIZE + 4), 5 * (FILE_SIZE + 4)];
const PATTERNDIAL4: [i32; 5] = [1 * (FILE_SIZE + 5), 2 * (FILE_SIZE + 5), 3 * (FILE_SIZE + 5), 4 * (FILE_SIZE + 5), 5 * (FILE_SIZE + 5)];
const PATTERNDIAR4: [i32; 5] = [1 * (FILE_SIZE + 3), 2 * (FILE_SIZE + 3), 3 * (FILE_SIZE + 3), 4 * (FILE_SIZE + 3), 5 * (FILE_SIZE + 3)];

// structures

pub struct Pos { // position
    state: [Color; SQUARE_SIZE as usize],
    p_turn: Side,
    p_last: Move,
}

impl Pos {

    pub fn init(&mut self) { // starting position
        for i in 0..SQUARE_SIZE as usize {
            self.state[i] = Color::Empty;
        }

        self.p_turn = Color::Black;
        self.p_last = square_make(0, 0);
    }

    pub fn do_move(&mut self, mv: Move) {

        let atk: Side = self.p_turn;
        let def: Side = side_opp(atk);

        match self.p_turn {
            Color::Black => self.state[mv as usize] = Color::Black,
            Color::White => self.state[mv as usize] = Color::White,
            Color::Empty => {},
        }

        self.p_last = mv;

        self.p_turn = def; 
    }

    fn turn(&self) -> Side {
        self.p_turn
    }



    pub fn can_play(&self, from: Square) -> bool {

        if self.state[from as usize] == Color::Empty { true } else { false }
    }

    pub fn count(&self, pc: Piece) -> i32 {
        
        let mut n: i32 = 0;

        for rk in 0..RANK_SIZE {
            for fl in 0..FILE_SIZE {
                let sq: Square = square_make(fl, rk);
                if self.state[sq as usize] == pc { n += 1; }
            }
        }
        n
    }

}

pub struct List {  // legal move list

    p_move: [Move; (FILE_SIZE * RANK_SIZE) as usize],
    p_size: i32,
}

impl List {
    
    pub fn clear(&mut self) {
        self.p_size = 0;
    }

    pub fn add(&mut self, mv: Move) {
        self.p_move[self.p_size as usize] = mv;
        self.p_size += 1;
    }

    pub fn size(&self) -> i32 {
        self.p_size
    }

    pub fn shuffle(&mut self) {

        let mut rng = thread_rng();

        let num = self.p_size;
        
        let mut new_move: Vec<Move> = vec![];

        for x in 0..(num as usize) {
            new_move.push(self.p_move[x]);
        }

        new_move.shuffle(&mut rng);

        for x in 0..(self.p_size as usize) {
            self.p_move[x] = new_move[x];
        }
    }
}

// functions

fn square_make(fl: i32, rk: i32) -> Square {
    (rk + 4) * (FILE_SIZE + 4) + (fl + 4)
}

fn square_file(sq: Square) -> i32 {
    sq % (FILE_SIZE + 4) - 4
}

fn square_rank(sq: Square) -> i32 {
    sq / (FILE_SIZE + 4) - 4
}

fn side_opp(sd: Side) -> Side {

    match sd {
        Side::White => Side::Black,
        Side::Black => Side::White,
        Side::Empty => panic!(""),
    }
}

fn pos_is_winner(pos : &Pos) -> bool {

    let current_side = side_opp(pos.p_turn);

    let mut found : bool = true;
    
    for x in 0..20 {
        for y in 0..4 {

            found = true;

            let adj = pos.p_last + ENDCHECK[x][y];
      
            if pos.state[adj as usize] != current_side { found = false; break }
        }
        if found == true { break; } 
    }

    found
}

fn pos_is_draw(pos : &Pos) -> bool {

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {

            let sq: Square = square_make(fl, rk);

            if pos.can_play(sq) {
                return false
            }
        }
    }

    if pos_is_winner(pos) { return false }

    true
}

fn pos_is_end(pos : &Pos) -> bool {

    if pos_is_winner(pos) || pos_is_draw(pos) { 
        true 
    } else {
        false
    }


}

fn pos_disp(pos: &Pos) {

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {

            let sq: Square = square_make(fl, rk);

            match pos.state[sq as usize] {
                Color::Black => print!("# "),
                Color::White => print!("O "),
                Color::Empty => print!("- "),
            }
        }

        println!("");    
    }

    match pos.turn() {
        Color::Black => println!("black to play"),
        Color::White => println!("white to play"),
        _ => (),
    }
}

fn gen_moves(list : &mut List, pos: &Pos) {

    list.clear();

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);
            if pos.can_play(sq) { list.add(sq); }
        }
    }

}

fn search(pos : &Pos, depth: i32, endgame: i32) -> Move {

    let mut new_depth = depth;

    let empties: i32 = pos.count(Color::Empty);
    if empties <= endgame || new_depth > empties { new_depth = empties; }

    search_real(pos, -EVAL_INF, EVAL_INF, new_depth, 0)

}

fn search_real(pos: &Pos, alpha: i32, beta: i32, depth: i32, ply: i32) -> i32 {


    assert!(-EVAL_INF <= alpha && alpha < beta && beta <= EVAL_INF);
    // leaf?

    if pos_is_winner(&pos) { return -EVAL_INF + ply }
    if pos_is_draw(&pos) { return 0 }

    if depth == 0 { return eval(&pos) }

    let p_move_new : [Move; (FILE_SIZE * RANK_SIZE) as usize] = [0; (FILE_SIZE * RANK_SIZE) as usize];

    let mut list = List {
    p_move: p_move_new,
    p_size: 0,
    };

    let mut bm: Move = MOVE_NONE;
    let mut bs: i32  = SCORE_NONE;

    gen_moves(&mut list, &pos);

    // move loop

    if ply == 0 { list.shuffle(); }

    for i in 0..list.size() {

        if bs < beta { 

        let mv: Move = list.p_move[i as usize];

        let mut new_pos = Pos {
            state: pos.state,
            p_turn: pos.p_turn,
            p_last: pos.p_last,
        };

        new_pos.do_move(mv);

        let sc: i32 = -search_real(&new_pos, -beta, -cmp::max(alpha, bs), depth - 1, ply + 1);

        if sc > bs { bm = mv; bs = sc; }
        }
    }

    assert!(bm != MOVE_NONE);
    assert!(bs >= -EVAL_INF && bs <= EVAL_INF);

    if ply == 0 { bm } else { bs } //best move at the root node, best score elsewhere
}

fn eval(pos: &Pos) -> i32 {

    let atk: Side = pos.turn();
    let def: Side = side_opp(atk);

    let check_live4: Side = def; 
    let check_live4_opp: Side = atk; 

    // opp live 4

    if check_patternfile4_once(&pos, check_live4) || 
       check_patternrank4_once(&pos, check_live4) ||
       check_patterndial4_once(&pos, check_live4) ||
       check_patterndiar4_once(&pos, check_live4) { return -4096 }

    // self live 4
    
    if check_patternfile4_once(&pos, check_live4_opp) || 
       check_patternrank4_once(&pos, check_live4_opp) ||
       check_patterndial4_once(&pos, check_live4_opp) ||
       check_patterndiar4_once(&pos, check_live4_opp) { return 2560 }

    // self dead 4

    if check_patternfile4_dead(&pos, check_live4_opp) || 
       check_patternrank4_dead(&pos, check_live4_opp) ||
       check_patterndial4_dead(&pos, check_live4_opp) ||
       check_patterndiar4_dead(&pos, check_live4_opp) { return 2560 }

    let c4f: i32  = check_patternfile4_dead_n(&pos, check_live4);
    let c4r: i32  = check_patternrank4_dead_n(&pos, check_live4);
    let c4dl: i32 = check_patterndial4_dead_n(&pos, check_live4);
    let c4dr: i32 = check_patterndiar4_dead_n(&pos, check_live4);

    let c3f: bool  = check_patternfile3_live(&pos, check_live4);
    let c3r: bool  = check_patternrank3_live(&pos, check_live4);
    let c3dl: bool = check_patterndial3_live(&pos, check_live4);
    let c3dr: bool = check_patterndiar3_live(&pos, check_live4);

    let n_c4: i32 = c4f + c4r + c4dl + c4dr;

    if n_c4 > 1 { return -2048 }

    // opp 4,3

    if n_c4 == 1 && ( c3f || c3r || c3dl || c3dr ) { return -3048 }
    
    // self live 3

    if check_patternfile3_live(&pos, check_live4_opp)
       || check_patternrank3_live(&pos, check_live4_opp)
       || check_patterndial3_live(&pos, check_live4_opp)
       || check_patterndiar3_live(&pos, check_live4_opp) { return 2560 }

    // opp 3,3
    if (c3f && c3r)  || (c3f && c3dl) || (c3f && c3dr) ||
       (c3r && c3dl) || (c3r && c3dr) || (c3dl && c3dr) { return -2048 }

    0 
}

fn check_patternfile4_once(pos: &Pos, sd: Side) -> bool {

    for rk in 0..RANK_SIZE {
        for fl in 0..(FILE_SIZE - 5) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNFILE4[0];
            let idx2 = sq + PATTERNFILE4[1];
            let idx3 = sq + PATTERNFILE4[2];
            let idx4 = sq + PATTERNFILE4[3];
            let idx5 = sq + PATTERNFILE4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patternrank4_once(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 5) {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNRANK4[0];
            let idx2 = sq + PATTERNRANK4[1];
            let idx3 = sq + PATTERNRANK4[2];
            let idx4 = sq + PATTERNRANK4[3];
            let idx5 = sq + PATTERNRANK4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patterndial4_once(pos: &Pos, sd : Side) -> bool {

    for rk in 0..(RANK_SIZE - 5) {
        for fl in 0..(FILE_SIZE - 5) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAL4[0];
            let idx2 = sq + PATTERNDIAL4[1];
            let idx3 = sq + PATTERNDIAL4[2];
            let idx4 = sq + PATTERNDIAL4[3];
            let idx5 = sq + PATTERNDIAL4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patterndiar4_once(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 5) {
        for fl in 5..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAR4[0];
            let idx2 = sq + PATTERNDIAR4[1];
            let idx3 = sq + PATTERNDIAR4[2];
            let idx4 = sq + PATTERNDIAR4[3];
            let idx5 = sq + PATTERNDIAR4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patternfile4_dead(pos: &Pos, sd: Side) -> bool {

    for rk in 0..RANK_SIZE {
        for fl in 0..(FILE_SIZE - 4) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNFILE4[0];
            let idx2 = sq + PATTERNFILE4[1];
            let idx3 = sq + PATTERNFILE4[2];
            let idx4 = sq + PATTERNFILE4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { return true }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { return true }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { return true }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { return true }
        }  
    } 

    false
}

fn check_patternrank4_dead(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNRANK4[0];
            let idx2 = sq + PATTERNRANK4[1];
            let idx3 = sq + PATTERNRANK4[2];
            let idx4 = sq + PATTERNRANK4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { return true }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { return true }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { return true }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { return true }
        }  
    } 

    false
}

fn check_patterndial4_dead(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 0..(FILE_SIZE - 4) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAL4[0];
            let idx2 = sq + PATTERNDIAL4[1];
            let idx3 = sq + PATTERNDIAL4[2];
            let idx4 = sq + PATTERNDIAL4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { return true }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { return true }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { return true }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { return true }
        }  
    } 

    false
}

fn check_patterndiar4_dead(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 4..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAR4[0];
            let idx2 = sq + PATTERNDIAR4[1];
            let idx3 = sq + PATTERNDIAR4[2];
            let idx4 = sq + PATTERNDIAR4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { return true }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { return true }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { return true }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { return true }
        }  
    } 

    false 
}


fn check_patternfile4_dead_n(pos: &Pos, sd: Side) -> i32 {

    let mut n: i32 = 0;

    for rk in 0..RANK_SIZE {
        for fl in 0..(FILE_SIZE - 4) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNFILE4[0];
            let idx2 = sq + PATTERNFILE4[1];
            let idx3 = sq + PATTERNFILE4[2];
            let idx4 = sq + PATTERNFILE4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { n += 1; }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { n += 1; }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { n += 1; }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { n += 1; }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1; }
        }  
    } 

    n
}

fn check_patternrank4_dead_n(pos: &Pos, sd: Side) -> i32 {

    let mut n: i32 = 0;

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNRANK4[0];
            let idx2 = sq + PATTERNRANK4[1];
            let idx3 = sq + PATTERNRANK4[2];
            let idx4 = sq + PATTERNRANK4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { n += 1; }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { n += 1; }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { n += 1; }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { n += 1; }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1; }
        }  
    } 

    n
}

fn check_patterndial4_dead_n(pos: &Pos, sd: Side) -> i32 {

    let mut n: i32 = 0;

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 0..(FILE_SIZE - 4) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAL4[0];
            let idx2 = sq + PATTERNDIAL4[1];
            let idx3 = sq + PATTERNDIAL4[2];
            let idx4 = sq + PATTERNDIAL4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { n += 1; }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { n += 1; }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { n += 1; }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { n += 1; }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1; }
        }  
    } 

    n
}

fn check_patterndiar4_dead_n(pos: &Pos, sd: Side) -> i32 {

    let mut n: i32 = 0;

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 4..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAR4[0];
            let idx2 = sq + PATTERNDIAR4[1];
            let idx3 = sq + PATTERNDIAR4[2];
            let idx4 = sq + PATTERNDIAR4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { n += 1; }
            if val0 == sd && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd { n += 1; }
            if val0 == sd && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd { n += 1; }
            if val0 == sd && val1 == Color::Empty && val2 == sd && val3 == sd && val4 == sd { n += 1; }
            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1; }
        }  
    } 

    n
}


fn check_patternfile3_live(pos: &Pos, sd: Side) -> bool {

    for rk in 0..RANK_SIZE {
        for fl in 0..(FILE_SIZE - 4) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNFILE4[0];
            let idx2 = sq + PATTERNFILE4[1];
            let idx3 = sq + PATTERNFILE4[2];
            let idx4 = sq + PATTERNFILE4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
        }  
    } 

    for rk in 0..RANK_SIZE {
        for fl in 0..(FILE_SIZE - 5) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNFILE4[0];
            let idx2 = sq + PATTERNFILE4[1];
            let idx3 = sq + PATTERNFILE4[2];
            let idx4 = sq + PATTERNFILE4[3];
            let idx5 = sq + PATTERNFILE4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd && val5 == Color::Empty { return true }
            if val0 == Color::Empty && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patternrank3_live(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNRANK4[0];
            let idx2 = sq + PATTERNRANK4[1];
            let idx3 = sq + PATTERNRANK4[2];
            let idx4 = sq + PATTERNRANK4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
        }  
    } 

    for rk in 0..(RANK_SIZE - 5) {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNRANK4[0];
            let idx2 = sq + PATTERNRANK4[1];
            let idx3 = sq + PATTERNRANK4[2];
            let idx4 = sq + PATTERNRANK4[3];
            let idx5 = sq + PATTERNRANK4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd && val5 == Color::Empty { return true }
            if val0 == Color::Empty && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patterndial3_live(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 0..(FILE_SIZE - 4) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAL4[0];
            let idx2 = sq + PATTERNDIAL4[1];
            let idx3 = sq + PATTERNDIAL4[2];
            let idx4 = sq + PATTERNDIAL4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
        }  
    } 

    for rk in 0..(RANK_SIZE - 5) {
        for fl in 0..(FILE_SIZE - 5) {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAL4[0];
            let idx2 = sq + PATTERNDIAL4[1];
            let idx3 = sq + PATTERNDIAL4[2];
            let idx4 = sq + PATTERNDIAL4[3];
            let idx5 = sq + PATTERNDIAL4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd && val5 == Color::Empty { return true }
            if val0 == Color::Empty && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}

fn check_patterndiar3_live(pos: &Pos, sd: Side) -> bool {

    for rk in 0..(RANK_SIZE - 4) {
        for fl in 4..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAR4[0];
            let idx2 = sq + PATTERNDIAR4[1];
            let idx3 = sq + PATTERNDIAR4[2];
            let idx4 = sq + PATTERNDIAR4[3];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { return true }
        }  
    } 

    for rk in 0..(RANK_SIZE - 5) {
        for fl in 5..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            let idx0 = sq;
            let idx1 = sq + PATTERNDIAR4[0];
            let idx2 = sq + PATTERNDIAR4[1];
            let idx3 = sq + PATTERNDIAR4[2];
            let idx4 = sq + PATTERNDIAR4[3];
            let idx5 = sq + PATTERNDIAR4[4];

            let val0 = pos.state[idx0 as usize];
            let val1 = pos.state[idx1 as usize];
            let val2 = pos.state[idx2 as usize];
            let val3 = pos.state[idx3 as usize];
            let val4 = pos.state[idx4 as usize];
            let val5 = pos.state[idx5 as usize];

            if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd && val5 == Color::Empty { return true }
            if val0 == Color::Empty && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd && val5 == Color::Empty { return true }
        }  
    } 

    false 
}


#[cfg(test)]
mod tests {

    use super::*;

   #[test]
    fn test_pos_is_draw() {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Black; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        // fill white to draw 
        for i in (0..19).step_by(2) {
            test1.state[ square_make(0, i) as usize ] = Color::White;
            test1.state[ square_make(1, i) as usize ] = Color::White;
            test1.state[ square_make(2, i) as usize ] = Color::White;
            test1.state[ square_make(3, i) as usize ] = Color::White;
    
            test1.state[ square_make(8, i) as usize ] = Color::White;
            test1.state[ square_make(9, i) as usize ] = Color::White;
            test1.state[ square_make(10,i) as usize ] = Color::White;
            test1.state[ square_make(11,i) as usize ] = Color::White;
        
            test1.state[ square_make(16,i) as usize ] = Color::White;
            test1.state[ square_make(17,i) as usize ] = Color::White;
            test1.state[ square_make(18,i) as usize ] = Color::White;
        }

        for i in (1..19).step_by(2) {
            test1.state[ square_make(4, i) as usize ] = Color::White;
            test1.state[ square_make(5, i) as usize ] = Color::White;
            test1.state[ square_make(6, i) as usize ] = Color::White;
            test1.state[ square_make(7, i) as usize ] = Color::White;
    
            test1.state[ square_make(12,i) as usize ] = Color::White;
            test1.state[ square_make(13,i) as usize ] = Color::White;
            test1.state[ square_make(14,i) as usize ] = Color::White;
            test1.state[ square_make(15,i) as usize ] = Color::White;
        }
            
        assert_eq!( pos_is_draw(&test1), true);
    }

   #[test]
    fn test_pos_is_winner() {

        //test _OOOO
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE );

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 1, rk) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk) as usize ] = Color::Black;
        test1.state[ square_make(fl + 3, rk) as usize ] = Color::Black;
        test1.state[ square_make(fl + 4, rk) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true);
        }

        //test O_OOO
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::White,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 1, rk) as usize ] = Color::White;
        test1.state[ square_make(fl + 1, rk) as usize ] = Color::White;
        test1.state[ square_make(fl + 2, rk) as usize ] = Color::White;
        test1.state[ square_make(fl + 3, rk) as usize ] = Color::White;

        assert_eq!(pos_is_winner(&test1), true );
        }

        //test OO_OO
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::White,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 2, rk) as usize ] = Color::White;
        test1.state[ square_make(fl - 1, rk) as usize ] = Color::White;
        test1.state[ square_make(fl + 1, rk) as usize ] = Color::White;
        test1.state[ square_make(fl + 2, rk) as usize ] = Color::White;

        assert_eq!(pos_is_winner(&test1), true );
        }

        //test OOO_O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::White,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 3, rk) as usize ] = Color::White;
        test1.state[ square_make(fl - 2, rk) as usize ] = Color::White;
        test1.state[ square_make(fl - 1, rk) as usize ] = Color::White;
        test1.state[ square_make(fl + 1, rk) as usize ] = Color::White;

        assert_eq!(pos_is_winner(&test1), true );
        }

        //test _OOOO
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::White,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 1, rk) as usize ] = Color::White;
        test1.state[ square_make(fl - 2, rk) as usize ] = Color::White;
        test1.state[ square_make(fl - 3, rk) as usize ] = Color::White;
        test1.state[ square_make(fl - 4, rk) as usize ] = Color::White;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // _
        // O
        // O
        // O
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 3) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 4) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        // _
        // O
        // O
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 3) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        // O
        // _
        // O
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 2) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        // O
        // O
        // _
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl, rk - 3) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl, rk + 1) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        // O
        // O
        // O
        // _
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::White,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl, rk - 4) as usize ] = Color::White;
        test1.state[ square_make(fl, rk - 3) as usize ] = Color::White;
        test1.state[ square_make(fl, rk - 2) as usize ] = Color::White;
        test1.state[ square_make(fl, rk - 1) as usize ] = Color::White;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // _
        //  O
        //   O
        //    O
        //     O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl + 3, rk + 3) as usize ] = Color::Black;
        test1.state[ square_make(fl + 4, rk + 4) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // _
        //  O
        //   O
        //    O
        //     O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl + 3, rk + 3) as usize ] = Color::Black;
        test1.state[ square_make(fl + 4, rk + 4) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        //  _
        //   O
        //    O
        //     O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 1, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl + 3, rk + 3) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        //  O
        //   _
        //    O
        //     O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 2, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl - 1, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk + 2) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        //  O
        //   O
        //    _
        //     O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 3, rk - 3) as usize ] = Color::Black;
        test1.state[ square_make(fl - 2, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl - 1, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl + 1, rk + 1) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        // O
        //  O
        //   O
        //    O
        //     _
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::White,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 4, rk - 4) as usize ] = Color::White;
        test1.state[ square_make(fl - 3, rk - 3) as usize ] = Color::White;
        test1.state[ square_make(fl - 2, rk - 2) as usize ] = Color::White;
        test1.state[ square_make(fl - 1, rk - 1) as usize ] = Color::White;


        assert_eq!(pos_is_winner(&test1), true );
        }

        //     _
        //    O
        //   O
        //  O
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl - 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl - 2, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl - 3, rk + 3) as usize ] = Color::Black;
        test1.state[ square_make(fl - 4, rk + 4) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }


        //     O
        //    _
        //   O
        //  O
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 1, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl - 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl - 2, rk + 2) as usize ] = Color::Black;
        test1.state[ square_make(fl - 3, rk + 3) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        //     O
        //    O
        //   _
        //  O
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 2, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl + 1, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl - 1, rk + 1) as usize ] = Color::Black;
        test1.state[ square_make(fl - 2, rk + 2) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        //     O
        //    O
        //   O
        //  _
        // O
        for _n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 3, rk - 3) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl + 1, rk - 1) as usize ] = Color::Black;
        test1.state[ square_make(fl - 1, rk + 1) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }

        //     O
        //    O
        //   O
        //  O
        // _
        for n in 0..1000 {

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
            p_last: square_make(5,5),
        };

        let mut rng = rand::thread_rng();

        let rk = rng.gen_range(0, RANK_SIZE);
        let fl = rng.gen_range(0, FILE_SIZE);

        test1.do_move(square_make(fl,rk));

        test1.state[ square_make(fl + 4, rk - 4) as usize ] = Color::Black;
        test1.state[ square_make(fl + 3, rk - 3) as usize ] = Color::Black;
        test1.state[ square_make(fl + 2, rk - 2) as usize ] = Color::Black;
        test1.state[ square_make(fl + 1, rk - 1) as usize ] = Color::Black;

        assert_eq!(pos_is_winner(&test1), true );
        }
    }

}



fn main() {
    println!("Hello, this is connect 6!");

    let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

    let mut test1 = Pos {
        state: test_state,
        p_turn: Color::Black,
        p_last: square_make(5,5),
    };

    test1.init();
    
    pos_disp(&test1);

    for i in 0..(FILE_SIZE*RANK_SIZE) {

    println!("----------------------------------------\n\n\n\n");
    println!("MOVE {}!!!!\n\n\n\n", i);

    let d = 4;
    let e = 4;

    let next_move: Move = search(&test1, d, e);

    println!("next move is {}", next_move);
    println!("file is {}",  square_file(next_move));
    println!("rank is {}",  square_rank(next_move));

    test1.do_move(next_move);

    pos_disp(&test1);

    if pos_is_end(&test1) { 
        println!("Game over!!!!!!");
        break; }
    }



}
