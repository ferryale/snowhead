use crate::movegen::movevalue::{MoveValue, MoveValues};
use crate::movegen::movepick::{MAX_MOVES};
use cozy_chess::{Move};

#[derive(Debug)]
pub struct RootMoves {
    move_values: MoveValues<MAX_MOVES>
}

impl RootMoves {
    pub fn new() -> RootMoves {
        RootMoves {
            move_values: MoveValues::new()
        }
        
    }

    pub fn list(&self) -> [MoveValue; MAX_MOVES] {
        self.move_values.list()
    }

    pub fn list_mut(&mut self) -> [MoveValue; MAX_MOVES] {
        self.move_values.list_mut()
    }

    pub fn idx(&self) -> usize {
        self.move_values.idx()
    }

    pub fn len(&self) -> usize {
        self.move_values.size()
    }

    fn incr_idx(&mut self, incr: usize) {
        self.move_values.incr_idx(incr)
    }

    fn decr_idx(&mut self, incr: usize) {
        self.move_values.decr_idx(incr)
    }

    fn set_idx(&mut self, idx: usize) {
        self.move_values.set_idx(idx)
    }

    // fn set_size(&mut self, size: usize) {
    //     self.move_values.set_size(size)
    // }

    // fn incr_size(&mut self, incr: usize) {
    //     self.move_values.incr_size(incr)
    // }

    fn push(&mut self, move_value: MoveValue) {
        self.move_values.push(move_value);
    }

    pub fn insert(&mut self, move_value: MoveValue, depth: i32) {
        if depth == 1 {
            self.push(move_value);
        } else {
            self.move_values.replace_next(move_value);
            self.incr_idx(1);
        }
    }

    pub fn sort(&mut self) {
        self.move_values.sort();
        self.set_idx(0);
    }

    pub fn next(&mut self) -> Option<MoveValue> {
        let next = self.move_values.next();
        self.decr_idx(1);
        next

    }

    pub fn next_move(&mut self) -> Option<Move> {
        let next_move = self.move_values.next_move();
        self.decr_idx(1);
        next_move
    }

    pub fn print(&self) {
        self.move_values.print();
    }


}