use crate::evaluate::score::Value;
use cozy_chess::{Move, Square};
use std::{cmp, fmt};

/* MoveValue struct */
#[derive(Debug, Clone, Copy, Eq)]
pub struct MoveValue {
    mv: Move,
    value: Value,
}

/* MoveValue implementation */
impl MoveValue {
    // Constructor from move and value
    pub fn new(mv: Move, value: Value) -> MoveValue {
        MoveValue {
            mv: mv,
            value: value,
        }
    }

    // Constructor from i16 value, default (invalid) move
    pub fn from_i16(val: i16) -> MoveValue {
        MoveValue {
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            },
            value: Value(val),
        }
    }

    // Returns move
    pub fn chess_move(&self) -> Move {
        self.mv
    }

    // Returns value
    pub fn value(&self) -> Value {
        self.value
    }
}

/* Default implementation for MoveValue*/
impl Default for MoveValue {
    fn default() -> MoveValue {
        MoveValue {
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            },
            value: Value::ZERO,
        }
    }
}

/* Trait implementations for MoveValue */
impl fmt::Display for MoveValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.mv, self.value.0)
    }
}

impl Ord for MoveValue {
    fn cmp(&self, other: &MoveValue) -> cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for MoveValue {
    fn partial_cmp(&self, other: &MoveValue) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for MoveValue {
    fn eq(&self, other: &MoveValue) -> bool {
        self.value == other.value
    }
}

/*
    MoveValues struct.
    Implements internally an array of movevalues (list),
    with the functionality of a vector.
    Not using a vector on the heap for performance reasons.
*/
#[derive(Debug, Clone, Copy)]
pub struct MoveValues<const N: usize> {
    pub list: [MoveValue; N],
    idx: usize,
    size: usize,
}

// Display implementation for movevalues
impl<const N: usize> fmt::Display for MoveValues<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for idx in 0..self.size {
            write!(f, "{}", self.list[idx])?;
            if idx < self.size - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

/* MoveValues implementation */
impl<const N: usize> MoveValues<N> {
    // Constructor
    pub fn new() -> MoveValues<N> {
        MoveValues {
            list: [MoveValue::default(); N],
            idx: 0,
            size: 0,
        }
    }

    // Adds a move_value at the end of the list
    pub fn push(&mut self, move_value: MoveValue) {
        self.list[self.size] = move_value;
        self.size += 1;
    }

    // Inserts a movevalue by sorting in descending order
    pub fn insert_sort(&mut self, move_value: MoveValue) {
        if self.size == 0 {
            self.push(move_value);
            return;
        }

        let mut inserted = false;
        for j in 0..=self.size {
            // If move_value is better than next in the array
            if move_value > self.list[j] {
                // Shift all values down the list
                for k in (j..self.size).rev() {
                    self.list[k + 1] = self.list[k];
                }
                // Insert it at position j
                self.list[j] = move_value;
                inserted = true;
                break;
            }
        }

        if !inserted {
            self.push(move_value);
            return;
        }

        self.size += 1;
    }

    // Sorts in descending order
    pub fn sort(&mut self) {
        self.list[0..self.size].sort();
        self.list[0..self.size].reverse();
    }

    // Replaces the next value in the list with passed move_value
    pub fn replace_next(&mut self, move_value: MoveValue) {
        self.list[self.idx] = move_value;
    }

    // Merges other at the end of the list
    pub fn extend(&mut self, other: &MoveValues<N>) {
        let new_size = self.size + other.len();
        for j in self.size..new_size {
            self.list[j] = other.list[j - self.size];
        }
        self.size = new_size;
    }

    // Iterator functionality: returns Option<next_value>
    pub fn next(&mut self) -> Option<MoveValue> {
        if self.idx < self.size {
            let next = self.list[self.idx];
            self.idx += 1;
            Some(next)
        } else {
            None
        }
    }

    // Iterator functionality: returns Option<next_move>
    pub fn next_move(&mut self) -> Option<Move> {
        match self.next() {
            Some(move_value) => Some(move_value.chess_move()),
            None => None,
        }
    }

    // Returns the list of movevalues
    pub fn list(&self) -> [MoveValue; N] {
        self.list
    }

    // Returns the length of the list
    pub fn len(&self) -> usize {
        self.size
    }

    // Returns the current idx
    pub fn idx(&self) -> usize {
        self.idx
    }

    // Sets the current index
    pub fn set_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    // Sets the size of the list
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    // Increments the current idx
    pub fn incr_idx(&mut self, incr: usize) {
        self.idx += incr;
    }

    // Decrements the current idx
    pub fn decr_idx(&mut self, incr: usize) {
        self.idx -= incr;
    }

    // Increments the size of the list
    pub fn incr_size(&mut self, incr: usize) {
        self.size += incr;
    }

    // pub fn print(&self) {
    //     for idx in 0..self.size {
    //         print!("{}:{:?}, ", self.list[idx].mv, self.list[idx].value.0);
    //     }
    //     println!("\n");
    // }
}

#[cfg(test)]
mod tests {
    use super::{MoveValue, MoveValues};

    #[test]
    fn insertion_sort() {
        let mut move_values = MoveValues::<5>::new();
        let mut moves_list = [
            MoveValue::from_i16(5),
            MoveValue::from_i16(31),
            MoveValue::from_i16(-3),
            MoveValue::from_i16(22),
            MoveValue::from_i16(-180),
        ];

        for move_value in moves_list {
            move_values.insert_sort(move_value);
        }

        moves_list.sort();
        moves_list.reverse();
        assert_eq!(move_values.list(), moves_list);
    }
}
