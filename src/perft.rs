use crate::types::r#move::Move;
use crate::types::score::{Depth, MAX_MOVES};
use crate::position::Position;
use crate::movegen::{ExtMove, generate_legal};


pub fn perft<const ROOT: bool>(pos: &mut Position, depth: Depth) -> usize {

    let mut list = [ExtMove {m: Move::NONE, value: 0}; MAX_MOVES];

    let leaf = depth == Depth(2);
    let mut cnt = 0;
    let mut nodes = 0;

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
        nodes: usize
    }

    impl PerftInfo {
        fn new(fen: &str, depth: i32, nodes: usize) -> PerftInfo {
            PerftInfo {
                fen: String::from(fen), 
                depth: Depth(depth),
                nodes: nodes
            }
        }
    }

    #[test]
    fn perft_test_positions() {

        let perft_data = vec!( 
            PerftInfo::new( "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",         6, 1134888 ),
            PerftInfo::new( "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1",         6, 1134888 ),
            PerftInfo::new( "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",          6, 1015133 ),
            PerftInfo::new( "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1",         6, 1015133 ),
        );

        let mut pos = Position::new();

        for info in perft_data {
            pos.set(&info.fen, false);
            let nodes = perft::<true>(&mut pos, info.depth);
            //println!("Testing fen {}", info.fen);
            assert_eq!(nodes, info.nodes)
        }

    }

}

//     // macro_rules! perft_info {

//     //     ($x: expr, $y: expr, $z: expr) => {

//     //         PerftInfo {
//     //             fen: String::from($x),
//     //             depth: Depth($y),
//     //             nodes: $z
//     //         }

//     //     };

//     // }



//     // let info = PerftInfo::new( "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",         6, 1134888 );

    






// // #[test]




// http://www.talkchess.com/forum3/viewtopic.php?t=59046
/* 
   // Martin Sedlak's test positions
   // (http://www.talkchess.com/forum/viewtopic.php?t=47318)
   // avoid illegal ep
   { "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",         6, 1134888 },
   { "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1",         6, 1134888 },
   { "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",         6, 1015133 },
   { "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1",         6, 1015133 },
   // en passant capture checks opponent: 
   { "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1",         6, 1440467 },
   { "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1",         6, 1440467 },
   // short castling gives check: 
   { "5k2/8/8/8/8/8/8/4K2R w K - 0 1",            6, 661072 },
   { "4k2r/8/8/8/8/8/8/5K2 b k - 0 1",            6, 661072 },
   // long castling gives check: 
   { "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",            6, 803711 },
   { "r3k3/8/8/8/8/8/8/3K4 b q - 0 1",            6, 803711 },
   // castling (including losing cr due to rook capture): 
   { "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",   4, 1274206 },
   { "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1",    4, 1274206 },
   // castling prevented: 
   { "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",   4, 1720476 },
   { "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1",   4, 1720476 },
   // promote out of check: 
   { "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1",         6, 3821001 },
   { "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1",         6, 3821001 },
   // discovered check: 
   { "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",         5, 1004658 },
   { "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1",         5, 1004658 },
   // promote to give check: 
   { "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",            6, 217342 },
   { "8/k7/8/8/8/8/1p6/4K3 b - - 0 1",            6, 217342 },
   // underpromote to check: 
   { "8/P1k5/K7/8/8/8/8/8 w - - 0 1",            6, 92683 },
   { "8/8/8/8/8/k7/p1K5/8 b - - 0 1",            6, 92683 },
   // self stalemate: 
   { "K1k5/8/P7/8/8/8/8/8 w - - 0 1",            6, 2217 },
   { "8/8/8/8/8/p7/8/k1K5 b - - 0 1",            6, 2217 },
   // stalemate/checkmate: 
   { "8/k1P5/8/1K6/8/8/8/8 w - - 0 1",            7, 567584 },
   { "8/8/8/8/1k6/8/K1p5/8 b - - 0 1",            7, 567584 },
   // double check: 
   { "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1",         4, 23527 },
   { "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1",         4, 23527 },

   // short castling impossible although the rook never moved away from its corner 
   { "1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1", 5, 1063513 },
   { "4k2r/8/8/7r/8/8/1B6/1K6 w k - 0 1", 5, 1063513 },

   // long castling impossible although the rook never moved away from its corner 
   { "1k6/8/8/8/R7/1n6/8/R3K3 b Q - 0 1", 5, 346695 },
   { "r3k3/8/1N6/r7/8/8/8/1K6 w q - 0 1", 5, 346695 },

   // From the Wiki
   { "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 4, 4085603 },
   { "rnbqkb1r/pp1p1ppp/2p5/4P3/2B5/8/PPP1NnPP/RNBQK2R w KQkq - 0 6", 3, 53392 },

   // Shortened form of the third position below
   { "8/7p/p5pb/4k3/P1pPn3/8/P5PP/1rB2RK1 b - d3 0 28", 4, 67197 },

   // Some FRC postions by Reinhard Scharnagl
   // (http://www.talkchess.com/forum/viewtopic.php?t=55274)
   // We have each of them twice, to get the number of moves at the root
   // correct too.
   { "r1k1r2q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K1R2Q w KQkq - 0 1", 1, 23 },
   { "r1k2r1q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K2R1Q w KQkq - 0 1", 1, 28 },
   { "8/8/8/4B2b/6nN/8/5P2/2R1K2k w Q - 0 1", 1, 34 },
   { "2r5/8/8/8/8/8/6PP/k2KR3 w K - 0 1", 1, 17 },
   { "4r3/3k4/8/8/8/8/6PP/qR1K1R2 w KQ - 0 1", 1, 19 },

   { "r1k1r2q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K1R2Q w KQkq - 0 1", 2, 522 },
   { "r1k2r1q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K2R1Q w KQkq - 0 1", 2, 738 },
   { "8/8/8/4B2b/6nN/8/5P2/2R1K2k w Q - 0 1", 2, 318 },
   { "2r5/8/8/8/8/8/6PP/k2KR3 w K - 0 1", 2, 242 },
   { "4r3/3k4/8/8/8/8/6PP/qR1K1R2 w KQ - 0 1", 2, 628 },

   { "r1k1r2q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K1R2Q w KQkq - 0 1", 5, 7096972 }, 
   { "r1k2r1q/p1ppp1pp/8/8/8/8/P1PPP1PP/R1K2R1Q w KQkq - 0 1", 5, 15194841 }, 
   { "8/8/8/4B2b/6nN/8/5P2/2R1K2k w Q - 0 1", 5, 3223406 }, 
   { "2r5/8/8/8/8/8/6PP/k2KR3 w K - 0 1", 5, 985298 }, 
   { "4r3/3k4/8/8/8/8/6PP/qR1K1R2 w KQ - 0 1", 5, 8992652 },

   // John Merlino's test positions, some of these take a long time, only do them
   // in debug mode.
#ifdef DEBUGMODE
   { "r3k2r/8/8/8/3pPp2/8/8/R3K1RR b KQkq e3 0 1", 6, 485647607 },
   { "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 6, 706045033 },
   { "8/7p/p5pb/4k3/P1pPn3/8/P5PP/1rB2RK1 b - d3 0 28", 6, 38633283 },
   { "8/3K4/2p5/p2b2r1/5k2/8/8/1q6 b - - 1 67", 7, 493407574 },
   { "rnbqkb1r/ppppp1pp/7n/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3", 6, 244063299 },
   { "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 5, 193690690 },
   { "8/p7/8/1P6/K1k3p1/6P1/7P/8 w - -", 8, 8103790 },
   { "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - -", 6, 71179139 },
   { "r3k2r/p6p/8/B7/1pp1p3/3b4/P6P/R3K2R w KQkq -", 6, 77054993 },

   { "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 7, 178633661 },
   { "8/5p2/8/2k3P1/p3K3/8/1P6/8 b - -", 8, 64451405 },
   { "r3k2r/pb3p2/5npp/n2p4/1p1PPB2/6P1/P2N1PBP/R3K2R w KQkq -", 5, 29179893 },
#endif
*/

