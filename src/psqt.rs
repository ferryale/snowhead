#[cfg(test)]
//#[macro_use]
mod psqt_test;
use crate::types::bitboard::*;
use crate::types::square::*;
use crate::types::piece::*;
use crate::types::r#move::*;
use crate::attacks::attack_bb::*;


use crate::position::*;



pub const MAX_PLY: i32 = 128;
pub const MAX_MATE_PLY: i32 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(pub i32);

impl Value {
    pub const ZERO     : Value = Value(0);
    pub const DRAW     : Value = Value(0);
    pub const KNOWN_WIN: Value = Value(10000);
    pub const MATE     : Value = Value(32000);
    pub const INFINITE : Value = Value(32001);
    pub const NONE     : Value = Value(32002);

    pub const PAWN  : Value = Value(100);
    pub const KIGHT: Value = Value(320);
    pub const BISHOP: Value = Value(330);
    pub const ROOK  : Value = Value(500);
    pub const QUEEN : Value = Value(900);

    pub const MATE_IN_MAX_PLY : Value =
        Value( Value::MATE.0 - MAX_MATE_PLY - MAX_PLY);
    pub const MATED_IN_MAX_PLY: Value =
        Value(-Value::MATE.0 + MAX_MATE_PLY + MAX_PLY);

    pub fn abs(self) -> Value {
        Value(self.0.abs())
    }
}

const PIECE_VALUE: [Value; PIECE_NB] = [Value(0), Value::PAWN, Value::KIGHT, Value::BISHOP, 
                    Value::ROOK, Value::QUEEN, Value(0), Value(0),
                    Value(0), Value::PAWN, Value::KIGHT, Value::BISHOP, 
                    Value::ROOK, Value::QUEEN, Value(0), Value(0),
                    ];



const BONUS: [[[i32; 4]; RANK_NB]; 7] = 
[
    [ // ALL_PIECES
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0],
        [ 0,  0,  0,  0]
    ],
    [ // PAWN
        [ 0,  0,  0,  0],
        [ 5, 10, 10,-20],
        [ 5, -5,-10,  0],
        [ 0,  0,  0, 20],
        [ 5,  5, 10, 25],
        [10, 10, 20, 30],
        [50, 50, 50, 50],
        [ 0,  0,  0,  0]
    ],
    [ // KNIGHT
        [-50,-40,-30,-30],
        [-40,-20,  0,  5],
        [-30,  5, 10, 15],
        [-30,  0, 15, 20],
        [-30,  5, 15, 20],
        [-30,  0, 10, 15],
        [-40,-20,  0,  0],
        [-50,-40,-30,-30]
    ],
    [ // BISHOP
        [-20,-10,-10,-10],
        [-10,  5,  0,  0],
        [-10, 10, 10, 10],
        [-10,  0, 10, 10],
        [-10,  5,  5, 10],
        [-10,  0,  5, 10],
        [-10,  0,  0,  0],
        [-20,-10,-10,-10]
    ],
    [ // ROOK
        [ 0,  0,  0,  5],
        [-5,  0,  0,  0],
        [-5,  0,  0,  0],
        [-5,  0,  0,  0],
        [-5,  0,  0,  0],
        [-5,  0,  0,  0],
        [ 5, 10, 10, 10],
        [ 0,  0,  0,  0]
    ],
    [ // QUEEN
        [-20,-10,-10, -5],
        [-10,  0,  5,  0],
        [-10,  5,  5,  5],
        [  0,  0,  5,  5],
        [ -5,  0,  5,  5],
        [-10,  0,  5,  5],
        [-10,  0,  0,  0],
        [-20,-10,-10, -5]
    ],
    [ // KING
        [ 20, 30, 10,  0],
        [ 20, 20,  0,  0],
        [-10,-20,-20,-20],
        [-20,-30,-30,-40],
        [-30,-40,-40,-50],
        [-30,-40,-40,-50],
        [-30,-40,-40,-50],
        [-30,-40,-40,-50],
    ]
];

pub const PSQ: [[Value; SQUARE_NB]; PIECE_NB] = init_psq();

pub const fn piece_value(pc: Piece) -> Value {
    PIECE_VALUE[pc.0 as usize]
}

pub fn psq(pc: Piece, s: Square) -> Value {
    PSQ[pc][s] 
}

const fn min_file(f1: File, f2: File) -> File {
    if f1.0 < f2.0 { f1 } else { f2 }
}

const fn init_psq() -> [[Value; SQUARE_NB]; PIECE_NB] {
    let mut psq_array = [[Value(0); SQUARE_NB]; PIECE_NB];
    let mut pc_idx = 1;

    while pc_idx < PIECE_TYPE_NB - 1 {

        let pc = Piece(pc_idx as u32);
        let bpc = Piece(pc.0 ^ 8);
        let v = piece_value(pc);

        let mut s_idx = 0;
        while s_idx < SQUARE_NB {
            let s = Square(s_idx as u32);
            let bs = Square(s.0 ^ Square::A8.0);

            let f = min_file(s.file(), File(FILE_H.0 - s.file().0));

            psq_array[pc.0 as usize][s.0 as usize] = Value(v.0
                + BONUS[pc.0 as usize][s.rank().0 as usize][f.0 as usize]);
            psq_array[bpc.0 as usize][bs.0 as usize] = 
                Value(-psq_array[pc.0 as usize][s.0 as usize].0);

            s_idx += 1;
        }

        pc_idx += 1;
    }

    psq_array
}
