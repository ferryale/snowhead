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
    root_moves: [ExtMove; MAX_MOVES],
}

impl Thread {
    pub fn new() -> Thread {

        let mut thread = Thread {
            ss: [Stack::new(0); MAX_PLY as usize],
            root_moves: [ExtMove::new(); MAX_MOVES]
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

    // pub fn score(&self) {
    //     self.root_moves
        
    // }

    pub fn info(&self) -> String {

        format!("depth {} seldepth {} nodes {} pv{}", 
            self.depth(), self.seldepth(), self.nodes(), self.pv_string())
        
    }

    fn init_stacks(&mut self) {

        for ply in 0..MAX_PLY as usize {
            self.ss[ply] = Stack::new(ply);
        }

    }

    pub fn search(&mut self, pos: &mut Position, depth: i32) {
        let mut alpha = -Value::INFINITE;
        let beta = Value::INFINITE;
        let mut value = Value::ZERO;
        let ply = 0;
        for curr_depth in 1..depth+1 {
            value = search(pos, 0, alpha, beta, curr_depth, self);
            println!("{}", self.info());
            if curr_depth < depth {
                self.init_stacks();
            }

        }

        let mut best_move_str = format!("bestmove {}", self.pv()[0].to_string(false));
        if depth > 1 {
            let ponder_str = format!("ponder {}", self.pv()[1].to_string(false));
            best_move_str = format!("{} {}", best_move_str, ponder_str);
        }

        println!("{}", best_move_str);

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

    let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];

    let num_moves = generate_legal(&pos, &mut list, 0);

    for ext_move in list {
        let m = ext_move.m;
        if m == Move::NONE { break; }

        pos.do_move(m);

        let value = -search(pos, ply+1, -beta, -alpha, depth-1, thread);

        pos.undo_move(m);

        if value >= beta {
            return beta;
        }
        if value > alpha {
            alpha = value;
            update_pv(&mut thread.ss, ply, m);
        }

    }

    // If there are no legal moves at this point, it is either checkmate or stalemate
    if num_moves == 0 {
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

    let mut num_moves = 0;

    let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];
    
    if pos.checkers() == 0 {
        num_moves = generate(CAPTURES, &pos, &mut list, num_moves);
        num_moves = generate(QUIET_CHECKS, &pos, &mut list, num_moves);

    } else {
        num_moves = generate(EVASIONS, &pos, &mut list, num_moves);
    }
    

    for ext_move in list {
        let m = ext_move.m;
        if m == Move::NONE { break; }
        if !pos.legal(m) { continue; }

        pos.do_move(m);

        value = -qsearch(pos, ply+1, -beta, -alpha, depth-1, thread);

        pos.undo_move(m);

        if value >= beta {
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

