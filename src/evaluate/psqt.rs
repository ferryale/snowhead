use crate::evaluate::score::{Phase, Score};
use core::ops::Not;
use cozy_chess::{File, Piece, Rank, Square};

pub const PIECE_VALUES: [[i16; Phase::NUM]; Piece::NUM] = [
    [100, 100],
    [320, 320],
    [330, 330],
    [500, 500],
    [900, 900],
    [20000, 20000],
];

#[rustfmt::skip]
pub const PSQ_BONUS: [[[[i16;Phase::NUM]; File::NUM]; Rank::NUM]; Piece::NUM] = 
[
    [
        [[0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0]],
        [[5,5], [10,10], [10,10], [-20,-20], [-20,-20], [10,10], [10,10], [5,5]],
        [[5,5], [-5,-5], [-10,-10], [0,0], [0,0], [-10,-10], [-5,-5], [5,5]],
        [[0,0], [0,0], [0,0], [20,20], [20,20], [0,0], [0,0], [0,0]],
        [[5,5], [5,5], [10,10], [25,25], [25,25], [10,10], [5,5], [5,5]],
        [[10,10], [10,10], [20,20], [30,30], [30,30], [20,20], [10,10], [10,10]],
        [[50,50], [50,50], [50,50], [50,50], [50,50], [50,50], [50,50], [50,50]],
        [[0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0]]
    ],
    [
        [[-50,-50], [-40,-40], [-30,-30], [-30,-30], [-30,-30], [-30,-30], [-40,-40], [-50,-50]],
        [[-40,-40], [-20,-20], [0,0], [5,5], [5,5], [0,0], [-20,-20], [-40,-40]],
        [[-30,-30], [5,5], [10,10], [15,15], [15,15], [10,10], [5,5], [-30,-30]],
        [[-30,-30], [0,0], [15,15], [20,20], [20,20], [15,15], [0,0], [-30,-30]],
        [[-30,-30], [5,5], [15,15], [20,20], [20,20], [15,15], [5,5], [-30,-30]],
        [[-30,-30], [0,0], [10,10], [15,15], [15,15], [10,10], [0,0], [-30,-30]],
        [[-40,-40], [-20,-20], [0,0], [0,0], [0,0], [0,0], [-20,-20], [-40,-40]],
        [[-50,-50], [-40,-40], [-30,-30], [-30,-30], [-30,-30], [-30,-30], [-40,-40], [-50,-50]]
    ],
    [
        [[-20,-20], [-10,-10], [-10,-10], [-10,-10], [-10,-10], [-10,-10], [-10,-10], [-20,-20]],
        [[-10,-10], [5,5], [0,0], [0,0], [0,0], [0,0], [5,5], [-10,-10]],
        [[-10,-10], [10,10], [10,10], [10,10], [10,10], [10,10], [10,10], [-10,-10]],
        [[-10,-10], [0,0], [10,10], [10,10], [10,10], [10,10], [0,0], [-10,-10]],
        [[-10,-10], [5,5], [5,5], [10,10], [10,10], [5,5], [5,5], [-10,-10]],
        [[-10,-10], [0,0], [5,5], [10,10], [10,10], [5,5], [0,0], [-10,-10]],
        [[-10,-10], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-10,-10]],
        [[-20,-20], [-10,-10], [-10,-10], [-10,-10], [-10,-10], [-10,-10], [-10,-10], [-20,-20]]
    ],
    [
        [[0,0], [0,0], [0,0], [5,5], [5,5], [0,0], [0,0], [0,0]],
        [[-5,-5], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-5,-5]],
        [[-5,-5], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-5,-5]],
        [[-5,-5], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-5,-5]],
        [[-5,-5], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-5,-5]],
        [[-5,-5], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-5,-5]],
        [[5,5], [10,10], [10,10], [10,10], [10,10], [10,10], [10,10], [5,5]],
        [[0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0]]
    ],
    [
        [[-20,-20], [-10,-10], [-10,-10], [-5,-5], [-5,-5], [-10,-10], [-10,-10], [-20,-20]],
        [[-10,-10], [0,0], [5,5], [0,0], [0,0], [0,0], [0,0], [-10,-10]],
        [[-10,-10], [5,5], [5,5], [5,5], [5,5], [5,5], [0,0], [-10,-10]],
        [[0,0], [0,0], [5,5], [5,5], [5,5], [5,5], [0,0], [-5,-5]],
        [[-5,-5], [0,0], [5,5], [5,5], [5,5], [5,5], [0,0], [-5,-5]],
        [[-10,-10], [0,0], [5,5], [5,5], [5,5], [5,5], [0,0], [-10,-10]],
        [[-10,-10], [0,0], [0,0], [0,0], [0,0], [0,0], [0,0], [-10,-10]],
        [[-20,-20], [-10,-10], [-10,-10], [-5,-5], [-5,-5], [-10,-10], [-10,-10], [-20,-20]]
    ],
    [
        [[20,-50], [30,-30], [10,-30], [0,-30], [0,-30], [10,-30], [30,-30], [20,-50]],
        [[20,-30], [20,-30], [0,0], [0,0], [0,0], [0,0], [20,-30], [20,-30]],
        [[-10,-30], [-20,-10], [-20,20], [-20,30], [-20,30], [-20,20], [-20,-10], [-10,-30]],
        [[-20,-30], [-30,-10], [-30,30], [-40,40], [-40,40], [-30,30], [-30,-10], [-20,-30]],
        [[-30,-30], [-40,-10], [-40,30], [-50,40], [-50,40], [-40,30], [-40,-10], [-30,-30]],
        [[-30,-30], [-40,-10], [-40,20], [-50,30], [-50,30], [-40,20], [-40,-10], [-30,-30]],
        [[-30,-30], [-40,-20], [-40,-10], [-50,0], [-50,0], [-40,-10], [-40,-20], [-30,-30]],
        [[-30,-50], [-40,-40], [-40,-30], [-50,-20], [-50,-20], [-40,-30], [-40,-40], [-30,-50]]
    ]
];

/* SquareTable: array of Scores for each Square */
#[derive(Debug, Clone, Copy)]
pub struct SquareTable {
    pub scores: [Score; Square::NUM],
}

/* PieceTable: array of Scores for each Piece */
#[derive(Debug, Clone, Copy)]
pub struct PieceTable {
    pub scores: [Score; Piece::NUM],
}

/* PsqTable: 2D array of Scores for each Piece/Square */
#[derive(Debug, Clone, Copy)]
pub struct PsqTable {
    pub scores: [[Score; Square::NUM]; Piece::NUM],
}

/* SquareTable implementation */
impl SquareTable {
    pub const ZERO: SquareTable = SquareTable::default();

    // Const default, no Default trait
    pub const fn default() -> SquareTable {
        SquareTable {
            scores: [Score::ZERO; Square::NUM],
        }
    }

    // Init a square table via a 3D array of of i16 for each Rank, File, Phase
    pub fn new(table: &[[[i16; Phase::NUM]; File::NUM]; Rank::NUM]) -> SquareTable {
        let mut sqt = SquareTable::ZERO;
        for &f in &File::ALL {
            for &r in &Rank::ALL {
                sqt.scores[Square::new(f, r) as usize] = Score::new(&table[r as usize][f as usize]);
            }
        }
        sqt
    }
}

/* PieceTable implementation */
impl PieceTable {
    pub const ZERO: PieceTable = PieceTable {
        scores: [Score::ZERO; Piece::NUM],
    };

    pub fn new(table: &[[i16; Phase::NUM]; Piece::NUM]) -> PieceTable {
        let mut piece_table = PieceTable::ZERO;
        for &pc in &Piece::ALL {
            piece_table.scores[pc as usize] = Score::new(&table[pc as usize]);
        }
        piece_table
    }

    pub fn default() -> PieceTable {
        PieceTable::new(&PIECE_VALUES)
    }
}

impl PsqTable {
    pub const ZERO: PsqTable = PsqTable {
        scores: [[Score::ZERO; Square::NUM]; Piece::NUM],
    };

    pub fn new(
        pc_table: &[[i16; Phase::NUM]; Piece::NUM],
        sq_tables: &[[[[i16; Phase::NUM]; File::NUM]; Rank::NUM]; Piece::NUM],
    ) -> PsqTable {
        let mut psqt = PsqTable::ZERO;
        for &pc in &Piece::ALL {
            for &f in &File::ALL {
                for &r in &Rank::ALL {
                    psqt.scores[pc as usize][Square::new(f, r) as usize] =
                        Score::new(&pc_table[pc as usize])
                            + Score::new(&sq_tables[pc as usize][r as usize][f as usize]);
                }
            }
        }
        psqt
    }

    pub fn probe(self, pc: &Piece, sq: &Square) -> Score {
        self.scores[*pc as usize][*sq as usize]
    }

    pub fn default() -> PsqTable {
        PsqTable::new(&PIECE_VALUES, &PSQ_BONUS)
    }
}

impl Not for PsqTable {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut ret = self.clone();
        for &pc in &Piece::ALL {
            for &sq in &Square::ALL {
                ret.scores[pc as usize][sq.flip_rank() as usize] =
                    -self.scores[pc as usize][sq as usize];
            }
        }
        ret
    }
}
