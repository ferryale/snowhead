use super::square::{Square, FILE_C, FILE_G};
use super::piece::{Color, PieceType, KNIGHT, WHITE, BLACK, PIECE_TO_CHAR};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveType(pub u32);

pub const NORMAL    : MoveType = MoveType(0);
pub const PROMOTION : MoveType = MoveType(1 << 14);
pub const EN_PASSANT : MoveType = MoveType(2 << 14);
pub const CASTLING  : MoveType = MoveType(3 << 14);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(pub u32);

impl Move {
    pub const NONE : Move = Move(0);
    pub const NULL : Move = Move(65);

    pub const fn from(self) -> Square {
        Square((self.0 >> 6) & 0x3f)
    }

    pub const fn to(self) -> Square {
        Square(self.0 & 0x3f)
    }

    pub const fn from_to(self) -> u32 {
        self.0 & 0xfff
    }

    pub const fn move_type(self) -> MoveType {
        MoveType(self.0 & (3 << 14))
    }

    pub const fn promotion_type(self) -> PieceType {
        PieceType(((self.0 >> 12) & 3) + KNIGHT.0)
    }

    pub fn is_ok(self) -> bool {
        self.from() != self.to()
    }

    pub const fn make(from: Square, to: Square) -> Move {
        Move((from.0 << 6) + to.0)
    }

    pub const fn make_prom(from: Square, to: Square, pt: PieceType) -> Move {
        Move(PROMOTION.0 + ((pt.0 - KNIGHT.0) << 12) + (from.0 << 6) + to.0)
    }

    pub const fn make_special(mt: MoveType, from: Square, to: Square) -> Move {
        Move(mt.0 + (from.0 << 6) + to.0)
    }

    pub fn to_string(self, chess960: bool) -> String {
        let from: Square = self.from();
        let mut to: Square = self.to();

        if self == Move::NONE { "(none)".to_string() }
        else if self == Move::NULL { "0000".to_string() }
        else {
            if self.move_type() == CASTLING && !chess960 {
                to = Square::make( if to > from { FILE_G } else { FILE_C }, from.rank() );
            }
            let mut move_str = format!("{}{}", from.to_string(), to.to_string());
            if self.move_type() == PROMOTION {
                move_str.push(PIECE_TO_CHAR[self.promotion_type().0 as usize +8])
            }
            move_str
        }
    }        
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRight(pub u32);

pub const CASTLING_RIGHT_NB: usize = 16;

pub const NO_CASTLING : CastlingRight = CastlingRight(0);
pub const WHITE_OO    : CastlingRight = CastlingRight(1);
pub const WHITE_OOO   : CastlingRight = CastlingRight(2);
pub const BLACK_OO    : CastlingRight = CastlingRight(4);
pub const BLACK_OOO   : CastlingRight = CastlingRight(8);
pub const KING_SIDE: CastlingRight      = CastlingRight(WHITE_OO.0  | BLACK_OO.0);
pub const QUEEN_SIDE: CastlingRight     = CastlingRight(WHITE_OOO.0 | BLACK_OOO.0);
pub const WHITE_CASTLING: CastlingRight = CastlingRight(WHITE_OO.0  | WHITE_OOO.0);
pub const BLACK_CASTLING: CastlingRight = CastlingRight(BLACK_OO.0  | BLACK_OOO.0);
pub const ANY_CASTLING: CastlingRight = CastlingRight(15);
pub const INVALID_CASTLING: CastlingRight = CastlingRight(16);

#[derive(Debug, Clone, Copy)]
pub struct CastlingRights {
    pub start: CastlingRight,
    pub end: CastlingRight
}

impl Iterator for CastlingRights {
    type Item = CastlingRight;
    fn next(&mut self) -> Option<Self::Item> {
        let cr = self.start;
        if cr != self.end {
            self.start += 1;
            Some(cr)
        } else {
            None
        }
    }
}

/// Castling rights
pub fn castling_right_c(c: Color, cr: CastlingRight) -> CastlingRight {
    match c {
        WHITE => WHITE_CASTLING & cr,
        BLACK => BLACK_CASTLING & cr,
        _ => panic!("Shit!"),
    }
}

pub static VALID_CASTLING_RIGHTS: CastlingRights = CastlingRights{start: NO_CASTLING, end: INVALID_CASTLING};
