use self::psqt::PsqTable;
use self::score::{Phase, Score, Value};
use core::iter::zip;
use cozy_chess::{Board, Color, File, Move, Piece, Rank, Square};

pub mod psqt;
pub mod score;

/*
    Evaluator struct.
    Stores the psq_tables, which can be passed via uci.
    The psq_stack is needed to undo a move.
*/
#[derive(Debug, Clone)]
pub struct Evaluator {
    psq_tables: [PsqTable; Color::NUM],
    psq: Score,
    psq_stack: Vec<Score>,
}

/* Default implementation for Evaluator */
impl Default for Evaluator {
    fn default() -> Evaluator {
        Evaluator {
            psq_tables: [PsqTable::default(), !PsqTable::default()],
            psq: Score::ZERO,
            psq_stack: vec![],
        }
    }
}

/* Evaluator implementation */
impl Evaluator {
    // A new evaluator requires piece and piece square tables
    pub fn new(
        pc_table: &[[i16; Phase::NUM]; Piece::NUM],
        sq_tables: &[[[[i16; Phase::NUM]; File::NUM]; Rank::NUM]; Piece::NUM],
    ) -> Evaluator {
        let psqt = PsqTable::new(pc_table, sq_tables);
        Evaluator {
            psq_tables: [psqt, !psqt],
            psq: Score::ZERO,
            psq_stack: vec![],
        }
    }

    // Returns the psq score
    pub fn psq(&self) -> Score {
        self.psq
    }

    // Returns a psq_score for a piece/square from the internal psq table
    pub fn probe_psqt(&self, c: Color, pc: Piece, sq: Square) -> Score {
        self.psq_tables[c as usize].probe(&pc, &sq)
    }

    // Computes the psq score of a position and stores it in psq
    pub fn compute_psq(&mut self, board: &Board) {
        self.psq = Score::ZERO;
        let mut pc: Piece;
        let mut c: Color;
        for sq in board.occupied() {
            pc = board.piece_on(sq).unwrap();
            c = board.color_on(sq).unwrap();
            self.psq += self.probe_psqt(c, pc, sq);
        }
    }

    // Incrementally updates the psq when a move is made
    pub fn do_move(&mut self, board: &Board, mv: Move) {
        let c = board.side_to_move();
        let pc_from = board.piece_on(mv.from).unwrap();
        let pc_to = board.piece_on(mv.to);

        // Get the en passant square
        let ep_square = board
            .en_passant()
            .map(|ep| Square::new(ep, Rank::Sixth.relative_to(c)));

        let is_castling = board.colors(c).has(mv.to);
        let is_enpassant = Some(mv.to) == ep_square;

        // Before computing new psq, push the old one in the stack
        self.psq_stack.push(self.psq.clone());

        // When castling, update both king and rook psq scores
        if is_castling {
            // Get the king and rook files
            let (kfile, rfile) = if mv.from.file() < mv.to.file() {
                // Short castle
                (File::G, File::F)
            } else {
                // Long castle
                (File::C, File::D)
            };
            let our_back_rank = Rank::First.relative_to(c);
            let kto = Square::new(kfile, our_back_rank);
            let rto = Square::new(rfile, our_back_rank);
            self.psq +=
                self.probe_psqt(c, Piece::King, kto) - self.probe_psqt(c, Piece::King, mv.from);
            self.psq +=
                self.probe_psqt(c, Piece::Rook, rto) - self.probe_psqt(c, Piece::Rook, mv.to);
        } else {
            // When en passant, remove the captured pawn
            if is_enpassant {
                let capsq = Square::new(mv.to.file(), Rank::Fifth.relative_to(c));
                let captured = board.piece_on(capsq).unwrap();
                self.psq -= self.probe_psqt(!c, captured, capsq);
            // If there is a capture, remove the capture piece
            } else if let Some(captured) = pc_to {
                self.psq -= self.probe_psqt(!c, captured, mv.to);
            } // if/else is en_passant

            // If there is a promotion, pc_to is replaced by the promoted piece
            if let Some(promoted) = mv.promotion {
                self.psq +=
                    self.probe_psqt(c, promoted, mv.to) - self.probe_psqt(c, pc_from, mv.from);
            } else {
                self.psq +=
                    self.probe_psqt(c, pc_from, mv.to) - self.probe_psqt(c, pc_from, mv.from);
            } // if/else promotion
        } // if/else is_castling
    }

    // Updates eval after a null move
    pub fn do_null_move(&mut self) {
        self.psq_stack.push(self.psq.clone());
    }

    // Unmakes a move by popping the prev psq from the stack
    pub fn undo_move(&mut self) {
        self.psq = self.psq_stack.pop().unwrap();
    }

    // Returns the tapered position evaluation
    pub fn evaluate(&self, board: &Board) -> Value {
        // Compute the game phase
        let mut phase = Phase::ZERO;
        for (&pc, &ph) in zip(&Piece::ALL, &Phase::ALL) {
            phase += ph * board.pieces(pc).len();
        }

        // Compute the tapered evaluation
        let egs = self.psq.values[1];
        let mgs = self.psq.values[0];
        let value = egs + (mgs - egs) * phase / Phase::MIDGAME;

        // Negate the evaluator fot black
        if board.side_to_move() == Color::White {
            value
        } else {
            -value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Evaluator;
    use crate::bench::epd::EpdEntry;
    use crate::position::Position;
    use cozy_chess::{Color, Move, Piece, Square};
    use std::fs;

    #[test]
    fn psqt_are_symmetric() {
        let evaluator = Evaluator::default();

        let test_squares = [
            (Square::A1, Square::A8),
            (Square::D2, Square::D7),
            (Square::E6, Square::E3),
            (Square::H5, Square::H4),
        ];

        for &pc in &Piece::ALL {
            for sq_tuple in test_squares {
                let (wsq, bsq) = sq_tuple;
                let score1 = evaluator.probe_psqt(Color::White, pc, wsq);
                let score2 = evaluator.probe_psqt(Color::Black, pc, bsq);
                println!("{score1:?} {score2:?}");
                assert_eq!(score1, -score2);
            }
        }
    }

    #[test]
    fn incremental_and_bulk_evals_match() {
        let contents = fs::read_to_string("./src/bench/bench.epd");

        for line in contents.expect("File 'bench.epd' not found").split("\n") {
            let epd_entry = EpdEntry::new(line);
            let mut pos = Position::new().fen(&epd_entry.fen).build();
            pos.init_psq();

            let mut move_list: Vec<Move> = vec![];
            pos.board().generate_moves(|piece_moves| {
                for mv in piece_moves {
                    move_list.push(mv);
                }
                false
            });

            for mv in move_list {
                let psq_before = pos.psq();
                pos.do_move(mv);
                println!("{}, {}", mv, pos.board());
                assert_eq!(pos.psq(), pos.compute_psq());
                pos.undo_move();

                let psq_after = pos.psq();
                assert_eq!(psq_before, psq_after);
            }
        }
    }
}
