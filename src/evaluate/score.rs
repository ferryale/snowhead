use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/* Value is the type returned by an evaluation */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(pub i16);

/* Phase of a game */
#[derive(Debug, Clone, Copy)]
pub struct Phase(pub u32);

/*
    Score is an array of values
    corresponding to different game phases.
    Default is two, but multiple phases can be implemented,
    for more complex tapered evaluations.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score {
    pub values: [Value; Phase::NUM],
}

/* Value implementation */
impl Value {
    pub const ZERO: Value = Value(0);
    pub const DRAW: Value = Value(0);
    pub const KNOWN_WIN: Value = Value(10000);
    pub const MATE: Value = Value(32000);
    pub const INFINITE: Value = Value(32001);

    // Returns a mate in N-plies score
    pub fn mate_in(ply: u32) -> Value {
        Value::MATE - ply as i32
    }

    // Returns a mated in N-plies value
    pub fn mated_in(ply: u32) -> Value {
        -Value::MATE + ply as i32
    }
}

/* Phase implementation */
impl Phase {
    pub const NUM: usize = 2;
    pub const ZERO: Phase = Phase(0);
    pub const ENDGAME: Phase = Phase(0);
    pub const MIDGAME: Phase = Phase(48);
    pub const PAWN: Phase = Phase(0);
    pub const KNIGHT: Phase = Phase(2);
    pub const BISHOP: Phase = Phase(2);
    pub const ROOK: Phase = Phase(4);
    pub const QUEEN: Phase = Phase(8);
    pub const ALL: [Phase; 5] = [
        Phase::PAWN,
        Phase::KNIGHT,
        Phase::BISHOP,
        Phase::ROOK,
        Phase::QUEEN,
    ];
}

/* Score implementation */
impl Score {
    pub const ZERO: Score = Score::default();

    // Default needs to be constant, so no Default trait
    pub const fn default() -> Score {
        Score {
            values: [Value(0); Phase::NUM],
        }
    }

    // Creates a new score from array of values
    pub fn new(vals: &[i16; Phase::NUM]) -> Score {
        let mut score = Score::ZERO;
        for idx in 0..Phase::NUM {
            score.values[idx] = Value(vals[idx]);
        }
        score
    }
}

/* Macros for math operations */
macro_rules! impl_math_ops {
    ($($type:ty, $trait:ident, $fn:ident;)*) => {$(
        impl $trait for $type {
            type Output = Self;
            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self::Output {
                Self($trait::$fn(self.0, rhs.0))
            }
        }
    )*};
}

/* Macros for math assign operations */
macro_rules! impl_math_assign_ops {
    ($($type:ty, $trait:ident, $fn:ident;)*) => {$(
        impl $trait for $type {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
                $trait::$fn(&mut self.0, rhs.0)
            }
        }
    )*};
}

/* Macros for vector math operations */
macro_rules! impl_vec_math_ops {
    ($($type:ty, $trait:ident, $fn:ident;)*) => {$(
        impl $trait for $type {
            type Output = Self;
            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self::Output {
                let mut sum = <$type>::default();
                for idx in 0..Phase::NUM {
                    sum.values[idx] = $trait::$fn(self.values[idx], rhs.values[idx])
                }
                sum
            }
        }
    )*};
}

/* Macros for vector math assign operations */
macro_rules! impl_vec_math_assign_ops {
    ($($type:ty, $trait:ident, $fn:ident;)*) => {$(
        impl $trait for $type {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
                for idx in 0..Phase::NUM {
                    $trait::$fn(&mut self.values[idx], rhs.values[idx])
                }
            }
        }
    )*};
}

/* Implement math operations on Value, Score and Phase */
impl_math_ops! {
    Value, Add, add;
    Value, Sub, sub;
    Phase, Add, add;
    Phase, Sub, sub;

}

impl_vec_math_ops! {
    Score, Add, add;
    Score, Sub, sub;
}

impl_math_assign_ops! {
    Value, AddAssign, add_assign;
    Value, SubAssign, sub_assign;
    Phase, AddAssign, add_assign;
    Phase, SubAssign, sub_assign;
}

impl_vec_math_assign_ops! {
    Score, AddAssign, add_assign;
    Score, SubAssign, sub_assign;
}

impl Neg for Value {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}

impl Neg for Score {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        let mut ret = Score::ZERO;
        for idx in 0..Phase::NUM {
            ret.values[idx] = -self.values[idx];
        }
        ret
    }
}

impl Mul<Phase> for Value {
    type Output = Self;
    fn mul(self, rhs: Phase) -> Self::Output {
        Value(self.0 * rhs.0 as i16)
    }
}

impl Mul<Phase> for Score {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Phase) -> Self::Output {
        let mut ret = Score::ZERO;
        for idx in 0..Phase::NUM {
            ret.values[idx] = self.values[idx] * rhs;
        }
        ret
    }
}

impl Add<i32> for Value {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Value(self.0 + rhs as i16)
    }
}

impl Sub<i32> for Value {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Value(self.0 - rhs as i16)
    }
}

impl Mul<i32> for Value {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Value(self.0 * rhs as i16)
    }
}

impl Mul<u32> for Phase {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Phase(self.0 * rhs)
    }
}

impl Mul<i32> for Score {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: i32) -> Self::Output {
        let mut ret = Score::ZERO;
        for idx in 0..Phase::NUM {
            ret.values[idx] = self.values[idx] * rhs;
        }
        ret
    }
}

impl Div<Phase> for Value {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Phase) -> Self::Output {
        Value(self.0 / rhs.0 as i16)
    }
}

impl Div<Phase> for Score {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Phase) -> Self::Output {
        let mut ret = Score::ZERO;
        for idx in 0..Phase::NUM {
            ret.values[idx] = self.values[idx] / rhs;
        }
        ret
    }
}

impl Div<i32> for Value {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i32) -> Self::Output {
        Value(self.0 / rhs as i16)
    }
}

impl Div<i32> for Score {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i32) -> Self::Output {
        let mut ret = Score::ZERO;
        for idx in 0..Phase::NUM {
            ret.values[idx] = self.values[idx] / rhs;
        }
        ret
    }
}
