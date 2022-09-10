use crate::evaluate::score::Value;
use cozy_chess::{Move, Square};
use std::{cmp, fmt};

#[derive(Debug, Clone, Copy, Eq)]
pub struct MoveValue {
    mv: Move,
    value: Value,
}

impl MoveValue {
    pub fn new(mv: Move, value: Value) -> MoveValue {
        MoveValue {
            mv: mv,
            value: value,
        }
    }

    pub fn new_val(val: i16) -> MoveValue {
        MoveValue {
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            },
            value: Value(val),
        }
    }

    pub fn chess_move(&self) -> Move {
        self.mv
    }

    pub fn value(&self) -> Value {
        self.value
    }
}

impl Default for MoveValue {
    fn default() -> MoveValue {
        MoveValue {
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            },
            value: Value(0),
        }
    }
}

impl fmt::Display for MoveValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.mv, self.value.0)
    }
}

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

#[derive(Debug, Clone, Copy)]
pub struct MoveValues<const N: usize> {
    pub list: [MoveValue; N],
    idx: usize,
    size: usize,
}

impl<const N: usize> MoveValues<N> {
    pub fn new() -> MoveValues<N> {
        MoveValues {
            list: [MoveValue::default(); N],
            idx: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, move_value: MoveValue) {
        self.list[self.size] = move_value;
        self.size += 1;
    }

    pub fn push_sort(&mut self, move_value: MoveValue) {
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

    pub fn sort(&mut self) {
        self.list[0..self.size].sort();
        self.list[0..self.size].reverse();
    }

    pub fn replace_next(&mut self, move_value: MoveValue) {
        self.list[self.idx] = move_value;
    }

    pub fn extend(&mut self, other: &MoveValues<N>) {
        let new_size = self.size + other.size();
        for j in self.size..new_size {
            self.list[j] = other.list[j - self.size()];
        }
        self.size = new_size;
    }

    pub fn next(&mut self) -> Option<MoveValue> {
        if self.idx < self.size {
            let next = self.list[self.idx];
            self.idx += 1;
            Some(next)
        } else {
            None
        }
    }

    pub fn next_move(&mut self) -> Option<Move> {
        match self.next() {
            Some(move_value) => Some(move_value.chess_move()),
            None => None,
        }
    }

    pub fn list(&self) -> [MoveValue; N] {
        self.list
    }

    pub fn list_mut(&mut self) -> [MoveValue; N] {
        self.list
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn set_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn incr_idx(&mut self, incr: usize) {
        self.idx+= incr;
    }

    pub fn decr_idx(&mut self, incr: usize) {
        self.idx -= incr;
    }

    pub fn incr_size(&mut self, incr: usize) {
        self.size += incr;
    }

    pub fn print(&self) {
        for idx in 0..self.size {
            print!("{}:{:?}, ", self.list[idx].mv, self.list[idx].value.0);
        }
        println!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::{MoveValue, MoveValues};

    #[test]
    fn insertion_sort_works() {
        let mut move_values = MoveValues::<5>::new();
        let mut moves_list = [
            MoveValue::new_val(5),
            MoveValue::new_val(31),
            MoveValue::new_val(-3),
            MoveValue::new_val(22),
            MoveValue::new_val(-180),
        ];
        for move_value in moves_list {
            move_values.push_sort(move_value);
        }
        moves_list.sort();
        moves_list.reverse();
        assert_eq!(move_values.list(), moves_list);
    }
}
