pub mod magics;
// pub mod attack_tables;
// pub mod attack_bb;

use crate::types::bitboard::*;
use crate::types::square::*;
use crate::types::piece::*;

const fn is_square_ok(s: Square) -> bool {
    s.0 >= Square::A1.0 && s.0 <= Square::H8.0
}

const fn shift_square(s: Square, step: i32) -> Square {
    Square(u32::wrapping_add(s.0, step as u32))
}

const fn rank_bb(r: Rank) -> Bitboard {
    Bitboard(RANK_1_BB.0 << (8 * r.0))
}

const fn file_bb(f: File) -> Bitboard {
    Bitboard(FILE_A_BB.0 << f.0)
}

// Pub function since it is used by the build script
pub const fn square_bb(s: Square) -> Bitboard {
    Bitboard(1u64 << (s.0 as u64))
}

/// File distance between two squares
const fn file_distance(s1: Square, s2: Square) -> u32 {
    i32::abs(s1.file().0 as i32 - s2.file().0 as i32) as u32
}

/// Rank distance between two squares
const fn rank_distance(s1: Square, s2: Square) -> u32 {
    i32::abs(s1.rank().0 as i32 - s2.rank().0 as i32) as u32
}

/// Square between two squares
const fn square_distance(s1: Square, s2: Square) -> u32 {
    let file_dist = file_distance(s1, s2);
    let rank_dist = rank_distance(s1, s2);
    if file_dist > rank_dist { file_dist } else { rank_dist }
}

/// safe_destination() returns the bitboard of target square for the given step
/// from the given square. If the step is off the board, returns empty bitboard.
const fn safe_destination(s: Square, step: i32) -> Bitboard {
    let to = shift_square(s, step);
    if is_square_ok(to) && square_distance(s, to) <= 2 { square_bb(to) } else { EMPTY_BB }
}

/// shift() moves a bitboard one step along direction D. Mainly for pawns.
const fn shift(d: Direction, b: Bitboard) -> Bitboard {
    match d {
        NORTH => Bitboard(b.0 << 8),
        SOUTH => Bitboard(b.0 >> 8),
        NORTH_EAST => Bitboard((b.0 & !FILE_H_BB.0) << 9),
        SOUTH_EAST => Bitboard((b.0 & !FILE_H_BB.0) >> 7),
        NORTH_WEST => Bitboard((b.0 & !FILE_A_BB.0) << 7),
        SOUTH_WEST => Bitboard((b.0 & !FILE_A_BB.0) >> 9),
        EAST => Bitboard((b.0 & !FILE_H_BB.0) << 1),
        WEST => Bitboard((b.0 & !FILE_A_BB.0) >> 1),
        _ => panic!("Invalid direction in shift()!")
    }
}

/// Sliding attacks for BISHOP, ROOK
pub const fn sliding_attacks(pt: PieceType, sq: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = EMPTY_BB;

    let directions = match pt {
        ROOK => &ROOK_DIRS,
        BISHOP => &BISHOP_DIRS,
        _ => panic!("Shit!")
    };

    let mut idx: usize = 0;

    while idx < 4 {
        let d  = directions[idx];
        let mut s: Square = sq;
        while safe_destination(s, d.0).0 !=0 && (occupied.0 & square_bb(s).0) == 0 {
            s = shift_square(s, d.0);
            attacks.0 |= square_bb(s).0;
        }
        idx += 1;
    }

    return attacks;
}

const fn pawn_attacks_bb(c: Color, b: Bitboard) -> Bitboard {
    match c {
        WHITE => Bitboard(shift(NORTH_WEST, b).0 | shift(NORTH_EAST, b).0),
        BLACK => Bitboard(shift(SOUTH_WEST, b).0 | shift(SOUTH_EAST, b).0),
        _ => panic!("Invalid color pawn_attacks_bb()!") // const functions can panic!
    }
}


