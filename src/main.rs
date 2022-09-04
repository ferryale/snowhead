use snowhead::uci::Uci;
// use snowhead::movegen::movepick::MovePicker;
// use snowhead::movegen::movepick;
use snowhead::position::Position;
use snowhead::evaluate::{Evaluator};
use cozy_chess::{Move, Square, Rank};

fn main() {

    // let mut pos = Position::default_uci();
    // //
    // //let moves = ["e2e4", "e7e6", "d1h5", "d8e7", "e1d1"];

    // let mut move_list: Vec<Move> = vec![];
    // pos.board.generate_moves(|piece_moves| {
    //     for mv in piece_moves {
    //         move_list.push(mv);
    //     }
    //     false
    // });

    // for mv in move_list {
    //     pos.init_psq();
        
    //     pos.do_move(mv);
        
    //     println!("{}, {}", mv, pos.board);
    //     assert_eq!(pos.psq(), pos.eval_psq());
    //     pos.undo_move();
        
    //     // assert_eq!(format!("{}", pos.board), expected);
    //     // assert_eq!(pos.board.hash(), expected.parse::<Board>().unwrap().hash());
    // }

    // let evaluator = Evaluator::default();
    // println!("{:?}", evaluator.psq_tables[1].scores[0]);

    // let fen = "rnbqk1nr/pppp1pp1/8/2b1p2p/4P3/5N2/PPPPBPPP/RNBQK2R w KQkq - 2 4";
    // //let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    // let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let mut pos = Position::new_uci(fen);
    // let color = pos.board.side_to_move();
    // let ep_square = pos.board.en_passant().map(|ep| {
    //                         Square::new(ep, Rank::Sixth.relative_to(color))
    //                     });

    // println!("{:?} {:?}", pos.board.en_passant(), ep_square);

    // pos.do_move("e2e4".parse().unwrap());
    // println!("{:?} {:?}", pos.eval(), pos.evaluate());

    
    // let mut move_list: Vec<Move> = vec![];
    // pos.board.generate_moves(|piece_moves| {
    //     for mv in piece_moves {
    //         move_list.push(mv);
    //         println!("{}\n", mv);
    //     }
    //     false
    // });

    // println!("{:?}\n, len={}", move_list, move_list.len());

    // let mut move_values = movepick::generate_captures(&pos);
    // println!("{}, len={}", move_values, move_values.size());
    // let quiet = &movepick::generate_quiet(&pos);
    // println!("{}, len={}", quiet, quiet.size());

    // move_values.extend(&movepick::generate_quiet(&pos));
    // println!("{}, len={}", move_values, move_values.size());


    // //println!("{:?}", pos);
    // let mut idx = 0;
    // let mut m = MovePicker::new();
    // // println!("{}", m.num_moves());
    // // while m.next_move(&pos).is_some(){
    // //     let mv = m.next_move(&pos);
    // //     println!("{idx} {}", mv.unwrap());
    // //     println!("{}", m.num_moves());
    // //     idx+=1;

    // // }

    // while let Some(mv) = m.next_move(&pos){
    //     println!("{idx} {} {:?}", mv, m.stage());
    //     //println!("{}", m.num_moves());
    //     idx+=1;

    // }

    // // idx = 0;

    // // while let Some(mval) = move_values.next(){
    // //     println!("{idx} {}", mval);
    // //     idx+=1;

    // // }

    // //println!("{}", m.num_picked);
    
    // //let capts = movepick::generate_captures(&pos);
    // //println!("{:?}", capts);
    match Uci::cmd_loop() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    

}
