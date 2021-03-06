#[cfg(test)]
//#[macro_use]
mod psqt_test;

use crate::types::square::*;
use crate::types::piece::*;
use crate::types::score::*;

macro_rules! S { ($x:expr, $y:expr) => (Score(($y << 16) + $x)) }


// Bonus is symmetric wrt ranks
const BONUS: [[[Score; 4]; FILE_NB]; 6] = 
[
  [  // Pawn
    [S!(0, 0), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(5, 5), S!(10, 10), S!(10, 10), S!(-20, -20)],
    [S!(5, 5), S!(-5, -5), S!(-10, -10), S!(0, 0)],
    [S!(0, 0), S!(0, 0), S!(0, 0), S!(20, 20)],
    [S!(5, 5), S!(5, 5), S!(10, 10), S!(25, 25)],
    [S!(10, 10), S!(10, 10), S!(20, 20), S!(30, 30)],
    [S!(50, 50), S!(50, 50), S!(50, 50), S!(50, 50)],
    [S!(0, 0), S!(0, 0), S!(0, 0), S!(0, 0)]
  ],
  [  // Knight
    [S!(-50, -50), S!(-40, -40), S!(-30, -30), S!(-30, -30)],
    [S!(-40, -40), S!(-20, -20), S!(0, 0), S!(5, 5)],
    [S!(-30, -30), S!(5, 5), S!(10, 10), S!(15, 15)],
    [S!(-30, -30), S!(0, 0), S!(15, 15), S!(20, 20)],
    [S!(-30, -30), S!(5, 5), S!(15, 15), S!(20, 20)],
    [S!(-30, -30), S!(0, 0), S!(10, 10), S!(15, 15)],
    [S!(-40, -40), S!(-20, -20), S!(0, 0), S!(0, 0)],
    [S!(-50, -50), S!(-40, -40), S!(-30, -30), S!(-30, -30)]
  ],
  [  // Bishop
    [S!(-20, -20), S!(-10, -10), S!(-10, -10), S!(-10, -10)],
    [S!(-10, -10), S!(5, 5), S!(0, 0), S!(0, 0)],
    [S!(-10, -10), S!(10, 10), S!(10, 10), S!(10, 10)],
    [S!(-10, -10), S!(0, 0), S!(10, 10), S!(10, 10)],
    [S!(-10, -10), S!(5, 5), S!(5, 5), S!(10, 10)],
    [S!(-10, -10), S!(0, 0), S!(5, 5), S!(10, 10)],
    [S!(-10, -10), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-20, -20), S!(-10, -10), S!(-10, -10), S!(-10, -10)]
  ],
  [  // Rook
    [S!(0, 0), S!(0, 0), S!(0, 0), S!(5, 5)],
    [S!(-5, -5), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-5, -5), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-5, -5), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-5, -5), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-5, -5), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(5, 5), S!(10, 10), S!(10, 10), S!(10, 10)],
    [S!(0, 0), S!(0, 0), S!(0, 0), S!(0, 0)]
  ],
  [  // Queen
    [S!(-20, -20), S!(-10, -10), S!(-10, -10), S!(-5, -5)],
    [S!(-10, -10), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-10, -10), S!(0, 0), S!(5, 5), S!(5, 5)],
    [S!(-5, -5), S!(0, 0), S!(5, 5), S!(5, 5)],
    [S!(-5, -5), S!(0, 0), S!(5, 5), S!(5, 5)],
    [S!(-10, -10), S!(0, 0), S!(5, 5), S!(5, 5)],
    [S!(-10, -10), S!(0, 0), S!(0, 0), S!(0, 0)],
    [S!(-20, -20), S!(-10, -10), S!(-10, -10), S!(-5, -5)]
  ],
  [  // King
    [S!(20, -50), S!(30, -30), S!(10, -30), S!(0, -30)],
    [S!(20, -30), S!(20, -30), S!(0, 0), S!(0, 0)],
    [S!(-10, -30), S!(-20, -10), S!(-20, 20), S!(-20, 30)],
    [S!(-20, -30), S!(-30, -10), S!(-30, 30), S!(-40, 40)],
    [S!(-30, -30), S!(-40, -10), S!(-40, 30), S!(-50, 40)],
    [S!(-30, -30), S!(-40, -10), S!(-40, 20), S!(-50, 30)],
    [S!(-30, -30), S!(-40, -20), S!(-40, -10), S!(-50, 0)],
    [S!(-30, -50), S!(-40, -40), S!(-40, -30), S!(-50, -20)]
  ]
];

// // Bonus is symmetric wrt ranks
// const BONUS: [[[Score; 4]; FILE_NB]; 6] = [
//   [ // Pawn
//    [ S!(  0, 0), S!(  0, 0), S!(  0, 0), S!( 0, 0) ],
//    [ S!(-11, 7), S!(  6,-4), S!(  7, 8), S!( 3,-2) ],
//    [ S!(-18,-4), S!( -2,-5), S!( 19, 5), S!(24, 4) ],
//    [ S!(-17, 3), S!( -9, 3), S!( 20,-8), S!(35,-3) ],
//    [ S!( -6, 8), S!(  5, 9), S!(  3, 7), S!(21,-6) ],
//    [ S!( -6, 8), S!( -8,-5), S!( -6, 2), S!(-2, 4) ],
//    [ S!( -4, 3), S!( 20,-9), S!( -8, 1), S!(-4,18) ],
//    [ S!(  0, 0), S!(  0, 0), S!(  0, 0), S!( 0, 0) ]
//   ],
//   [ // Knight
//    [ S!(-161,-105), S!(-96,-82), S!(-80,-46), S!(-73,-14) ],
//    [ S!( -83, -69), S!(-43,-54), S!(-21,-17), S!(-10,  9) ],
//    [ S!( -71, -50), S!(-22,-39), S!(  0, -7), S!(  9, 28) ],
//    [ S!( -25, -41), S!( 18,-25), S!( 43,  6), S!( 47, 38) ],
//    [ S!( -26, -46), S!( 16,-25), S!( 38,  3), S!( 50, 40) ],
//    [ S!( -11, -54), S!( 37,-38), S!( 56, -7), S!( 65, 27) ],
//    [ S!( -63, -65), S!(-19,-50), S!(  5,-24), S!( 14, 13) ],
//    [ S!(-195,-109), S!(-67,-89), S!(-42,-50), S!(-29,-13) ]
//   ],
//   [ // Bishop
//    [ S!(-44,-58), S!(-13,-31), S!(-25,-37), S!(-34,-19) ],
//    [ S!(-20,-34), S!( 20, -9), S!( 12,-14), S!(  1,  4) ],
//    [ S!( -9,-23), S!( 27,  0), S!( 21, -3), S!( 11, 16) ],
//    [ S!(-11,-26), S!( 28, -3), S!( 21, -5), S!( 10, 16) ],
//    [ S!(-11,-26), S!( 27, -4), S!( 16, -7), S!(  9, 14) ],
//    [ S!(-17,-24), S!( 16, -2), S!( 12,  0), S!(  2, 13) ],
//    [ S!(-23,-34), S!( 17,-10), S!(  6,-12), S!( -2,  6) ],
//    [ S!(-35,-55), S!(-11,-32), S!(-19,-36), S!(-29,-17) ]
//   ],
//   [ // Rook
//    [ S!(-25, 0), S!(-16, 0), S!(-16, 0), S!(-9, 0) ],
//    [ S!(-21, 0), S!( -8, 0), S!( -3, 0), S!( 0, 0) ],
//    [ S!(-21, 0), S!( -9, 0), S!( -4, 0), S!( 2, 0) ],
//    [ S!(-22, 0), S!( -6, 0), S!( -1, 0), S!( 2, 0) ],
//    [ S!(-22, 0), S!( -7, 0), S!(  0, 0), S!( 1, 0) ],
//    [ S!(-21, 0), S!( -7, 0), S!(  0, 0), S!( 2, 0) ],
//    [ S!(-12, 0), S!(  4, 0), S!(  8, 0), S!(12, 0) ],
//    [ S!(-23, 0), S!(-15, 0), S!(-11, 0), S!(-5, 0) ]
//   ],
//   [ // Queen
//    [ S!( 0,-71), S!(-4,-56), S!(-3,-42), S!(-1,-29) ],
//    [ S!(-4,-56), S!( 6,-30), S!( 9,-21), S!( 8, -5) ],
//    [ S!(-2,-39), S!( 6,-17), S!( 9, -8), S!( 9,  5) ],
//    [ S!(-1,-29), S!( 8, -5), S!(10,  9), S!( 7, 19) ],
//    [ S!(-3,-27), S!( 9, -5), S!( 8, 10), S!( 7, 21) ],
//    [ S!(-2,-40), S!( 6,-16), S!( 8,-10), S!(10,  3) ],
//    [ S!(-2,-55), S!( 7,-30), S!( 7,-21), S!( 6, -6) ],
//    [ S!(-1,-74), S!(-4,-55), S!(-1,-43), S!( 0,-30) ]
//   ],
//   [ // King
//    [ S!(267,  0), S!(320, 48), S!(270, 75), S!(195, 84) ],
//    [ S!(264, 43), S!(304, 92), S!(238,143), S!(180,132) ],
//    [ S!(200, 83), S!(245,138), S!(176,167), S!(110,165) ],
//    [ S!(177,106), S!(185,169), S!(148,169), S!(110,179) ],
//    [ S!(149,108), S!(177,163), S!(115,200), S!( 66,203) ],
//    [ S!(118, 95), S!(159,155), S!( 84,176), S!( 41,174) ],
//    [ S!( 87, 50), S!(128, 99), S!( 63,122), S!( 20,139) ],
//    [ S!( 63,  9), S!( 88, 55), S!( 47, 80), S!(  0, 90) ]
//   ]
// ];

// const BONUS: [[[i32; 4]; RANK_NB]; 7] = 
// [
//     [ // ALL_PIECES
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0],
//         [ 0,  0,  0,  0]
//     ],
//     [ // PAWN
//         [ 0,  0,  0,  0],
//         [ 5, 10, 10,-20],
//         [ 5, -5,-10,  0],
//         [ 0,  0,  0, 20],
//         [ 5,  5, 10, 25],
//         [10, 10, 20, 30],
//         [50, 50, 50, 50],
//         [ 0,  0,  0,  0]
//     ],
//     [ // KNIGHT
//         [-50,-40,-30,-30],
//         [-40,-20,  0,  5],
//         [-30,  5, 10, 15],
//         [-30,  0, 15, 20],
//         [-30,  5, 15, 20],
//         [-30,  0, 10, 15],
//         [-40,-20,  0,  0],
//         [-50,-40,-30,-30]
//     ],
//     [ // BISHOP
//         [-20,-10,-10,-10],
//         [-10,  5,  0,  0],
//         [-10, 10, 10, 10],
//         [-10,  0, 10, 10],
//         [-10,  5,  5, 10],
//         [-10,  0,  5, 10],
//         [-10,  0,  0,  0],
//         [-20,-10,-10,-10]
//     ],
//     [ // ROOK
//         [ 0,  0,  0,  5],
//         [-5,  0,  0,  0],
//         [-5,  0,  0,  0],
//         [-5,  0,  0,  0],
//         [-5,  0,  0,  0],
//         [-5,  0,  0,  0],
//         [ 5, 10, 10, 10],
//         [ 0,  0,  0,  0]
//     ],
//     [ // QUEEN
//         [-20,-10,-10, -5],
//         [-10,  0,  5,  0],
//         [-10,  5,  5,  5],
//         [  0,  0,  5,  5],
//         [ -5,  0,  5,  5],
//         [-10,  0,  5,  5],
//         [-10,  0,  0,  0],
//         [-20,-10,-10, -5]
//     ],
//     [ // KING
//         [ 20, 30, 10,  0],
//         [ 20, 20,  0,  0],
//         [-10,-20,-20,-20],
//         [-20,-30,-30,-40],
//         [-30,-40,-40,-50],
//         [-30,-40,-40,-50],
//         [-30,-40,-40,-50],
//         [-30,-40,-40,-50],
//     ]
// ];

pub const PSQ: [[Score; SQUARE_NB]; PIECE_NB] = init_psq();

pub fn psq(pc: Piece, s: Square) -> Score {
    PSQ[pc][s] 
}

pub const fn min_file(f1: File, f2: File) -> File {
    if f1.0 < f2.0 { f1 } else { f2 }
}

const fn init_psq() -> [[Score; SQUARE_NB]; PIECE_NB] {
    
    let mut psq_array = [[Score::ZERO; SQUARE_NB]; PIECE_NB];
    let mut pc_idx = 1;

    while pc_idx < PIECE_TYPE_NB - 1 {

        let pc = Piece(pc_idx as u32);
        let bpc = Piece(pc.0 ^ 8);
        let score = Score::make(piece_value(MG, pc), piece_value(EG, pc));

        let mut s_idx = 0;
        while s_idx < SQUARE_NB {
            let s = Square(s_idx as u32);
            let bs = Square(s.0 ^ Square::A8.0);

            let f = min_file(s.file(), File(FILE_H.0 - s.file().0));

            psq_array[pc.0 as usize][s.0 as usize] = Score(score.0
                + BONUS[(pc.0 - 1) as usize][s.rank().0 as usize][f.0 as usize].0);
            psq_array[bpc.0 as usize][bs.0 as usize] = 
                Score(-psq_array[pc.0 as usize][s.0 as usize].0);

            s_idx += 1;
        }

        pc_idx += 1;
    }

    psq_array
}
