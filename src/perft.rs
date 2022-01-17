use crate::types::r#move::Move;
use crate::types::score::{Depth, Value, MAX_MOVES};
use crate::position::Position;
use crate::movegen::{ExtMove, generate_legal};


pub fn perft<const ROOT: bool>(pos: &mut Position, depth: Depth) -> usize {

    let mut list = [ExtMove {m: Move::NONE, value: Value(0)}; MAX_MOVES];

    let leaf = depth == Depth(2);
    let mut cnt;// = 0;
    let mut nodes = 0;
    if ROOT {
        println!("Starting perft{} on position {}", depth.0, pos.fen());
    }

    let mut _num_moves = generate_legal(&pos, &mut list, 0);

    for ext_move in list{
        let m = ext_move.m;
        if m == Move::NONE { break; }

        if ROOT && depth.0 <= 1 {
            cnt = 1;
            nodes += 1;
        } else {
   
            pos.do_move(ext_move.m);

            cnt = if leaf {
                generate_legal(&pos, &mut list, 0)
            }
            else { 
                perft::<false>(pos, depth - 1) 
            };

            nodes += cnt;

            pos.undo_move(ext_move.m);
        }
        if ROOT {
            println!("{}: {}", m.to_string(pos.is_chess960()), cnt);
        }
        
    }
    nodes
}

#[cfg(test)]
mod perft_test {

    use crate::types::score::Depth;
    use super::perft;
    use crate::position::Position;

    struct PerftInfo {
        fen: String,
        depth: Depth,
        nodes: usize,
        chess960: bool
    }

    impl PerftInfo {
        fn new(fen: &str, depth: i32, nodes: usize, chess960: bool) -> PerftInfo {
            PerftInfo {
                fen: String::from(fen), 
                depth: Depth(depth),
                nodes: nodes,
                chess960: chess960,
            }
        }
    }

    #[test]
    fn perft_test_positions() {

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

        let mut pos = Position::new();

        if debug {
            perft_data.append(&mut perft_debug_data);
        }

        for info in perft_data {
            pos.set(&info.fen, info.chess960);
            let nodes = perft::<true>(&mut pos, info.depth);
            assert_eq!(nodes, info.nodes)
        }

    }

}