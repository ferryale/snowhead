use crate::attacks::attack_bb::*;
use crate::types::square::*;
use crate::types::piece::*;
use crate::types::r#move::*;
use crate::types::bitboard::*;
use crate::types::score::*;
use crate::zobrist::*;
use crate::psqt;
use crate::movegen::*;

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
    ss: [Stack; MAX_PLY as usize],
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
            if self.ss[ply].pv[0] != Move::NONE {
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
        ret
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
}