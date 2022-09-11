use crate::movegen::movevalue::{MoveValue, MoveValues};
use crate::movegen::MAX_MOVES;
use cozy_chess::Move;
use std::fmt;

/*
    RootMoves to sort the root moves during search (depth=1).
    Internal field is move_values struct,
    but sort and push methods are redifined
*/
#[derive(Debug)]
pub struct RootMoves {
    move_values: MoveValues<MAX_MOVES>,
}

// Display implementation for movevalues
impl fmt::Display for RootMoves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.move_values)?;
        Ok(())
    }
}

/* RootMoves implemnetation */
impl RootMoves {
    // Constructor
    pub fn new() -> RootMoves {
        RootMoves {
            move_values: MoveValues::new(),
        }
    }

    // Returns movevalues list
    pub fn list(&self) -> [MoveValue; MAX_MOVES] {
        self.move_values.list()
    }

    // Returns current index
    pub fn idx(&self) -> usize {
        self.move_values.idx()
    }

    // Returns length of move_values list
    pub fn len(&self) -> usize {
        self.move_values.len()
    }

    // Inserts a new move in the list
    pub fn insert(&mut self, move_value: MoveValue, depth: i32) {
        if depth == 1 {
            self.push(move_value);
        } else {
            self.move_values.replace_next(move_value);
            self.incr_idx(1);
        }
    }

    // Sorts the list and points at the beginning of it
    pub fn sort(&mut self) {
        self.move_values.sort();
        self.set_idx(0);
    }

    // Gets next move_value in the list, but does not increment idx
    pub fn next(&mut self) -> Option<MoveValue> {
        let next = self.move_values.next();
        self.decr_idx(1);
        next
    }

    // Gets next move in the list, but does not increment idx
    pub fn next_move(&mut self) -> Option<Move> {
        let next_move = self.move_values.next_move();
        self.decr_idx(1);
        next_move
    }

    /* Helper methods */

    // Increments index of move_values list
    fn incr_idx(&mut self, incr: usize) {
        self.move_values.incr_idx(incr)
    }

    // Decrements index of move_values list
    fn decr_idx(&mut self, incr: usize) {
        self.move_values.decr_idx(incr)
    }

    // Sets index of move_values list
    fn set_idx(&mut self, idx: usize) {
        self.move_values.set_idx(idx)
    }

    // Push a new move_value at the end of the list
    fn push(&mut self, move_value: MoveValue) {
        self.move_values.push(move_value);
    }

    // pub fn print(&self) {
    //     self.move_values.print();
    // }
}
