// use super::{attacks_bb};
use crate::position::*;
// use crate::attacks::{sliding_attacks};
use crate::types::square::Square;
use crate::types::r#move::*;
// use crate::types::piece::*;
// use crate::types::bitboard::{EMPTY_BB, pretty};
use super::*;



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


fn perft(pos: &mut Position, depth: u32, root: bool) -> usize {

    let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];

    let leaf = depth == 2;
    let mut cnt = 0;
    let mut nodes = 0;

    let mut num_moves = generate_legal(&pos, &mut list, 0);
    //let prev_pos = pos.clone();

    for ext_move in list{

        if ext_move.m == Move::NONE { break; }


        if root && depth <= 1 {
            cnt = 1;
            nodes += 1;
        } else {
            pos.do_move(ext_move.m);
            if leaf { cnt = generate_legal(&pos, &mut list, 0); }
            else { cnt = perft(pos, depth - 1, false) };
            nodes += cnt;

            pos.undo_move(ext_move.m);
            //assert_eq!(&prev_pos, pos);


        }
        
        if root {
            println!("{}: {}", ext_move.m.to_string(pos.is_chess960()), cnt);
        }
        
    }
    nodes

}

#[test]
fn perft5_startfen() {
    let depth = 6;
    let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut pos = Position::new();
    pos.set(start_fen, false);
    let nodes = perft(&mut pos, depth, true);

    assert_eq!(nodes, 119_060_324);
}
    

