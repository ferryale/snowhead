use crate::position::Position;
use crate::types::r#move::{Move, EN_PASSANT, CASTLING};
use crate::types::piece::*;
use crate::types::score::{Value, MAX_MOVES};
use super::{ExtMove, generate_legal};



// #[test]
// fn do_undo_move_gives_same_position() {
//     let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//     let check_fen = "r1bqkbnr/p3p1pp/1Pn2P2/3p4/8/8/PPP2PPP/RNBQKBNR w KQkq - 2 6";
//     let ep_fen = "r1bqkbnr/p3p1pp/1Pn2P2/8/2Pp4/8/PP3PPP/RNBQKBNR b KQkq c3 0 7";
//     let oo_fen = "r2qkb1r/p2b2pp/1Pn2p1n/8/2B2P2/2p2N2/PP4PP/RNBQK2R w KQkq - 2 11";

//     let start_move = Move::make(Square::E2, Square::E4);
//     let check_move = Move::make(Square::D1, Square::H5);
//     let ep_move = Move::make(Square::D4, Square::C3);
//     let oo_move = Move::make(Square::E1, Square::G1);

//     let fens = [start_fen, check_fen, ep_fen, oo_fen];
//     let moves = [start_move, check_move, ep_move, oo_move];
//     let mut pos = Position::new();
//     let mut prev_pos = Position::new();
//     //println!("{:?}", pos.zobrist);

//     for idx in 0..4 {
//         let m = moves[idx];
//         let fen = fens[idx];
//         pos.set(fen, false);
//         let prev_pos = pos.clone();
//         pos.do_move(m);
//         pos.undo_move(m);
//         assert_eq!(prev_pos, pos);
//         //assert_eq!(prev_pos.by_type_bb, pos.by_type_bb);
//         // assert_eq!(prev_pos.board, pos.board);
//         // assert_eq!(prev_pos.by_type_bb, pos.by_type_bb);

//     }


// }

// #[test]
// fn illegal_castling() {
//     let oo_fen = "r2qk2r/p2bb1pp/1Pn2p1n/8/2B2P2/2N2N2/PP4PP/R1BQ1RK1 b kq - 0 12";
//     let m = Move::make(Square::E8, Square::G8);
//     let mut pos = Position::new();
//     pos.set(oo_fen, false);
//     assert!(!pos.legal(m))
// }


// fn perft(pos: &mut Position, depth: u32, root: bool) -> usize {

//     let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];

//     let leaf = depth == 2;
//     let mut cnt = 0;
//     let mut nodes = 0;

//     let mut num_moves = generate_legal(&pos, &mut list, 0);
//     //let prev_pos = pos.clone();

//     for ext_move in list{

//         if ext_move.m == Move::NONE { break; }


//         if root && depth <= 1 {
//             cnt = 1;
//             nodes += 1;
//         } else {
//             pos.do_move(ext_move.m);
//             if leaf { cnt = generate_legal(&pos, &mut list, 0); }
//             else { cnt = perft(pos, depth - 1, false) };
//             nodes += cnt;

//             pos.undo_move(ext_move.m);
//             //assert_eq!(&prev_pos, pos);


//         }
        
//         if root {
//             println!("{}: {}", ext_move.m.to_string(pos.is_chess960()), cnt);
//         }
        
//     }
//     nodes

// }

    
#[derive(Debug)]
struct Stats {
    pub nodes: u32,
    pub en_passant: u32,
    pub castles: u32,
    pub checks: u32,
    pub captures: u32,
}


impl Stats {
    pub fn new() -> Stats {
        Stats {
            nodes: 0,
            en_passant: 0,
            castles: 0,
            checks: 0,
            captures: 0
        }
    }
}   

fn is_capture(pos: &Position, m: Move) -> bool {

    let to = m.to();
    let captured = if m.move_type() == EN_PASSANT {
        Piece::make(!pos.side_to_move(), PAWN)
    } else {
        pos.piece_on(to)
    };

    captured != NO_PIECE

} 

fn perft_stats(pos: &mut Position, depth: u32, stats: &mut Stats) {

    let mut list = [ExtMove {m: Move::NONE, value: Value::ZERO}; MAX_MOVES];

    if depth == 0 {
        return ();
    }

    let _num_moves = generate_legal(&pos, &mut list, 0);
   
    for ext_move in list{
    
        if ext_move.m == Move::NONE { break; }

        if depth == 1 {
            stats.nodes += 1;
            if pos.gives_check(ext_move.m) { stats.checks += 1; }
            if is_capture(pos, ext_move.m) { stats.captures += 1};
            if ext_move.m.move_type() == CASTLING {  stats.castles += 1; } 
            if ext_move.m.move_type() == EN_PASSANT  { stats.en_passant += 1; }
        }

    
        pos.do_move(ext_move.m);
        perft_stats(pos, depth - 1, stats);
        pos.undo_move(ext_move.m);

        
        

    }

}

#[test]
fn perft4_stats() {

    let mut stats = Stats::new();
    let depth = 4;

    let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut pos = Position::new();
    pos.set(start_fen, false);
    perft_stats(&mut pos, depth, &mut stats);

    assert_eq!(stats.nodes, 197_281);
    assert_eq!(stats.captures, 1_576);
    assert_eq!(stats.checks, 469);
        

}

// #[test]
// fn perft5_stats() {
//     //5   4,865,609   82,719  258 0   0   27,351  6   0   347

//     let mut stats = Stats::new();
//     let depth = 5;

//     let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//     let mut pos = Position::new();
//     pos.set(start_fen, false);
//     perft_stats(&mut pos, depth, &mut stats);

//     assert_eq!(stats.nodes, 4_865_609);
//     assert_eq!(stats.captures, 82_719);
//     assert_eq!(stats.checks, 27_351);
//     assert_eq!(stats.en_passant, 258);
// }

// #[test]
// fn perft6_stats() {
//     //119,060,324 2,812,008   5248    0   0   809,099

//     let mut stats = Stats::new();
//     let depth = 6;

//     let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//     let mut pos = Position::new();
//     pos.set(start_fen, false);
//     perft_stats(&mut pos, depth, &mut stats);

//     assert_eq!(stats.nodes, 119_060_324);
//     assert_eq!(stats.captures, 2_812_008);
//     assert_eq!(stats.checks, 809_099);
//     assert_eq!(stats.en_passant, 5248);
// }

// #[test]
// fn perft7_stats() {
//     //Depth   Nodes   Captures    E.p.    Castles Promotions  Checks  Discovery Checks    Double Checks   Checkmates
//     //3,195,901,860 108,329,926 319,617 883,453 0   33,103,848  18,026  1628    435,767

//     let mut stats = Stats::new();
//     let depth = 7;

//     let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//     let mut pos = Position::new();
//     pos.set(start_fen, false);
//     perft_stats(&mut pos, depth, &mut stats);

//     assert_eq!(stats.nodes, 3_195_901_860);
//     assert_eq!(stats.captures, 108_329_926);
//     assert_eq!(stats.checks, 33_103_848);
//     assert_eq!(stats.en_passant, 319_617);
//     assert_eq!(stats.castles, 883_453);
// }


    

