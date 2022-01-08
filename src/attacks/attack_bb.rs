#[cfg(test)]
//#[macro_use]
mod attack_bb_test;

use crate::types::bitboard::*;
use crate::types::square::*;
use crate::types::piece::*;

use super::magics::*;
use super::attack_tables::*;

const KNIGHT_STEPS: [i32; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
const KING_STEPS: [i32; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

const KNIGHT_ATTACKS: [Bitboard; SQUARE_NB] = init_non_slider_attacks(KNIGHT);
const KING_ATTACKS: [Bitboard; SQUARE_NB] = init_non_slider_attacks(KING);
const PAWN_ATTACKS: [[Bitboard; SQUARE_NB]; COLOR_NB] = init_pawn_attacks();

const SQUARE_BB: [Bitboard; SQUARE_NB] = init_square_bb();
const SQUARE_DISTANCE: [[u32; SQUARE_NB]; SQUARE_NB] = init_square_distance();


impl std::convert::From<Square> for Bitboard {
    fn from(s: Square) -> Self {
        square_bb(s)
    }
}

impl Square {
    pub fn bb(self) -> Bitboard {
        Bitboard::from(self)
    }

    pub fn file_bb(self) -> Bitboard {
        file_bb(self.file())
    }

    pub fn rank_bb(self) -> Bitboard {
        rank_bb(self.rank())
    }
}

impl std::ops::BitOr<Square> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Square) -> Self {
        self | Bitboard::from(rhs)
    }
}

impl std::ops::BitAnd<Square> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Square) -> Self {
        self & Bitboard::from(rhs)
    }
}

impl std::ops::BitXor<Square> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Square) -> Self {
        self ^ Bitboard::from(rhs)
    }
}

pub fn square_bb(sq: Square) -> Bitboard{
    SQUARE_BB[sq]
}

pub fn rank_bb(r: Rank) -> Bitboard {
    RANK_1_BB << (8 * r.0) as i32
}

pub fn file_bb(f: File) -> Bitboard {
    FILE_A_BB << f.0 as i32
}

pub fn line_bb(s1: Square, s2: Square) -> Bitboard {
    LINE_BB[s1][s2]
}

pub fn between_bb(s1: Square, s2: Square) -> Bitboard {
    BETWEEN_BB[s1][s2]
}

pub fn square_distance(s1: Square, s2: Square) -> u32 {
    SQUARE_DISTANCE[s1][s2]
}

pub fn attacks_bb(pt: PieceType, sq: Square, occ: Bitboard) -> Bitboard {
    match pt {
        KNIGHT => knight_attacks(sq),
        BISHOP => bishop_attacks(sq, occ),
        ROOK => rook_attacks(sq, occ),
        QUEEN => bishop_attacks(sq, occ) | rook_attacks(sq, occ),
        KING => king_attacks(sq),
        _ => panic!("Bad piecetype for attacks_bb()."),
    }
}

pub fn pseudo_attacks(pt: PieceType, sq: Square) -> Bitboard {
    attacks_bb(pt, sq, EMPTY_BB)
}

pub fn pawn_attacks_bb(c: Color, sq: Square) -> Bitboard {
    PAWN_ATTACKS[c][sq]
}

// aligned() returns true if the squares s1, s2 and s3 are aligned either on
// a straight or on a diagonal line.

pub fn aligned(s1: Square, s2: Square, s3: Square) -> bool {
    line_bb(s1, s2) & s3 != EMPTY_BB
}

/// adjacent_files_bb() returns a bitboard representing all the squares on the
/// adjacent files of a given square.
pub fn adjacent_files_bb(s: Square) -> Bitboard {
    s.file_bb().shift(EAST) | s.file_bb().shift(WEST)
}

/// forward_ranks_bb() returns a bitboard representing the squares on the ranks in
/// front of the given one, from the point of view of the given color. For instance,
/// forward_ranks_bb(BLACK, SQ_D3) will return the 16 squares on ranks 1 and 2.
pub fn forward_ranks_bb(c: Color, s: Square) -> Bitboard {
    match c {
        WHITE => !RANK_1_BB << (s.relative_rank(WHITE).0 * 8) as i32,
        BLACK => !RANK_8_BB << (s.relative_rank(BLACK).0 * 8) as i32,
        _ => panic!("Invalid color in forward_ranks_bb()!")
    }
}

/// forward_file_bb() returns a bitboard representing all the squares along the
/// line in front of the given one, from the point of view of the given color.
pub fn forward_file_bb(c: Color, s: Square) -> Bitboard {
    forward_ranks_bb(c, s) & s.file_bb()
}


/// pawn_attack_span() returns a bitboard representing all the squares that can
/// be attacked by a pawn of the given color when it moves along its file, starting
/// from the given square.
pub fn pawn_attack_span(c: Color, s: Square) -> Bitboard {
    forward_ranks_bb(c, s) & adjacent_files_bb(s)
}

/// passed_pawn_span() returns a bitboard which can be used to test if a pawn of
/// the given color and on the given square is a passed pawn.
pub fn passed_pawn_span(c: Color, s: Square) -> Bitboard {
    pawn_attack_span(c, s) | forward_file_bb(c, s)
}

fn bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    Bitboard(ATTACKS[BISHOP_MAGICS[sq].index(occ)])
}

fn rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    Bitboard(ATTACKS[ROOK_MAGICS[sq].index(occ)])
}

fn knight_attacks(sq: Square) -> Bitboard {
    KNIGHT_ATTACKS[sq]
}

fn king_attacks(sq: Square) -> Bitboard {
    KING_ATTACKS[sq]
}

const fn init_non_slider_attacks(pt: PieceType) -> [Bitboard; SQUARE_NB] {
    let mut attacks = [EMPTY_BB; SQUARE_NB];
    let mut s_idx = 0;

    let steps = match pt {
        KING => KING_STEPS,
        KNIGHT => KNIGHT_STEPS,
        _ => panic!("Shit!")
    };

    while s_idx < SQUARE_NB {
        let mut idx = 0;
        while idx < 8 {
            attacks[s_idx].0 |= super::safe_destination(Square(s_idx as u32), steps[idx]).0;
            idx += 1;
        }
        s_idx += 1;
    }
    attacks
}

const fn init_pawn_attacks() -> [[Bitboard; SQUARE_NB]; COLOR_NB] {
    let mut pawn_attacks = [[EMPTY_BB; SQUARE_NB]; COLOR_NB];
    let mut s_idx = 0;
    while s_idx < SQUARE_NB {
        pawn_attacks[WHITE.0 as usize][s_idx] = super::pawn_attacks_bb(WHITE, super::square_bb(Square(s_idx as u32)));
        pawn_attacks[BLACK.0 as usize][s_idx] = super::pawn_attacks_bb(BLACK, super::square_bb(Square(s_idx as u32)));
        s_idx += 1;
    }
    pawn_attacks
}

const fn init_square_bb() -> [Bitboard; SQUARE_NB] {
    let mut square_bb = [EMPTY_BB; SQUARE_NB];
    let mut s_idx = 0;
    while s_idx < SQUARE_NB {
        square_bb[s_idx] = super::square_bb(Square(s_idx as u32));
        s_idx += 1;
    }
    square_bb
}

const fn init_square_distance() -> [[u32; SQUARE_NB]; SQUARE_NB] {
    let mut square_distance = [[0u32; SQUARE_NB]; SQUARE_NB];
    let mut s1_idx = 0;
    while s1_idx < SQUARE_NB {
        let mut s2_idx = 0;
        while s2_idx < SQUARE_NB {
            square_distance[s1_idx][s2_idx] = super::square_distance(Square(s1_idx as u32), Square(s2_idx as u32));
            s2_idx += 1;
        }
        s1_idx += 1;
    }
    square_distance
}
