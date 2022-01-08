use super::square::*;
use super::piece::{Color, WHITE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

pub const fn popcount(bb: Bitboard) -> u32 {
    bb.0.count_ones()
}

pub const EMPTY_BB: Bitboard = Bitboard(0u64);
pub const ALL_SQUARES: Bitboard = Bitboard(!0u64);
pub const DARK_SQUARES: Bitboard = Bitboard(0xaa55aa55aa55aa55);

pub const FILE_A_BB: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B_BB: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_C_BB: Bitboard = Bitboard(0x0404040404040404);
pub const FILE_D_BB: Bitboard = Bitboard(0x0808080808080808);
pub const FILE_E_BB: Bitboard = Bitboard(0x1010101010101010);
pub const FILE_F_BB: Bitboard = Bitboard(0x2020202020202020);
pub const FILE_G_BB: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H_BB: Bitboard = Bitboard(0x8080808080808080);

pub const RANK_1_BB: Bitboard = Bitboard(0xff);
pub const RANK_2_BB: Bitboard = Bitboard(0xff00);
pub const RANK_3_BB: Bitboard = Bitboard(0xff0000);
pub const RANK_4_BB: Bitboard = Bitboard(0xff000000);
pub const RANK_5_BB: Bitboard = Bitboard(0xff00000000);
pub const RANK_6_BB: Bitboard = Bitboard(0xff0000000000);
pub const RANK_7_BB: Bitboard = Bitboard(0xff000000000000);
pub const RANK_8_BB: Bitboard = Bitboard(0xff00000000000000);

impl std::ops::Neg for Bitboard {
    type Output = Bitboard;
    fn neg(self) -> Self {
        Bitboard(self.0.wrapping_neg())
    }
}

impl<RHS> std::ops::BitOrAssign<RHS> for Bitboard
    where Bitboard: std::ops::BitOr<RHS, Output=Bitboard>
{
    fn bitor_assign(&mut self, rhs: RHS) {
        *self = *self | rhs;
    }
}

impl<RHS> std::ops::BitAndAssign<RHS> for Bitboard
    where Bitboard: std::ops::BitAnd<RHS, Output=Bitboard>
{
    fn bitand_assign(&mut self, rhs: RHS) {
        *self = *self & rhs;
    }
}

impl<RHS> std::ops::BitXorAssign<RHS> for Bitboard
    where Bitboard: std::ops::BitXor<RHS, Output=Bitboard>
{
    fn bitxor_assign(&mut self, rhs: RHS) {
        *self = *self ^ rhs;
    }
}

impl std::cmp::PartialEq<u64> for Bitboard {
    fn eq(&self, rhs: &u64) -> bool {
        debug_assert!(*rhs == 0);
        (*self).0 == *rhs
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for s in *self {
            write!(f, "{} ", s.to_string()).unwrap();
        }
        write!(f, "")
    }
}

/// Print a bitboard
pub fn pretty(b: Bitboard) -> String {
    let mut s = String::from("+---+---+---+---+---+---+---+---+\n");
    for r in (0..RANK_NB).rev() {
        for f in 0..FILE_NB {
            let sq = Square::make(File(f as u32), Rank(r as u32));
            let sq_bb = Bitboard(1u64 << (sq.0 as u64));
            s += if (b & sq_bb) != 0 { "| X " } else { "|   " }
        }
        s = format!("{}| {} \n+---+---+---+---+---+---+---+---+\n", s, (1+r).to_string());
    }
    s += "  a   b   c   d   e   f   g   h\n";
    s
}

pub const fn more_than_one(b: Bitboard) -> bool {
    (b.0 & u64::wrapping_sub(b.0, 1)) != 0
}

pub fn lsb(b: Bitboard) -> Square {
    debug_assert!(b != 0);
    Square(u64::trailing_zeros(b.0))
}

pub fn msb(b: Bitboard) -> Square {
    debug_assert!(b != 0);
    Square(63 ^ u64::leading_zeros(b.0))
}

pub fn pop_lsb(b: &mut Bitboard) -> Square {
    let s = lsb(*b);
    b.0 &= u64::wrapping_sub(b.0, 1);
    s
}

pub fn frontmost_sq(c: Color, b: Bitboard) -> Square {
    if c == WHITE { msb(b) } else { lsb(b) }
}

pub fn backmost_sq(c: Color, b: Bitboard) -> Square {
    if c == WHITE { lsb(b) } else { msb(b) }
}

impl Iterator for Bitboard {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if (*self).0 != 0 {
            Some(pop_lsb(self))
        } else {
            None
        }
    }
}

// shift() moves a bitboard one step along direction D. Mainly for pawns.

impl Bitboard {
    pub fn shift(self, d: Direction) -> Bitboard {
        match d {
            NORTH => self << 8,
            SOUTH => self >> 8,
            NORTH_EAST => (self & !FILE_H_BB) << 9,
            SOUTH_EAST => (self & !FILE_H_BB) >> 7,
            NORTH_WEST => (self & !FILE_A_BB) << 7,
            SOUTH_WEST => (self & !FILE_A_BB) >> 9,
            _ => panic!("Wrong direction in types::bitboard::Bitboard::shift()!")
        }
    }
}



