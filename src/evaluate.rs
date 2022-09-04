use self::psqt::PsqTable;
use self::score::{Phase, Score};
use cozy_chess::{Board, Color, File, Piece, Rank, Move, Square};

pub mod psqt;
pub mod score;

#[derive(Debug, Clone)]
pub struct Evaluator {
    pub psq_tables: [PsqTable; Color::NUM],
    pub psq: Score,
    psq_stack: Vec<Score>,
}

impl Evaluator {
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

    pub fn probe_psqt(&self, c: Color, pc: Piece, sq: Square) -> Score {
        self.psq_tables[c as usize].probe(&pc, &sq)

    }

    pub fn default() -> Evaluator {
        Evaluator {
            psq_tables: [PsqTable::default(), !PsqTable::default()],
            psq: Score::ZERO,
            psq_stack: vec![],
        }
    }

    pub fn eval_psq(&mut self, board: &Board) {
        self.psq = Score::ZERO;
        let mut pc: Piece;
        let mut c: Color;
        for sq in board.occupied() {
            pc = board.piece_on(sq).unwrap();
            c = board.color_on(sq).unwrap();
            self.psq += self.probe_psqt(c, pc, sq);
        }

    }

    pub fn evaluate(&self, board: &Board) -> Score {
        let mut score = Score::ZERO;
        let mut pc: Piece;
        let mut c: Color;
        for sq in board.occupied() {
            pc = board.piece_on(sq).unwrap();
            c = board.color_on(sq).unwrap();
            score += self.probe_psqt(c, pc, sq);
        }

        if board.side_to_move() == Color::White {
            score
        } else {
            -score
        }
        
    }

    pub fn do_move(&mut self, board: &Board, mv: Move, next_board: &Board) {
        let c = board.side_to_move();
        let pc_from = board.piece_on(mv.from).unwrap();
        let pc_to = board.piece_on(mv.to);

        let ep_square = board.en_passant().map(|ep| {
                            Square::new(ep, Rank::Sixth.relative_to(c))
                        });

        let is_castling = board.colors(c).has(mv.to);
        let is_enpassant = Some(mv.to) == ep_square;

        self.psq_stack.push(self.psq.clone());
        
        if is_castling || is_enpassant {
            self.eval_psq(next_board);
        } else {
            if let Some(captured) = pc_to {
                self.psq -= self.probe_psqt(!c, captured, mv.to);
            } 
            
            if let Some(promoted) = mv.promotion {
                self.psq += self.probe_psqt(c, promoted, mv.to) - self.probe_psqt(c, pc_from, mv.from);
            } else {
                self.psq += self.probe_psqt(c, pc_from, mv.to) - self.probe_psqt(c, pc_from, mv.from);

            }

        }
             
    }

    pub fn undo_move(&mut self) {
        self.psq = self.psq_stack.pop().unwrap();
    }


}

#[cfg(test)]
mod tests{
    use super::{Evaluator};
    use crate::position::Position;
    use cozy_chess::{Square, Piece, Color, Move, Board};


    #[test]
    fn psqt_are_symmetric() {
        let evaluator = Evaluator::default();

        let test_squares = [(Square::A1, Square::A8), (Square::D2, Square::D7), (Square::E6, Square::E3), (Square::H5, Square::H4)];
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
    fn play_moves() {

        struct PerftInfo {
            fen: String,
            depth: i32,
            nodes: usize,
            chess960: bool
        }

        impl PerftInfo {
            fn new(fen: &str, depth: i32, nodes: usize, chess960: bool) -> PerftInfo {
                PerftInfo {
                    fen: String::from(fen), 
                    depth: depth,
                    nodes: nodes,
                    chess960: chess960,
                }
            }

            fn change_color(&mut self) {
                if self.fen.contains("w") {
                    self.fen = self.fen.replace("w", "b");
                } else if self.fen.contains("b") {
                    self.fen = self.fen.replace("b", "w");
                }
                
            }
        
        }



        // Enable debug to run extra tests on John Merlino's test positions.
        let debug = false;

        // http://www.talkchess.com/forum3/viewtopic.php?t=59046
        // Martin Sedlak's test positions
        // (http://www.talkchess.com/forum/viewtopic.php?t=47318)
        let mut perft_data = vec!(
           // avoid illegal ep
           PerftInfo::new( "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",         6, 1134888, false ),
           PerftInfo::new( "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1",         6, 1134888, false ),
           PerftInfo::new( "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",         6, 1015133, false ),
           PerftInfo::new( "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1",         6, 1015133, false ),
           // en passant capture checks opponent: 
           PerftInfo::new( "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1",         6, 1440467, false ),
           PerftInfo::new( "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1",         6, 1440467, false ),
           // short castling gives check: 
           PerftInfo::new( "5k2/8/8/8/8/8/8/4K2R w K - 0 1",            6, 661072,false ),
           PerftInfo::new( "4k2r/8/8/8/8/8/8/5K2 b k - 0 1",            6, 661072,false ),
           // long castling gives check: 
           PerftInfo::new( "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",            6, 803711, false ),
           PerftInfo::new( "r3k3/8/8/8/8/8/8/3K4 b q - 0 1",            6, 803711, false ),
           // castling (including losing cr due to rook capture): 
           PerftInfo::new( "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",   4, 1274206, false ),
           PerftInfo::new( "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1",    4, 1274206, false ),
           // castling prevented: 
           PerftInfo::new( "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",   4, 1720476, false ),
           PerftInfo::new( "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1",   4, 1720476, false ),
           // promote out of check: 
           PerftInfo::new( "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1",         6, 3821001, false ),
           PerftInfo::new( "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1",         6, 3821001, false ),
           // discovered check: 
           PerftInfo::new( "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",         5, 1004658, false ),
           PerftInfo::new( "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1",         5, 1004658, false ),
           // promote to give check: 
           PerftInfo::new( "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",            6, 217342, false ),
           PerftInfo::new( "8/k7/8/8/8/8/1p6/4K3 b - - 0 1",            6, 217342, false ),
           // underpromote to check: 
           PerftInfo::new( "8/P1k5/K7/8/8/8/8/8 w - - 0 1",            6, 92683, false ),
           PerftInfo::new( "8/8/8/8/8/k7/p1K5/8 b - - 0 1",            6, 92683, false ),
           // self stalemate: 
           PerftInfo::new( "K1k5/8/P7/8/8/8/8/8 w - - 0 1",            6, 2217, false ),
           PerftInfo::new( "8/8/8/8/8/p7/8/k1K5 b - - 0 1",            6, 2217, false ),
           // stalemate/checkmate: 
           PerftInfo::new( "8/k1P5/8/1K6/8/8/8/8 w - - 0 1",            7, 567584, false ),
           PerftInfo::new( "8/8/8/8/1k6/8/K1p5/8 b - - 0 1",            7, 567584, false ),
           // double check: 
           PerftInfo::new( "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1",         4, 23527, false ),
           PerftInfo::new( "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1",         4, 23527, false ),

           // short castling impossible although the rook never moved away from its corner 
           PerftInfo::new( "1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1", 5, 1063513, false ),
           PerftInfo::new( "4k2r/8/8/7r/8/8/1B6/1K6 w k - 0 1", 5, 1063513, false ),

           // long castling impossible although the rook never moved away from its corner 
           PerftInfo::new( "1k6/8/8/8/R7/1n6/8/R3K3 b Q - 0 1", 5, 346695, false ),
           PerftInfo::new( "r3k3/8/1N6/r7/8/8/8/1K6 w q - 0 1", 5, 346695, false ),

           // From the Wiki
           PerftInfo::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 4, 4085603, false ),
           PerftInfo::new( "rnbqkb1r/pp1p1ppp/2p5/4P3/2B5/8/PPP1NnPP/RNBQK2R w KQkq - 0 6", 3, 53392, false ),

           // Shortened form of the third position below
           PerftInfo::new( "8/7p/p5pb/4k3/P1pPn3/8/P5PP/1rB2RK1 b - d3 0 28", 4, 67197, false ),
        );

        for mut info in perft_data {

            let mut pos = Position::new_uci(&info.fen);
            pos.init_psq();
            //let moves = ["e2e4", "e7e6", "d1h5", "d8e7", "e1d1"];

            let mut move_list: Vec<Move> = vec![];
            pos.board.generate_moves(|piece_moves| {
                for mv in piece_moves {
                    move_list.push(mv);
                }
                false
            });

            for mv in move_list {
                
                let psq_before = pos.evaluator.psq;
                pos.do_move(mv);
                println!("{}, {}", mv, pos.board);
                
                assert_eq!(pos.psq(), pos.eval_psq());
                pos.undo_move();
                let psq_after = pos.evaluator.psq;
                assert_eq!(psq_before, psq_after);
                //assert_eq!(psq_before, psq_after);
                
                // assert_eq!(format!("{}", pos.board), expected);
                // assert_eq!(pos.board.hash(), expected.parse::<Board>().unwrap().hash());
            }
        }

        // let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        // let mut pos = Position::new_uci(&fen);
        // pos.init_psq();

        // const MOVES: &[(&str, &str)] = &[
        //     ("f3f5", "r3k2r/p1ppqpb1/bn2pnp1/3PNQ2/1p2P3/2N4p/PPPBBPPP/R3K2R b KQkq - 1 1"),
        //     ("h3g2", "r3k2r/p1ppqpb1/bn2pnp1/3PNQ2/1p2P3/2N5/PPPBBPpP/R3K2R w KQkq - 0 2"),
        //     ("e5g6", "r3k2r/p1ppqpb1/bn2pnN1/3P1Q2/1p2P3/2N5/PPPBBPpP/R3K2R b KQkq - 0 2"),
        //     ("g2h1r", "r3k2r/p1ppqpb1/bn2pnN1/3P1Q2/1p2P3/2N5/PPPBBP1P/R3K2r w Qkq - 0 3"),
        //     ("e2f1", "r3k2r/p1ppqpb1/bn2pnN1/3P1Q2/1p2P3/2N5/PPPB1P1P/R3KB1r b Qkq - 1 3"),
        //     ("f7g6", "r3k2r/p1ppq1b1/bn2pnp1/3P1Q2/1p2P3/2N5/PPPB1P1P/R3KB1r w Qkq - 0 4"),
        //     ("d2h6", "r3k2r/p1ppq1b1/bn2pnpB/3P1Q2/1p2P3/2N5/PPP2P1P/R3KB1r b Qkq - 1 4"),
        //     ("e7d6", "r3k2r/p1pp2b1/bn1qpnpB/3P1Q2/1p2P3/2N5/PPP2P1P/R3KB1r w Qkq - 2 5"),
        //     ("f2f4", "r3k2r/p1pp2b1/bn1qpnpB/3P1Q2/1p2PP2/2N5/PPP4P/R3KB1r b Qkq f3 0 5"),
        //     ("e8a8", "2kr3r/p1pp2b1/bn1qpnpB/3P1Q2/1p2PP2/2N5/PPP4P/R3KB1r w Q - 1 6"),
        //     ("f5h5", "2kr3r/p1pp2b1/bn1qpnpB/3P3Q/1p2PP2/2N5/PPP4P/R3KB1r b Q - 2 6"),
        //     ("f6e4", "2kr3r/p1pp2b1/bn1qp1pB/3P3Q/1p2nP2/2N5/PPP4P/R3KB1r w Q - 0 7"),
        //     ("a2a4", "2kr3r/p1pp2b1/bn1qp1pB/3P3Q/Pp2nP2/2N5/1PP4P/R3KB1r b Q a3 0 7"),
        //     ("b4a3", "2kr3r/p1pp2b1/bn1qp1pB/3P3Q/4nP2/p1N5/1PP4P/R3KB1r w Q - 0 8"),
        //     ("c3d1", "2kr3r/p1pp2b1/bn1qp1pB/3P3Q/4nP2/p7/1PP4P/R2NKB1r b Q - 1 8"),
        //     ("a6b5", "2kr3r/p1pp2b1/1n1qp1pB/1b1P3Q/4nP2/p7/1PP4P/R2NKB1r w Q - 2 9"),
        //     ("h6g7", "2kr3r/p1pp2B1/1n1qp1p1/1b1P3Q/4nP2/p7/1PP4P/R2NKB1r b Q - 0 9"),
        //     ("d6d5", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/4nP2/p7/1PP4P/R2NKB1r w Q - 0 10"),
        //     ("b2b4", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P2nP2/p7/2P4P/R2NKB1r b Q b3 0 10"),
        //     ("e4d2", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P3P2/p7/2Pn3P/R2NKB1r w Q - 1 11"),
        //     ("a1b1", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P3P2/p7/2Pn3P/1R1NKB1r b - - 2 11"),
        //     ("h1h2", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P3P2/p7/2Pn3r/1R1NKB2 w - - 0 12"),
        //     ("b1c1", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P3P2/p7/2Pn3r/2RNKB2 b - - 1 12"),
        //     ("d2b3", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P3P2/pn6/2P4r/2RNKB2 w - - 2 13"),
        //     ("d1b2", "2kr3r/p1pp2B1/1n2p1p1/1b1q3Q/1P3P2/pn6/1NP4r/2R1KB2 b - - 3 13"),
        //     ("c7c6", "2kr3r/p2p2B1/1np1p1p1/1b1q3Q/1P3P2/pn6/1NP4r/2R1KB2 w - - 0 14"),
        //     ("h5h6", "2kr3r/p2p2B1/1np1p1pQ/1b1q4/1P3P2/pn6/1NP4r/2R1KB2 b - - 1 14"),
        //     ("d5d6", "2kr3r/p2p2B1/1npqp1pQ/1b6/1P3P2/pn6/1NP4r/2R1KB2 w - - 2 15"),
        //     ("h6h2", "2kr3r/p2p2B1/1npqp1p1/1b6/1P3P2/pn6/1NP4Q/2R1KB2 b - - 0 15"),
        //     ("d6d1", "2kr3r/p2p2B1/1np1p1p1/1b6/1P3P2/pn6/1NP4Q/2RqKB2 w - - 1 16"),
        //     ("e1d1", "2kr3r/p2p2B1/1np1p1p1/1b6/1P3P2/pn6/1NP4Q/2RK1B2 b - - 0 16"),
        //     ("d7d6", "2kr3r/p5B1/1nppp1p1/1b6/1P3P2/pn6/1NP4Q/2RK1B2 w - - 0 17")
        // ];


        // for &(mv, expected) in MOVES {
        //     println!("{}, {}", mv, pos.board.hash());
        //     pos.do_move(mv.parse().unwrap());
        //     assert_eq!(pos.psq(), pos.eval_psq());
            
        //     // assert_eq!(format!("{}", pos.board), expected);
        //     // assert_eq!(pos.board.hash(), expected.parse::<Board>().unwrap().hash());
        // }
    }

    
    fn evaluation_is_symmetric() {

        struct PerftInfo {
            fen: String,
            depth: i32,
            nodes: usize,
            chess960: bool
        }

        impl PerftInfo {
            fn new(fen: &str, depth: i32, nodes: usize, chess960: bool) -> PerftInfo {
                PerftInfo {
                    fen: String::from(fen), 
                    depth: depth,
                    nodes: nodes,
                    chess960: chess960,
                }
            }

            fn change_color(&mut self) {
                if self.fen.contains("w") {
                    self.fen = self.fen.replace("w", "b");
                } else if self.fen.contains("b") {
                    self.fen = self.fen.replace("b", "w");
                }
                
            }
        
        }



        // Enable debug to run extra tests on John Merlino's test positions.
        let debug = false;

        // http://www.talkchess.com/forum3/viewtopic.php?t=59046
        // Martin Sedlak's test positions
        // (http://www.talkchess.com/forum/viewtopic.php?t=47318)
        let mut perft_data = vec!(
           // avoid illegal ep
           PerftInfo::new( "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",         6, 1134888, false ),
           PerftInfo::new( "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1",         6, 1134888, false ),
           PerftInfo::new( "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",         6, 1015133, false ),
           PerftInfo::new( "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1",         6, 1015133, false ),
           // en passant capture checks opponent: 
           PerftInfo::new( "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1",         6, 1440467, false ),
           PerftInfo::new( "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1",         6, 1440467, false ),
           // short castling gives check: 
           PerftInfo::new( "5k2/8/8/8/8/8/8/4K2R w K - 0 1",            6, 661072,false ),
           PerftInfo::new( "4k2r/8/8/8/8/8/8/5K2 b k - 0 1",            6, 661072,false ),
           // long castling gives check: 
           PerftInfo::new( "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",            6, 803711, false ),
           PerftInfo::new( "r3k3/8/8/8/8/8/8/3K4 b q - 0 1",            6, 803711, false ),
           // castling (including losing cr due to rook capture): 
           PerftInfo::new( "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",   4, 1274206, false ),
           PerftInfo::new( "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1",    4, 1274206, false ),
           // castling prevented: 
           PerftInfo::new( "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",   4, 1720476, false ),
           PerftInfo::new( "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1",   4, 1720476, false ),
           // promote out of check: 
           PerftInfo::new( "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1",         6, 3821001, false ),
           PerftInfo::new( "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1",         6, 3821001, false ),
           // discovered check: 
           PerftInfo::new( "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",         5, 1004658, false ),
           PerftInfo::new( "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1",         5, 1004658, false ),
           // promote to give check: 
           PerftInfo::new( "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",            6, 217342, false ),
           PerftInfo::new( "8/k7/8/8/8/8/1p6/4K3 b - - 0 1",            6, 217342, false ),
           // underpromote to check: 
           PerftInfo::new( "8/P1k5/K7/8/8/8/8/8 w - - 0 1",            6, 92683, false ),
           PerftInfo::new( "8/8/8/8/8/k7/p1K5/8 b - - 0 1",            6, 92683, false ),
           // self stalemate: 
           PerftInfo::new( "K1k5/8/P7/8/8/8/8/8 w - - 0 1",            6, 2217, false ),
           PerftInfo::new( "8/8/8/8/8/p7/8/k1K5 b - - 0 1",            6, 2217, false ),
           // stalemate/checkmate: 
           PerftInfo::new( "8/k1P5/8/1K6/8/8/8/8 w - - 0 1",            7, 567584, false ),
           PerftInfo::new( "8/8/8/8/1k6/8/K1p5/8 b - - 0 1",            7, 567584, false ),
           // double check: 
           PerftInfo::new( "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1",         4, 23527, false ),
           PerftInfo::new( "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1",         4, 23527, false ),

           // short castling impossible although the rook never moved away from its corner 
           PerftInfo::new( "1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1", 5, 1063513, false ),
           PerftInfo::new( "4k2r/8/8/7r/8/8/1B6/1K6 w k - 0 1", 5, 1063513, false ),

           // long castling impossible although the rook never moved away from its corner 
           PerftInfo::new( "1k6/8/8/8/R7/1n6/8/R3K3 b Q - 0 1", 5, 346695, false ),
           PerftInfo::new( "r3k3/8/1N6/r7/8/8/8/1K6 w q - 0 1", 5, 346695, false ),

           // From the Wiki
           PerftInfo::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 4, 4085603, false ),
           PerftInfo::new( "rnbqkb1r/pp1p1ppp/2p5/4P3/2B5/8/PPP1NnPP/RNBQK2R w KQkq - 0 6", 3, 53392, false ),

           // Shortened form of the third position below
           PerftInfo::new( "8/7p/p5pb/4k3/P1pPn3/8/P5PP/1rB2RK1 b - d3 0 28", 4, 67197, false ),

           // Some FRC postions by Reinhard Scharnagl
           // (http://www.talkchess.com/forum/viewtopic.php?t=55274)
           // We have each of them twice, to get the number of moves at the root
           // correct too.
           PerftInfo::new( "r1k1r2q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K1R2Q w KQkq - 0 1", 1, 23, true ),
           PerftInfo::new( "r1k2r1q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K2R1Q w KQkq - 0 1", 1, 28, true ),
           PerftInfo::new( "8/8/8/4B2b/6nN/8/5P2/2R1K2k w Q - 0 1", 1, 34, true ),
           PerftInfo::new( "2r5/8/8/8/8/8/6PP/k2KR3 w K - 0 1", 1, 17, true ),
           PerftInfo::new( "4r3/3k4/8/8/8/8/6PP/qR1K1R2 w KQ - 0 1", 1, 19, true ),

           PerftInfo::new( "r1k1r2q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K1R2Q w KQkq - 0 1", 2, 522, true ),
           PerftInfo::new( "r1k2r1q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K2R1Q w KQkq - 0 1", 2, 738, true ),
           PerftInfo::new( "8/8/8/4B2b/6nN/8/5P2/2R1K2k w Q - 0 1", 2, 318, true ),
           PerftInfo::new( "2r5/8/8/8/8/8/6PP/k2KR3 w K - 0 1", 2, 242, true ),
           PerftInfo::new( "4r3/3k4/8/8/8/8/6PP/qR1K1R2 w KQ - 0 1", 2, 628, true ),

           PerftInfo::new( "r1k1r2q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K1R2Q w KQkq - 0 1", 5, 7096972, true ), 
           PerftInfo::new( "r1k2r1q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K2R1Q w KQkq - 0 1", 5, 15194841, true ), 
           PerftInfo::new( "8/8/8/4B2b/6nN/8/5P2/2R1K2k w Q - 0 1", 5, 3223406, true ), 
           PerftInfo::new( "2r5/8/8/8/8/8/6PP/k2KR3 w K - 0 1", 5, 985298, true ), 
           PerftInfo::new( "4r3/3k4/8/8/8/8/6PP/qR1K1R2 w KQ - 0 1", 5, 8992652, true )

        );

        // John Merlino's test positions, some of these take a long time, only do them
        // in debug mode.
        let mut perft_debug_data = vec!(
           PerftInfo::new( "r3k2r/8/8/8/3pPp2/8/8/R3K1RR b KQkq e3 0 1", 6, 485647607, false ),
           PerftInfo::new( "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 6, 706045033, false ),
           PerftInfo::new( "8/7p/p5pb/4k3/P1pPn3/8/P5PP/1rB2RK1 b - d3 0 28", 6, 38633283, false ),
           PerftInfo::new( "8/3K4/2p5/p2b2r1/5k2/8/8/1q6 b - - 1 67", 7, 493407574, false ),
           PerftInfo::new( "rnbqkb1r/ppppp1pp/7n/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3", 6, 244063299, false ),
           PerftInfo::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 5, 193690690, false ),
           PerftInfo::new( "8/p7/8/1P6/K1k3p1/6P1/7P/8 w - -", 8, 8103790, false ),
           PerftInfo::new( "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - -", 6, 71179139, false ),
           PerftInfo::new( "r3k2r/p6p/8/B7/1pp1p3/3b4/P6P/R3K2R w KQkq -", 6, 77054993, false ),

           PerftInfo::new( "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 7, 178633661, false ),
           PerftInfo::new( "8/5p2/8/2k3P1/p3K3/8/1P6/8 b - -", 8, 64451405, false ),
           PerftInfo::new( "r3k2r/pb3p2/5npp/n2p4/1p1PPB2/6P1/P2N1PBP/R3K2R w KQkq -", 5, 29179893, false ),
        );

        let mut pos = Position::default_uci();

        if debug {
            perft_data.append(&mut perft_debug_data);
        }

        for mut info in perft_data {
            
            pos = Position::set(&info.fen, info.chess960);
            println!("{}", pos.board);
            let eval1 = pos.evaluate();
            // info.change_color();
            
            let board = pos.board.null_move().unwrap();
            pos = Position::set(&format!("{}", board), info.chess960);
            println!("{}", pos.board);
            let eval2 = pos.evaluate();
            assert_eq!(eval1, -eval2);


        }





        // let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        // let mut pos = Position::new_uci(&fen);

        // let mut move_list: Vec<Move> = vec![];
        // pos.board.generate_moves(|piece_moves| {
        //     for mv in piece_moves {
        //         move_list.push(mv);
                
        //     }
        //     false
        // });

        // for mv in move_list {
        //     println!("{}\n", mv);
        //     pos = Position::new_uci(&fen);
            
        //     pos.do_move(mv);
        //     println!("{}", pos.board.side_to_move());
        //     let eval1 = pos.evaluate();
        //     pos = Position::new_uci(&fen.replace("w", "b"));
        //     println!("{}", pos.board.side_to_move());
        //     let eval2 = pos.evaluate();
        //     assert_eq!(eval1.0, -eval2.0);

        // }
    }


}
