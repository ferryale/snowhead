pub mod square;
pub mod piece;
pub mod r#move;
pub mod bitboard;
pub mod score;

use std::ops;
use self::square::{Rank, File, Square, Direction};
use self::piece::{Color, PieceType, Piece};
use self::r#move::{Move, CastlingRight};
use self::bitboard::Bitboard;
use self::score::Value;

macro_rules! enable_base_operations_on {

    ($ty: ident) => {

        impl ops::Add<$ty> for $ty {
            type Output = $ty;
            fn add(self, rhs: $ty) -> Self {
                $ty(self.0 + rhs.0)
            }

        }

        impl ops::AddAssign<$ty> for $ty {
            fn add_assign(&mut self, rhs: $ty) {
                *self = *self + rhs;
            }

        }

        impl ops::Sub<$ty> for $ty {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> Self {
                $ty(self.0 - rhs.0)
            }

        }

        impl ops::SubAssign<$ty> for $ty {
            fn sub_assign(&mut self, rhs: $ty) {
                *self = *self - rhs;
            }

        }

    };

}

macro_rules! enable_full_operations_on {

    ($ty: ident) => {

        enable_base_operations_on!($ty);

        impl ops::Mul<$ty> for $ty {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> Self {
                $ty(self.0 * rhs.0)
            }

        }

        impl ops::MulAssign<$ty> for $ty {
            fn mul_assign(&mut self, rhs: $ty) {
                *self = *self * rhs;
            }

        }

        impl ops::Div<$ty> for $ty {
            type Output = $ty;
            fn div(self, rhs: $ty) -> Self {
                $ty(self.0 / rhs.0)
            }

        }

        impl ops::DivAssign<$ty> for $ty {
            fn div_assign(&mut self, rhs: $ty) {
                *self = *self / rhs;
            }

        }

    };

}

macro_rules! enable_base_i32_operations_for_u32_on {

    ($ty: ident) => {

        impl ops::Add<i32> for $ty {
            type Output = $ty;
            fn add(self, rhs: i32) -> Self {
                $ty(u32::wrapping_add(self.0, rhs as u32))
            }

        }

        impl ops::AddAssign<i32> for $ty {
            fn add_assign(&mut self, rhs: i32) {
                *self = *self + rhs;
            }

        }

        impl ops::Sub<i32> for $ty {
            type Output = $ty;
            fn sub(self, rhs: i32) -> Self {
                $ty(u32::wrapping_add(self.0, rhs as u32))
            }

        }

        impl ops::SubAssign<i32> for $ty {
            fn sub_assign(&mut self, rhs: i32) {
                *self = *self - rhs;
            }

        }

    };

}

macro_rules! enable_base_i32_operations_for_i32_on {

    ($ty: ident) => {

        impl ops::Add<i32> for $ty {
            type Output = $ty;
            fn add(self, rhs: i32) -> Self {
                $ty(self.0 + rhs)
            }

        }

        impl ops::AddAssign<i32> for $ty {
            fn add_assign(&mut self, rhs: i32) {
                *self = *self + rhs;
            }

        }

        impl ops::Sub<i32> for $ty {
            type Output = $ty;
            fn sub(self, rhs: i32) -> Self {
                $ty(self.0 - rhs)
            }

        }

        impl ops::SubAssign<i32> for $ty {
            fn sub_assign(&mut self, rhs: i32) {
                *self = *self - rhs;
            }

        }

    };

}

macro_rules! enable_full_i32_operations_for_i32_on {

    ($ty: ident) => {

        enable_base_i32_operations_for_i32_on!($ty);

        impl ops::Mul<i32> for $ty {
            type Output = $ty;
            fn mul(self, rhs: i32) -> Self {
                $ty(self.0 * rhs)
            }

        }

        impl ops::MulAssign<i32> for $ty {
            fn mul_assign(&mut self, rhs: i32) {
                *self = *self * rhs;
            }

        }

        impl ops::Div<i32> for $ty {
            type Output = $ty;
            fn div(self, rhs: i32) -> Self {
                $ty(self.0 / rhs)
            }

        }

        impl ops::DivAssign<i32> for $ty {
            fn div_assign(&mut self, rhs: i32) {
                *self = *self / rhs;
            }

        }

    };

}

macro_rules! enable_bit_operations_on {

    ($ty: ident) => {

        impl ops::Shl<i32> for $ty {
            type Output = $ty;
            fn shl(self, rhs: i32) -> Self {
                $ty(self.0 << rhs)
            }

        }

        impl ops::Shr<i32> for $ty {
            type Output = $ty;
            fn shr(self, rhs: i32) -> Self {
                $ty(self.0 >> rhs)
            }

        }

        impl ops::Not for $ty {
            type Output = $ty;
            fn not(self) -> Self {
                $ty(!self.0)
            }

        }

        impl ops::BitAnd for $ty {
            type Output = $ty;
            fn bitand(self, rhs: $ty) -> Self {
                $ty(self.0 & rhs.0)
            }

        }

        impl ops::BitOr for $ty {
            type Output = $ty;
            fn bitor(self, rhs: $ty) -> Self {
                $ty(self.0 | rhs.0)
            }

        }

        impl ops::BitXor for $ty {
            type Output = $ty;
            fn bitxor(self, rhs: $ty) -> Self {
                $ty(self.0 ^ rhs.0)
            }

        }
        
    };

}

macro_rules! enable_bit_assign_operations_on {
    ($ty: ident) => {

        impl ops::BitAndAssign<$ty> for $ty {
            fn bitand_assign(&mut self, rhs: $ty) {
                *self = *self & rhs;
            }

        }

       

        impl ops::BitOrAssign<$ty> for $ty {
            fn bitor_assign(&mut self, rhs: $ty) {
                *self = *self | rhs;
            }

        }


        impl ops::BitXorAssign<$ty> for $ty {
            fn bitxor_assign(&mut self, rhs: $ty) {
                *self = *self ^ rhs;
            }

        }
        
    };

}

macro_rules! enable_indexing_by {
    ($ty: ident) => {

        impl<T> ops::Index<$ty> for [T] {
            type Output = T;
            fn index(&self, idx: $ty) -> &Self::Output {
                &self[idx.0 as usize]
            }
        }

        impl<T> ops::IndexMut<$ty> for [T] {
            fn index_mut(&mut self, idx: $ty) -> &mut Self::Output {
                &mut self[idx.0 as usize]
            }
        }
    };
}

pub(crate) use enable_base_i32_operations_for_u32_on;
pub(crate) use enable_indexing_by;

enable_base_i32_operations_for_u32_on!(Square);
enable_base_i32_operations_for_u32_on!(File);
enable_base_i32_operations_for_u32_on!(Rank);
enable_base_i32_operations_for_u32_on!(CastlingRight);
enable_base_i32_operations_for_i32_on!(Direction);


enable_full_i32_operations_for_i32_on!(Value);
enable_full_operations_on!(Value);
//enable_full_i32_operations_for_i32_on!(Depth);

enable_bit_operations_on!(CastlingRight);
enable_bit_operations_on!(Bitboard);
enable_bit_assign_operations_on!(CastlingRight);

enable_indexing_by!(Square);
enable_indexing_by!(File);
enable_indexing_by!(Rank);
enable_indexing_by!(Color);
enable_indexing_by!(PieceType);
enable_indexing_by!(Piece);
enable_indexing_by!(CastlingRight);
enable_indexing_by!(Move);

