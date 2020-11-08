//! <b>Outer-Open Gomoku</b> is a board game which is a enchanced version of connect5 (Gomoku).\
//! The game is a two-player game which played on a 15x15 Go board.\
//! Two players take turns placing a move on an empty intersection in this board.\
//! The winner is the first player to form an unbroken chain of five moves horizontally, vertically, or diagonally.\
//! Unlike Gomoku, the first move is required to be placed at the two outer rows or columns of this board.\
//! This program provides an AI playing with Minimax search with alpha-beta pruning.

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::cmp;
use std::time::{Instant};

// types

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
    Empty,
    Border,
}

type Square = i32; 
type Move = i32;
type Side = Color;
type Piece = Color;

// constants

const FILE_SIZE: i32 = 15;
const RANK_SIZE: i32 = 15;
const SQUARE_SIZE: i32 = (FILE_SIZE + 1) * (FILE_SIZE + 4) + 16 + 4;

const EVAL_INF: i32 = FILE_SIZE * RANK_SIZE * 100;
const MOVE_NONE: Move = -1;
const SCORE_NONE: i32 = -EVAL_INF - 1;

/// DIRECTION 0: left to right\
/// DIRECTION 1: top to bottom\
/// DIRECTION 2: top left to bottom right\
/// DIRECTION 3: top right to bottom left 
const DIRECTION: [[i32; 5]; 4] = [ [1, 2, 3, 4, 5],
                                   [1 * (FILE_SIZE + 1), 2 * (FILE_SIZE + 1), 3 * (FILE_SIZE + 1), 4 * (FILE_SIZE + 1), 5 * (FILE_SIZE + 1)],
                                   [1 * (FILE_SIZE + 2), 2 * (FILE_SIZE + 2), 3 * (FILE_SIZE + 2), 4 * (FILE_SIZE + 2), 5 * (FILE_SIZE + 2)],
                                   [1 * (FILE_SIZE + 0), 2 * (FILE_SIZE + 0), 3 * (FILE_SIZE + 0), 4 * (FILE_SIZE + 0), 5 * (FILE_SIZE + 0)]];

// variables

static mut ENDGAME: bool = false;

// structures

/// Use one-dimensional array to store the board. The position 0 is top left.\
/// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  <b>15</b>\
/// 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  <b>31</b>\
/// ... \
/// position 15, 31, ... are Borders.\
/// position 0 is file 0, rank 0.\
/// position 17 is file 1, rank 1.

pub struct Pos { // position
    state: [Color; SQUARE_SIZE as usize],
    p_turn: Side,
}

impl Pos {

    pub fn init(&mut self) { // starting position
        for i in 0..SQUARE_SIZE as usize {
            self.state[i] = Color::Border;
        }

        for rk in 0..RANK_SIZE {
            for fl in 0..FILE_SIZE {
                let sq: Square = square_make(fl, rk);
                self.state[sq as usize] = Color::Empty;
            }
        }

        self.p_turn = Color::Black;
    } 

    pub fn do_move(&mut self, mv: Move) {

        let atk: Side = self.p_turn;
        let def: Side = side_opp(atk);

        match self.p_turn {
            Color::Black => { self.state[mv as usize] = Color::Black; },
            Color::White => { self.state[mv as usize] = Color::White; },
            Color::Empty => {},
            Color::Border => {},
        }

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

/// Use List to store legal moves. 
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
//
fn square_make(fl: i32, rk: i32) -> Square {
    rk * (FILE_SIZE + 1) + fl
}

fn side_opp(sd: Side) -> Side {

    match sd {
        Side::White => Side::Black,
        Side::Black => Side::White,
        Side::Empty => panic!(""),
        Side::Border => panic!(""),
    }
}

fn pos_is_winner(pos : &Pos) -> bool {

   let current_side = side_opp(pos.p_turn);
   check_pattern5(&pos, current_side)
}

fn pos_is_draw(pos : &Pos) -> bool {

    let mut found : bool = true;
        
    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {

            let sq: Square = square_make(fl, rk);
            if  pos.can_play(sq) {
                found = false;
                break;
            }

        if found == false { break;}
        }
    }

    let mut out: bool = false;

    if found == true && !pos_is_winner(pos) { out = true; }

    out
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
                Color::Border => print!("| "),
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

/// AI: use Minimax search with alpha-beta pruning
fn search(pos : &Pos, depth: i32, endgame: i32) -> Move {

    let mut new_depth = depth;

    let empties: i32 = pos.count(Color::Empty);
    if empties <= endgame || new_depth > empties { new_depth = empties; }

    if new_depth == empties { unsafe { ENDGAME = true; } }

    search_real(pos, -EVAL_INF, EVAL_INF, new_depth, 0)
}

fn search_real(pos: &Pos, alpha: i32, beta: i32, depth: i32, ply: i32) -> i32 {

    assert!(-EVAL_INF <= alpha && alpha < beta && beta <= EVAL_INF);
    // leaf?

    if pos_is_winner(&pos) { return -EVAL_INF + ply }

    if pos_is_draw(&pos) { return 0 }

    if depth == 0 {
         return eval(&pos)
    }

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

/// Evaluation function: give different scores to different patterns.
fn eval(pos: &Pos) -> i32 {

    let atk: Side = pos.turn();
    let def: Side = side_opp(atk);

    if check_patternlive4(&pos, def) { return -4096 }
    
    if check_patternlive4(&pos, def) { return 2560 }

    if check_patterndead4(&pos, atk) > 0 { return 2560 }

    let n_c4: i32 = check_patterndead4(&pos, def);
    let n_c3: i32 = check_patternlive3(&pos, def);

    // 4,4
    if n_c4 > 1 { return -2048 }
    // 4,3
    if n_c4 == 1 && n_c3 > 0 { return -3048 }

    // 3,3
    if check_patternlive3(&pos, atk) > 1 { return 2560 }

    if n_c3 > 1 { return -2048 }

    0 
}

/// Check <b>OOOOO</b>
fn check_pattern5(pos: &Pos, sd: Side) -> bool {

    let mut n: i32 = 0;

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

	    for dir in 0..4 { //4 DIRECTION
                let idx0 = sq;
                let idx1 = sq + DIRECTION[dir][0];
                let idx2 = sq + DIRECTION[dir][1];
                let idx3 = sq + DIRECTION[dir][2];
                let idx4 = sq + DIRECTION[dir][3];

                let val0 = pos.state[idx0 as usize];
                let val1 = pos.state[idx1 as usize];
                let val2 = pos.state[idx2 as usize];
                let val3 = pos.state[idx3 as usize];
                let val4 = pos.state[idx4 as usize];

                if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1; }
            }
        }
    }

    if n > 0 { true } else { false }
}

/// Check <b>-OOOO-</b>
fn check_patternlive4(pos: &Pos, sd: Side) -> bool {

    let mut n: i32 = 0;

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            for dir in 0..4 { //4 DIRECTION 
                let idx0 = sq;
                let idx1 = sq + DIRECTION[dir][0];
                let idx2 = sq + DIRECTION[dir][1];
                let idx3 = sq + DIRECTION[dir][2];
                let idx4 = sq + DIRECTION[dir][3];
                let idx5 = sq + DIRECTION[dir][4];

                let val0 = pos.state[idx0 as usize];
                let val1 = pos.state[idx1 as usize];
                let val2 = pos.state[idx2 as usize];
                let val3 = pos.state[idx3 as usize];
                let val4 = pos.state[idx4 as usize];
                let val5 = pos.state[idx5 as usize];

                if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd && val5 == Color::Empty { n += 1; }
            }
        } 
    } 

    if n > 0 { true } else { false }
}

/// Check <b>OOOO_, OOO_O, OO_OO, O_OOO, _OOOO</b>
fn check_patterndead4(pos: &Pos, sd: Side) -> i32 {

    let mut n: i32 = 0;

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            for dir in 0..4 { //4 DIRECTION 
                let idx0 = sq;
                let idx1 = sq + DIRECTION[dir][0];
                let idx2 = sq + DIRECTION[dir][1];
                let idx3 = sq + DIRECTION[dir][2];
                let idx4 = sq + DIRECTION[dir][3];

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
    } 

    n 
}

/// Check <b>-OOO-, -OO-O-, -O-OO-</br>
fn check_patternlive3(pos: &Pos, sd: Side) -> i32 {

    let mut n: i32 = 0;

    for rk in 0..RANK_SIZE {
        for fl in 0..FILE_SIZE {
            let sq : Square = square_make(fl, rk);

            for dir in 0..4 { //4 DIRECTION
                let idx0 = sq;
                let idx1 = sq + DIRECTION[dir][0];
                let idx2 = sq + DIRECTION[dir][1];
                let idx3 = sq + DIRECTION[dir][2];
                let idx4 = sq + DIRECTION[dir][3];
                let idx5 = sq + DIRECTION[dir][4];

                let val0 = pos.state[idx0 as usize];
                let val1 = pos.state[idx1 as usize];
                let val2 = pos.state[idx2 as usize];
                let val3 = pos.state[idx3 as usize];
                let val4 = pos.state[idx4 as usize];
                let val5 = pos.state[idx5 as usize];

                if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == sd && val4 == Color::Empty { n +=1 ; }
                if val0 == Color::Empty && val1 == sd && val2 == sd && val3 == Color::Empty && val4 == sd && val5 == Color::Empty { n += 1; }
                if val0 == Color::Empty && val1 == sd && val2 == Color::Empty && val3 == sd && val4 == sd && val5 == Color::Empty { n += 1; }
            }
        }  
    } 

    n
}

fn main() {

    loop {

        let start = Instant::now();

        println!("Hello, this is Outer-Open Gomoku!");
        println!("Self-playing with search depth = 4");

        let test_state: [Color; SQUARE_SIZE as usize] = [Color::Empty; SQUARE_SIZE as usize];

        let mut test1 = Pos {
            state: test_state,
            p_turn: Color::Black,
        };

        test1.init();
        pos_disp(&test1);

        for i in 0..(FILE_SIZE*RANK_SIZE) {

            let mut next_move: Move = square_make(1,7);
            if i > 0 {  next_move = search(&test1, 4, 8); }

            test1.do_move(next_move);
            pos_disp(&test1);

            if pos_is_end(&test1) { 
                println!("Game over!!!!!!");
                println!("Total play {} moves.\n", i);
                break;
            }
        }

        let duration = start.elapsed();

        println!("Time for this game is: {:?}", duration);
    }
}
