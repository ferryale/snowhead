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
    node_count: u32
}


impl Stack {
    pub fn new(ply: usize) -> Stack {
        Stack {
            ply: ply,
            pv: [Move::NONE; MAX_PLY as usize],
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
            cnt = self.ss[ply].node_count;
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

        format!("depth {} seldepth {} nodes {} pv {}", 
            self.depth(), self.seldepth(), self.nodes(), self.pv_string())
        
    }

    fn init_stacks(&mut self) {

        for ply in 0..MAX_PLY as usize {
            self.ss[ply] = Stack::new(ply);
        }

    }

    pub fn search(&mut self, pos: &mut Position, depth: i32) -> Value {
        let mut alpha = -Value::INFINITE;
        let mut beta = Value::INFINITE;
        search(pos, 0, alpha, beta, depth, self)
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


pub fn search(pos: &mut Position, ply: usize, mut alpha: Value, beta: Value, depth: i32, thread: &mut Thread) -> Value {

    thread.ss[ply].node_count += 1;

    // Checks for 50 rule count and repetition draw. Stalemate is handled later.
    if pos.is_draw(ply as i32) {
        return Value::DRAW;
    }

    if depth == 0 {
        return evaluate(pos);
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
        } else if pos.side_to_move() == WHITE {
            return mated_in(ply as i32);
        } else {
            return mate_in(ply as i32);
        }


    }

    alpha

}

