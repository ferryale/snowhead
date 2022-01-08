use crate::types::square::{SQUARE_NB, FILE_NB, VALID_SQUARES, VALID_FILES};
use crate::types::piece::{PIECE_NB, VALID_PIECES};
use crate::types::r#move::{CASTLING_RIGHT_NB, VALID_CASTLING_RIGHTS};
use crate::rng;

pub type Key = u64;
pub const KEY_ZERO: Key = 0u64; 

#[derive(Debug)]
pub struct Zobrist {
    pub psq: [[Key; SQUARE_NB]; PIECE_NB],
    pub en_passant: [Key; FILE_NB],
    pub castling: [Key; CASTLING_RIGHT_NB],
    pub side: Key,
    pub no_pawns: Key,
}

impl Zobrist {

    pub fn new() -> Zobrist {
        Zobrist {
            psq : [[KEY_ZERO; SQUARE_NB]; PIECE_NB],
            en_passant : [KEY_ZERO; FILE_NB],
            castling : [KEY_ZERO; CASTLING_RIGHT_NB],
            side : KEY_ZERO,
            no_pawns : KEY_ZERO,
        }
    }

    pub fn init(& mut self) {

        let mut rng = rng::Prng::new(1070372);

        for pc in VALID_PIECES {
            for s in VALID_SQUARES {
                self.psq[pc][s] = rng.rand::<Key>();
            }
        }

        for f in VALID_FILES {
            self.en_passant[f] = rng.rand::<Key>();
        }

        for cr in VALID_CASTLING_RIGHTS {
            self.castling[cr] = rng.rand::<Key>();
        }
        
        self.side = rng.rand::<Key>();
        self.no_pawns = rng.rand::<Key>();

    }
}
