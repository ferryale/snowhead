use std::mem;

use crate::zobrist::Key;
use crate::types::score::Value;
use crate::types::r#move::Move;

const MAX_TT_SIZE_MB: usize = 1024;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TTFlag {
    EXACT,
    LOWER,
    UPPER,
    NONE,
}


#[derive(Debug, Clone, Copy)]
pub struct TTEntry{
    key16: u16,
    value16: i16,
    flag: TTFlag,
    move16: u16,
    
}

pub struct TranspositionTable {
    table: Vec<TTEntry>
}

impl TranspositionTable {
    pub fn new(mb_size: usize) -> TranspositionTable {
        
        TranspositionTable {
            table: vec![Default::default(); Self::num_entries(mb_size)]
        }
        
    }

    pub fn save(&mut self, key: Key, value: Value, flag: TTFlag, m: Move) {
        let idx = self.idx(key);
        self.table[idx] = TTEntry::new(key, value, flag, m);
    }

    pub fn probe(&mut self, key: Key) -> (bool, Value, TTFlag, Move) {
        
        let idx = self.idx(key);
        let tte = self.table[idx];
        let tt_hit = tte.flag != TTFlag::NONE;
        if tt_hit {
            return (true, tte.get_value(), tte.get_flag(), tte.get_move());
        }
        (false, Value::ZERO, TTFlag::NONE, Move::NONE)
    }

    pub fn len(&self) -> usize {
        self.table.len()

    }

    pub fn size_mb(&self) -> usize {
        self.table.len() * mem::size_of::<TTEntry>() / 1024 / 1024 

    }

    pub fn clear(&mut self) {
        for idx in 0..self.len() {
            self.table[idx] = Default::default();
        }

    }

    fn idx(&self, key: Key) -> usize {
        key as usize % self.table.len()
    }
    
    // // This method clears up the table
    // fn resize(&mut self, mb_size: usize) {
    //     self.table = vec![Default::default(); Self::num_entries(mb_size)]
    // }
    
    fn num_entries(size_mb: usize) -> usize {
        if size_mb > MAX_TT_SIZE_MB {
            eprintln!("Transposition table size exceeds max size.\n
                 Size will be set to {} MB", MAX_TT_SIZE_MB);
            return MAX_TT_SIZE_MB * 1024 * 1024 / mem::size_of::<TTEntry>();
        }
        size_mb * 1024 * 1024 / mem::size_of::<TTEntry>()
    }
    
    
}



impl TTEntry {
    
    pub fn new(key: Key, value: Value, flag: TTFlag, m: Move) -> TTEntry {
        
        TTEntry {
            key16: key as u16, 
            value16: value.0 as i16, 
            flag: flag, 
            move16: m.0 as u16
        }
        
    }

    pub fn get_value(&self) -> Value {
        Value(self.value16 as i32)
    }

    pub fn get_flag(&self) -> TTFlag {
        self.flag
    }

    pub fn get_move(&self) -> Move {
        Move(self.move16 as u32)
    }

}

impl Default for TTEntry {
    fn default() -> Self {

        TTEntry {
            key16: 0u16,
            value16: 0i16,
            flag: TTFlag::NONE,
            move16: 0u16
        }

    }
}

#[cfg(test)]
mod tt_test {

    use crate::evaluate::evaluate;
    use crate::position::Position;
    //use crate::position::Position;
    use super::*;

    #[test]
    fn size_mb_is_correctly_set() {
        let sizes = [1, 10, 100, 1000];
        let mut ttable;
        for size in sizes {
            ttable = TranspositionTable::new(size);
            assert_eq!(size, ttable.size_mb());
        } 
    }

    #[test]
    fn size_mb_is_set_to_limit() {
        let sizes = [1100, 1500];
        let mut ttable;
        for size in sizes {
            ttable = TranspositionTable::new(size);
            assert_eq!(MAX_TT_SIZE_MB, ttable.size_mb());
        } 
    }

    #[test]
    fn save_and_probe_returns_same_value() {
        
        let test_fens = vec!(
           // avoid illegal ep
           "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",
           "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1",  
           "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",
           "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1",
           // en passant capture checks opponent: 
           "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1", 
           "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1",
           // short castling gives check: 
           "5k2/8/8/8/8/8/8/4K2R w K - 0 1", 
           "4k2r/8/8/8/8/8/8/5K2 b k - 0 1",
           // long castling gives check: 
           "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",
           "r3k3/8/8/8/8/8/8/3K4 b q - 0 1", 
           // castling (including losing cr due to rook capture): 
           "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",
           "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1",
           // castling prevented: 
           "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",
           "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1", 
           // promote out of check: 
           "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1", 
           "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1",  
           // discovered check: 
           "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",
           "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1", 
           // promote to give check: 
           "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",
           "8/k7/8/8/8/8/1p6/4K3 b - - 0 1", 
           // underpromote to check: 
           "8/P1k5/K7/8/8/8/8/8 w - - 0 1",
           "8/8/8/8/8/k7/p1K5/8 b - - 0 1", 
           // self stalemate: 
           "K1k5/8/P7/8/8/8/8/8 w - - 0 1", 
           "8/8/8/8/8/p7/8/k1K5 b - - 0 1",
           // stalemate/checkmate: 
           "8/k1P5/8/1K6/8/8/8/8 w - - 0 1", 
           "8/8/8/8/1k6/8/K1p5/8 b - - 0 1", 
           // double check: 
           "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1", 
           "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1",  

           // short castling impossible although the rook never moved away from its corner 
           "1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1",
           "4k2r/8/8/7r/8/8/1B6/1K6 w k - 0 1", 

           // long castling impossible although the rook never moved away from its corner 
           "1k6/8/8/8/R7/1n6/8/R3K3 b Q - 0 1", 
           "r3k3/8/1N6/r7/8/8/8/1K6 w q - 0 1",

           // From the Wiki
           "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
           "rnbqkb1r/pp1p1ppp/2p5/4P3/2B5/8/PPP1NnPP/RNBQK2R w KQkq - 0 6",

           // Shortened form of the third position below
           "8/7p/p5pb/4k3/P1pPn3/8/P5PP/1rB2RK1 b - d3 0 28"
        );

        let mut ttable = TranspositionTable::new(10);
        let mut pos = Position::new();
        let mut value;

        for fen in &test_fens {
            pos.set(fen, false);
            value = evaluate(&pos);
            ttable.save(pos.key(), value, TTFlag::EXACT, Move::NONE);
            let (_tt_hit, new_value, flag, _m) = ttable.probe(pos.key());

            assert_ne!(flag, TTFlag::NONE);
            assert_eq!(value, new_value);

        }

    

    }

}



