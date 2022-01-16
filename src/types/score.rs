use super::piece::{Piece, PIECE_NB, COLOR_NB};

pub const MAX_MOVES: usize = 256;
pub const MAX_PLY: i32 = 128;
pub const MAX_MATE_PLY : i32 = 128;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Depth(pub i32);

impl std::ops::Mul<Depth> for i32 {
    type Output = Depth;
    fn mul(self, rhs: Depth) -> Depth { Depth(self * rhs.0) }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Phase(pub u32);

impl Phase {
    pub const ENDGAME: Phase = Phase(0);
    pub const MIDGAME: Phase = Phase(128);
}

pub const MG: usize = 0;
pub const EG: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(pub i32);

impl Value {
    pub const ZERO     : Value = Value(0);
    pub const DRAW     : Value = Value(0);
    pub const KNOWN_WIN: Value = Value(10000);
    pub const MATE     : Value = Value(32000);
    pub const INFINITE : Value = Value(32001);
    pub const NONE     : Value = Value(32002);

    pub const PAWN_MG  : Value = Value(100);
    pub const KNIGHT_MG: Value = Value(320);
    pub const BISHOP_MG: Value = Value(330);
    pub const ROOK_MG  : Value = Value(500);
    pub const QUEEN_MG : Value = Value(900);

    pub const PAWN_EG  : Value = Value(100);
    pub const KNIGHT_EG: Value = Value(320);
    pub const BISHOP_EG: Value = Value(330);
    pub const ROOK_EG  : Value = Value(500);
    pub const QUEEN_EG : Value = Value(900);

    pub const MATE_IN_MAX_PLY : Value =
        Value( Value::MATE.0 - MAX_MATE_PLY - MAX_PLY);
    pub const MATED_IN_MAX_PLY: Value =
        Value(-Value::MATE.0 + MAX_MATE_PLY + MAX_PLY);

    pub fn abs(self) -> Value {
        Value(self.0.abs())
    }
}

pub const MIDGAME_LIMIT : Value = Value(15258);
pub const ENDGAME_LIMIT : Value = Value(3915);

const PIECE_VALUE: [[Value; PIECE_NB]; COLOR_NB] = [
    [ Value::ZERO,      Value::PAWN_MG, Value::KNIGHT_MG, Value::BISHOP_MG,
      Value::ROOK_MG,   Value::QUEEN_MG, Value::ZERO, Value::ZERO,
      Value::ZERO, Value::PAWN_MG, Value::KNIGHT_MG, Value::BISHOP_MG,
      Value::ROOK_MG, Value::QUEEN_MG, Value::ZERO, Value::ZERO],
    [ Value::ZERO, Value::PAWN_EG, Value::KNIGHT_EG, Value::BISHOP_EG,
      Value::ROOK_EG, Value::QUEEN_EG, Value::ZERO, Value::ZERO,
      Value::ZERO, Value::PAWN_EG, Value::KNIGHT_EG, Value::BISHOP_EG,
      Value::ROOK_EG, Value::QUEEN_EG, Value::ZERO, Value::ZERO ]
];

pub const fn piece_value(phase: usize, pc: Piece) -> Value {
    PIECE_VALUE[phase][pc.0 as usize]
}

impl std::ops::Neg for Value {
    type Output = Self;
    fn neg(self) -> Self { Value(-self.0) }
}

impl std::ops::Mul<Value> for i32 {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value { Value(self * rhs.0) }
}

pub fn mate_in(ply: i32) -> Value {
    Value::MATE - ply
}

pub fn mated_in(ply: i32) -> Value {
    -Value::MATE + ply
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score(pub i32);

impl Score {
    pub const ZERO: Score = Score(0);

    pub fn eg(self) -> Value {
        Value((((self.0 + 0x8000) >> 16) as i16) as i32)
    }

    pub fn mg(self) -> Value {
        Value((self.0 as i16) as i32)
    }

    pub const fn make(mg: Value, eg: Value) -> Self {
        Score((eg.0 << 16) + mg.0)
    }
}

impl std::ops::Add<Score> for Score {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Score(self.0 + rhs.0) }
}

impl std::ops::AddAssign<Score> for Score {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl std::ops::Sub<Score> for Score {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Score(self.0 - rhs.0) }
}

impl std::ops::SubAssign<Score> for Score {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl std::ops::Neg for Score {
    type Output = Self;
    fn neg(self) -> Self { Score(-self.0) }
}

impl std::ops::Mul<i32> for Score {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Score::make(rhs * self.mg(), rhs * self.eg())
    }
}
