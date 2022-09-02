use self::psqt::PsqTable;
use self::score::{Phase, Score, Value};
use cozy_chess::{Board, Color, File, Piece, Rank};

pub mod psqt;
pub mod score;

#[derive(Debug, Clone, Copy)]
pub struct Evaluator {
    pub psq_tables: [PsqTable; Color::NUM],
}

impl Evaluator {
    pub fn new(
        pc_table: &[[i16; Phase::NUM]; Piece::NUM],
        sq_tables: &[[[[i16; Phase::NUM]; File::NUM]; Rank::NUM]; Piece::NUM],
    ) -> Evaluator {
        let psqt = PsqTable::new(pc_table, sq_tables);

        Evaluator {
            psq_tables: [psqt, !psqt],
        }
    }

    pub fn default() -> Evaluator {
        Evaluator {
            psq_tables: [PsqTable::default(), !PsqTable::default()],
        }
    }

    pub fn evaluate(self, board: &Board) -> Value {
        let mut score = Score::ZERO;
        let mut pc: Piece;
        let mut c: Color;
        for sq in board.occupied() {
            pc = board.piece_on(sq).unwrap();
            c = board.color_on(sq).unwrap();
            score += self.psq_tables[c as usize].probe(&pc, &sq);
        }
        score.values[0]
    }
}
