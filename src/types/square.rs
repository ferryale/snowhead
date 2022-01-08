#[cfg(test)]
//#[macro_use]
mod square_test;

use super::piece::{Color, WHITE, BLACK};


use super::*;
//use super::*; // pub(super) use enable_base_operations_for_u32_on;
//#[macro_use]   
use super::{enable_base_operations_for_u32_on, enable_indexing_by};

//enable_base_operations_for_u32_on!(Square);
// enable_base_operations_for_u32_on!(File);
// enable_base_operations_for_u32_on!(Rank);
// // enable_base_operations_for_u32_on!(CastlingRight);
// // enable_base_operations_for_i32_on!(Direction);

// // enable_bit_operations_on!(CastlingRight);
// // enable_bit_operations_on!(Bitboard);
// // enable_bit_assign_operations_on!(CastlingRight);

// enable_indexing_by!(Square);
// enable_indexing_by!(File);
// enable_indexing_by!(Rank);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct File(pub u32);

pub const FILE_A: File = File(0);
pub const FILE_B: File = File(1);
pub const FILE_C: File = File(2);
pub const FILE_D: File = File(3);
pub const FILE_E: File = File(4);
pub const FILE_F: File = File(5);
pub const FILE_G: File = File(6);
pub const FILE_H: File = File(7);
pub const FILE_NONE: File = File(8);

impl Iterator for File {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let f = self.0;
        *self = File(f + 1);
        Some(File(f))
    }
}

#[derive(Clone, Copy)]
pub struct Files {
    pub start: File,
    pub end: File
}

impl Iterator for Files {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        let incr = if self.start < self.end { 1 } else { -1 };
        // match incr {
        //     1  => { 
        //                 let start = self.start;
        //                 let 
        //             }
        //     -1 => {}
        //     _ > panic!("Shit")
        // }
        let f = self.start;
        //let end = if incr > 0 { self.end } else { self.start };
        if f != self.end {
            self.start += incr;
            Some(f)
        } else {
            None
        }
    }
}

pub static VALID_FILES: Files = Files{start: FILE_A, end: FILE_NONE};
pub static REVERSED_FILES: Files = Files{start: FILE_H, end: FILE_A};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rank(pub u32);

pub const RANK_1: Rank = Rank(0);
pub const RANK_2: Rank = Rank(1);
pub const RANK_3: Rank = Rank(2);
pub const RANK_4: Rank = Rank(3);
pub const RANK_5: Rank = Rank(4);
pub const RANK_6: Rank = Rank(5);
pub const RANK_7: Rank = Rank(6);
pub const RANK_8: Rank = Rank(7);
pub const RANK_NONE: Rank = Rank(8);

pub fn relative_rank(c: Color, r: Rank) -> Rank {
    Rank(r.0 ^ (c.0 * 7))
}

impl Iterator for Rank {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.0;
        *self = Rank(r + 1);
        Some(Rank(r))
    }
}

#[derive(Clone, Copy)]
pub struct Ranks {
    pub start: Rank,
    pub end: Rank
}

impl Iterator for Ranks {
    type Item = Rank;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.start;
        if r != self.end {
            self.start += 1;
            Some(r)
        } else {
            None
        }
    }
}

pub static VALID_RANKS: Ranks = Ranks{start: RANK_1, end: RANK_NONE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square(pub u32);

pub const SQUARE_NB: usize = 64;
pub const FILE_NB: usize = 8;
pub const RANK_NB: usize = 8;
pub const FILE_TO_CHAR: [char; FILE_NB] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANK_TO_CHAR: [char; RANK_NB] = ['1', '2', '3', '4', '5', '6', '7', '8'];

impl Square {
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    pub const NONE: Square = Square(64);

    pub const fn file(self) -> File {
        File(self.0 & 7)
    }

    pub const fn rank(self) -> Rank {
        Rank(self.0 >> 3)
    }

    pub fn relative(self, c: Color) -> Self {
        Square(self.0 ^ (c.0 * 56))
    }

    pub fn relative_rank(self, c: Color) -> Rank {
        relative_rank(c, self.rank())
    }

    pub fn is_ok(self) -> bool {
        self >= Square::A1 && self <= Square::H8
    }

    pub fn make(f: File, r: Rank) -> Square {
        Square((r.0 << 3) | f.0)
    }

    pub fn to_string(self) -> String {
        format!("{}{}", FILE_TO_CHAR[self.file()], RANK_TO_CHAR[self.rank()])
    }
}

pub fn relative_square(c: Color, s: Square) -> Square {
    s.relative(c)
}

impl std::ops::Not for Square {
    type Output = Self;
    fn not(self) -> Self { Square(self.0 ^ Square::A8.0) }
}

impl std::ops::BitXor<bool> for Square {
    type Output = Self;
    fn bitxor(self, rhs: bool) -> Self {
        Square(self.0 ^ if rhs { 0x38 } else { 0 })
    }
}

impl Iterator for Square {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let sq = self.0;
        *self = Square(sq + 1);
        Some(Square(sq))
    }
}

pub fn opposite_colors(s1: Square, s2: Square) -> bool {
    let s = s1.0 ^ s2.0;
    (((s >> 3) ^ s) & 1) != 0
}


#[derive(Clone, Copy)]
pub struct Squares {
    pub start: Square,
    pub end: Square
}

impl Iterator for Squares {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self.start;
        if s != self.end {
            self.start += Direction(1);
            Some(s)
        } else {
            None
        }
    }
}

pub static VALID_SQUARES: Squares = Squares{start: Square::A1, end: Square::NONE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction(pub i32);

pub const NORTH: Direction = Direction( 8);
pub const EAST : Direction = Direction( 1);
pub const SOUTH: Direction = Direction(-8);
pub const WEST : Direction = Direction(-1);

pub const NORTH_EAST: Direction = Direction( 9);
pub const NORTH_WEST: Direction = Direction( 7);
pub const SOUTH_EAST: Direction = Direction(-7);
pub const SOUTH_WEST: Direction = Direction(-9);

pub const ROOK_DIRS: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];
pub const BISHOP_DIRS: [Direction; 4] = [NORTH_EAST, SOUTH_EAST, SOUTH_WEST, NORTH_WEST];

impl std::ops::Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self { Direction(-self.0) }
}

impl std::ops::Add<Direction> for Square {
    type Output = Square;
    fn add(self, rhs: Direction) -> Self {
        Square(u32::wrapping_add(self.0, rhs.0 as u32))
    }
}

impl std::ops::AddAssign<Direction> for Square {
    fn add_assign(&mut self, rhs: Direction) { *self = *self + rhs; }
}

impl std::ops::Sub<Direction> for Square {
    type Output = Square;
    fn sub(self, rhs: Direction) -> Self {
        Square(u32::wrapping_sub(self.0, rhs.0 as u32))
    }
}

impl std::ops::SubAssign<Direction> for Square {
    fn sub_assign(&mut self, rhs: Direction) { *self = *self - rhs; }
}

impl std::ops::Mul<Direction> for i32 {
    type Output = Direction;
    fn mul(self, rhs: Direction) -> Direction { Direction(self * rhs.0) }
}

pub fn pawn_push(c: Color) -> Direction {
    match c {
        WHITE => NORTH,
        BLACK => SOUTH, 
        _ => panic!("Invalid color in types::square::pawn_push()!")
    }
}








