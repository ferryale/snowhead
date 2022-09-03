use cozy_chess::{Square, Move};
use crate::evaluate::score::Value;
use std::cmp;

#[derive(Debug, Clone, Copy, Eq)]
pub struct MoveValue {
    mv: Move,
    value: Value,
}

impl MoveValue {

    pub fn new() -> MoveValue {
        MoveValue {
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            }, 
            value: Value(0)
        }
    }

    pub fn new_val(val: i16) -> MoveValue {
        MoveValue {
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            }, 
            value: Value(val)
        }
    }

    pub fn chess_move(&self) -> Move {
        self.mv
    }

    pub fn value(&self) -> Value{
        self.value
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
    list: [MoveValue;N],
    idx: usize,
    size: usize,
}
impl<const N: usize> MoveValues<N> {
    pub fn new() -> MoveValues<N> {
        MoveValues {
            list: [MoveValue::new();N],
            idx: 0,
            size: 0
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
                    self.list[k+1] = self.list[k];
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
        self.list.sort();
    }

    pub fn next(&mut self) -> MoveValue {
        self.idx += 1;
        self.list[self.idx-1]
    }

    pub fn next_move(&mut self) -> Move {
        self.next().chess_move()
    }

    pub fn list(&self) -> [MoveValue;N] {
        self.list
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn idx(&self) -> usize {
        self.idx
    }
    
}

#[cfg(test)]
mod tests{
    use super::{MoveValue, MoveValues};

    #[test]
    fn insertion_sort_works() {
        let mut move_values = MoveValues::<5>::new();
        let mut moves_list = [MoveValue::new_val(5), MoveValue::new_val(31), MoveValue::new_val(-3), MoveValue::new_val(22), MoveValue::new_val(-180)];
        for move_value in moves_list {
            move_values.push_sort(move_value);
        }
        moves_list.sort();
        moves_list.reverse();
        assert_eq!(move_values.list(), moves_list);
    }


}