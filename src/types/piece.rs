#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

pub const COLOR_NB: usize = 2;

pub const WHITE: Color = Color(0);
pub const BLACK: Color = Color(1);

impl std::ops::Not for Color {
    type Output = Color;
    fn not(self) -> Self { Color(self.0 ^ 1) }
}

impl std::ops::BitXor<bool> for Color {
    type Output = Self;
    fn bitxor(self, rhs: bool) -> Self { Color(self.0 ^ (rhs as u32)) }
}

impl Iterator for Color {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let sq = self.0;
        self.0 += 1;
        Some(Color(sq))
    }
}

impl Color {
    pub fn is_ok(self) -> bool {
        self == WHITE || self == BLACK
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PieceType(pub u32);

pub const PIECE_TYPE_NB: usize = 8;

pub const NO_PIECE_TYPE: PieceType = PieceType(0);

pub const PAWN  : PieceType = PieceType(1);
pub const KNIGHT: PieceType = PieceType(2);
pub const BISHOP: PieceType = PieceType(3);
pub const ROOK  : PieceType = PieceType(4);
pub const QUEEN : PieceType = PieceType(5);
pub const KING  : PieceType = PieceType(6);

pub const QUEEN_DIAGONAL: PieceType = PieceType(7);

pub const ALL_PIECES: PieceType = PieceType(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub u32);

pub const PIECE_NB: usize = 16;

pub const NO_PIECE: Piece = Piece(0);

pub const W_PAWN  : Piece = Piece(1);
pub const W_KNIGHT: Piece = Piece(2);
pub const W_BISHOP: Piece = Piece(3);
pub const W_ROOK  : Piece = Piece(4);
pub const W_QUEEN : Piece = Piece(5);
pub const W_KING  : Piece = Piece(6);

pub const B_PAWN  : Piece = Piece(9);
pub const B_KNIGHT: Piece = Piece(10);
pub const B_BISHOP: Piece = Piece(11);
pub const B_ROOK  : Piece = Piece(12);
pub const B_QUEEN : Piece = Piece(13);
pub const B_KING  : Piece = Piece(14);

pub const VALID_PIECES: [Piece; 12] = [W_PAWN, W_KNIGHT, W_BISHOP, W_ROOK, W_QUEEN, W_KING, B_PAWN, B_KNIGHT, B_BISHOP, B_ROOK, B_QUEEN, B_KING];

/// Char conversion
pub const PIECE_TO_CHAR: [char; PIECE_NB] = [' ', 'P', 'N', 'B', 'R', 'Q', 'K', ' ', ' ', 'p', 'n', 'b', 'r', 'q', 'k', ' '];

impl Piece {

    pub fn to_char(self) -> char {
        PIECE_TO_CHAR[self]
    }
    pub const fn piece_type(self) -> PieceType { PieceType(self.0 & 7) }

    pub const fn color(self) -> Color { Color(self.0 >> 3) }

    pub const fn make(c: Color, pt: PieceType) -> Piece { Piece((c.0 << 3) + pt.0) }
}

impl Iterator for Piece {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let pc = self.0;
        self.0 += 1;
        Some(Piece(pc))
    }
}

impl std::ops::Not for Piece {
    type Output = Self;
    fn not(self) -> Self { Piece(self.0 ^ 8) }
}

impl std::ops::BitXor<bool> for Piece {
    type Output = Self;
    fn bitxor(self, rhs: bool) -> Self { Piece(self.0 ^ ((rhs as u32) << 3)) }
}
