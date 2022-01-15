use crate::attacks::attack_bb::*;
use crate::types::square::*;
use crate::types::piece::*;
use crate::types::r#move::*;
use crate::types::bitboard::*;
use crate::types::score::*;
use crate::zobrist::*;
use crate::psqt;
use crate::movegen::*;
use crate::position::*;
use crate::evaluate::*;
use crate::position::inline::*;
use crate::movepick::*;

use std::io::{self, Write};


#[derive(Debug, Clone, Copy)]
pub struct Stack {
    ply: usize,
    pv: [Move; MAX_PLY as usize],
    pub killers: [Move; 2],
    node_count: u32
}


impl Stack {
    pub fn new(ply: usize) -> Stack {
        Stack {
            ply: ply,
            pv: [Move::NONE; MAX_PLY as usize],
            killers: [Move::NONE; 2],
            node_count: 0
        }
    }
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub ss: [Stack; MAX_PLY as usize],
    pub value: Value,
    root_moves: [ExtMove; MAX_MOVES],
    root_idx: usize,
}

impl Thread {
    pub fn new() -> Thread {

        let mut thread = Thread {
            ss: [Stack::new(0); MAX_PLY as usize],
            value: Value(0),
            root_moves: [ExtMove::new(); MAX_MOVES],
            root_idx: 0,
        };

        thread.init_stacks();
        thread
    }

    pub fn depth(&self) -> usize {

        let mut ret: usize = 0;

        for ply in 0..MAX_PLY as usize {
            if self.ss[ply].pv[0] == Move::NONE {
                ret = ply;
                break;
            }
             
        }

        ret
    }

    pub fn seldepth(&self) -> usize {
        
        let mut ret: usize = 0;
        for ply in 0..MAX_PLY as usize {
            if self.ss[ply].node_count == 0 {
                ret = ply;
                break;
            }
        }
        if ret > 0 { ret-1 } else { 0 } 
    }

    pub fn nodes(&self) -> u32 {
        let mut cnt = 0;
        for ply in 0..MAX_PLY as usize {
            if self.ss[ply].node_count == 0 {
                break;
            }
            cnt += self.ss[ply].node_count;
        }
        cnt
    }

    pub fn pv(&self) -> [Move; MAX_PLY as usize] {
        self.ss[0].pv
    }

    pub fn pv_string(&self) -> String {
        let mut ret = String::new();
        for m in &self.pv() {
            if *m == Move::NONE { break; }
            ret = format!("{} {}", ret, m.to_string(false));
        }
        ret
    }

    pub fn score_cp(&self) -> i32 {
        self.value.0
    }

    // pub fn score(&self) {
    //     self.root_moves
        
    // }

    pub fn info(&self) -> String {

        format!("depth {} seldepth {} nodes {} score cp {} pv{}", 
            self.depth(), self.seldepth(), self.nodes(), self.score_cp(), self.pv_string())
        
    }

    pub fn best_move(&self) -> String {
        let best_move_str = self.pv()[0].to_string(false);
        let ponder_str = self.pv()[1].to_string(false);
        format!("bestmove {} ponder {}", best_move_str, ponder_str)

    }

    pub fn print_info(&self) {
        println!("info {}", self.info());
    }

    pub fn print_best_move(&self) {
        println!("{}", self.best_move());
    }

    fn init_stacks(&mut self) {

        for ply in 0..MAX_PLY as usize {
            self.ss[ply] = Stack::new(ply);
        }

    }

    // fn sort_root_moves(&mut self) {

    //     for ext_move in self.root_moves.iter().enumerate() {
    //         if ext_move
    //     }
    // }

    pub fn search(&mut self, pos: &mut Position, depth: i32) {

        let mut stdout = std::io::stdout();
        //let mut lock = stdout.lock();
        let mut alpha = -Value::INFINITE;
        let beta = Value::INFINITE;
        let mut value = Value::ZERO;
        let ply = 0;
        for curr_depth in 1..depth+1 {
            self.value = search(pos, 0, alpha, beta, curr_depth, self);
            
            self.print_info();

            self.root_moves.sort();
            self.root_idx = 0;



            //println!("{:?}", self.root_moves);

            //writeln!(lock, "info {}", self.info());
            //io::stdout().flush().unwrap();
            if curr_depth < depth {
                self.init_stacks();
            }

        }

        self.print_best_move();

        // let mut best_move_str = format!("bestmove {}", self.pv()[0].to_string(false));
        // let mut ponder_str = format!("ponder {}", self.pv()[1].to_string(false));

        // println!()
        // if depth > 1 {
        //     let ponder_str = format!("ponder {}", self.pv()[1].to_string(false));
        //     best_move_str = format!("{} {}", best_move_str, ponder_str);
        // }

        //writeln!(lock, "{}", best_move_str);
        //io::stdout().flush().unwrap();

    }
}

// fn update_pv(pv: &mut[Move], m: Move, child_pv: &[Move]){
//     pv[0] = m;
//     let mut idx = 0;
//     while child_pv[idx] != Move::NONE {
//         pv[idx+1] = child_pv[idx];
//         idx += 1;
//     }
    
// }

fn update_pv(ss: &mut [Stack], ply: usize, m: Move){
    ss[ply].pv[0] = m;
    let mut idx = 0;
    while ss[ply+1].pv[idx] != Move::NONE {
        ss[ply].pv[idx+1] = ss[ply+1].pv[idx];
        idx += 1;
    }
    
}

fn update_killers(ss: &mut [Stack], ply: usize, m: Move) {
    if m != ss[ply].killers[0]{
        ss[ply].killers[1] = ss[ply].killers[0];
        ss[ply].killers[0] = m;
    }
}


fn search(pos: &mut Position, ply: usize, mut alpha: Value, beta: Value, depth: i32, thread: &mut Thread) -> Value {

    thread.ss[ply].node_count += 1;

    // Checks for 50 rule count and repetition draw. Stalemate is handled later.
    if pos.is_draw(ply as i32) {
        return Value::DRAW;
    }

    if depth == 0 {
        //return evaluate(pos);
        thread.ss[ply].node_count -= 1;
        return qsearch(pos, ply, alpha, beta, 0, thread);
    }

    // let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];

    // let num_moves = generate_legal(&pos, &mut list, 0);
    let mut mp = MovePicker::new(pos, Move::NONE, ply, depth, &mut thread.ss);
    let mut num_legal = 0;
    let root_node = ply == 0;

    loop {

        let m = if root_node && depth > 1 {
            thread.root_moves[thread.root_idx].m
            
        } else { 
            mp.next_move(pos, false)
        };

        // if root_node { 
        //     println!("depth {}, move {}", depth, m.to_string(false));
        // }

        

        if m == Move::NONE { break; }
        if !pos.legal(m) { continue; }
        num_legal += 1;

        pos.do_move(m);

        let value = -search(pos, ply+1, -beta, -alpha, depth-1, thread);

        pos.undo_move(m);


        if value >= beta { // Fail high.
            
            // Store move as killer.
            // No need to check if it is a capture because movepicker already does it.
            update_killers(&mut thread.ss, ply, m); 
            return beta;
        }
        if value > alpha {
            alpha = value;
            update_pv(&mut thread.ss, ply, m);
        }

        if root_node { 
            //println!("depth {}, move {}", depth, m.to_string(false));
            thread.root_moves[thread.root_idx] = ExtMove { m: m, value: value.0};
            thread.root_idx += 1; 
        }

    }

    // If there are no legal moves at this point, it is either checkmate or stalemate
    if num_legal == 0 {
        // Stalemate
        if pos.checkers() == 0 {
            return Value::DRAW;
        } else {
            return mated_in(ply as i32);
        } 
    }

    alpha

}

fn qsearch(pos: &mut Position, ply: usize, mut alpha: Value, beta: Value, depth: i32, thread: &mut Thread) -> Value {
    thread.ss[ply].node_count += 1;

    // Checks for 50 rule count and repetition draw. Stalemate is handled later.
    if pos.is_draw(ply as i32) {
        return Value::DRAW;
    }

    let mut value = evaluate(pos);
    if value >= beta {
        return beta;
    }
    if value > alpha {
        alpha = value;
    }

    let mut mp = MovePicker::new(pos, Move::NONE, ply, depth, &mut thread.ss);
    let mut num_moves = 0;

    // let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];
    
    // if pos.checkers() == 0 {
    //     num_moves = generate(CAPTURES, &pos, &mut list, num_moves);
    //     num_moves = generate(QUIET_CHECKS, &pos, &mut list, num_moves);

    // } else {
    //     num_moves = generate(EVASIONS, &pos, &mut list, num_moves);
    // }
    

    loop {
        let m = mp.next_move(pos, true);
        if m == Move::NONE { break; }
        if !pos.legal(m) { continue; }

        pos.do_move(m);

        value = -qsearch(pos, ply+1, -beta, -alpha, depth-1, thread);

        pos.undo_move(m);

        if value >= beta {
            update_killers(&mut thread.ss, ply, m);
            return beta;
        }
        if value > alpha {
            alpha = value;
        }

    }

    // If there are no moves at this point ans we are in check, it is checkmate, 
    // since all evasions have been generated
    // If there are no moves and we are not in check, it is not necessarily stalemate,
    // since not all moves are generated in qsearch
    if num_moves == 0 && pos.checkers() != 0 {
        return mated_in(ply as i32);
    } 

    alpha

}

