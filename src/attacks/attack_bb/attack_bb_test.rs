use super::{attacks_bb};
use crate::attacks::{sliding_attacks};
use crate::types::square::Square;
use crate::types::piece::*;
use crate::types::bitboard::{EMPTY_BB, pretty};

#[test]
fn magic_attacks_match_sliding_attacks() {

    for pt in [ROOK, BISHOP] {
        let b1 = sliding_attacks(pt, Square::C5, EMPTY_BB);
        let b2 = attacks_bb(pt, Square::C5, EMPTY_BB);

        assert_eq!(b1, b2);

    }
    
}
