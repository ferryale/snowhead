use snowhead::uci::Uci;
// use snowhead::movegen::movepick::MovePicker;
// use snowhead::movegen::movepick;
// use snowhead::position::Position;
// use cozy_chess::Move;

fn main() {

    // let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    // let pos = Position::new_uci(fen);

    
    // let mut move_list: Vec<Move> = vec![];
    // pos.board.generate_moves(|piece_moves| {
    //     for mv in piece_moves {
    //         move_list.push(mv);
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
