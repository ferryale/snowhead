use snowhead::types::square::*;
use snowhead::types::r#move::*;
use snowhead::position::*;


fn main() {
    //println!("{:?}", Square::A1);

    let mut pos = Position::new();
    

    //pos.set("rnbqkbnr/1pp1pppp/p2p4/8/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq - 1 3", false);

    //pos.set("rnbqkbnr/1pp1pppp/p2p4/8/Q7/2P2P2/PP1PP1PP/RNB1KBNR b KQkq - 1 3", false);

    pos.set("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", false);

    // let mut m = Move::make(Square::D2, Square::D4);
    // pos.do_move(m);
    // pos.print();

    // let mut m = Move::make(Square::D7, Square::D6);
    // pos.do_move(m);
    // pos.print();

    // let mut m = Move::make(Square::E2, Square::E4);
    // pos.do_move(m);
    // pos.print();

    //pos.set("rnbqkbnr/ppp1p1pp/5P2/3p4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 4", false);

    // let m = Move::make(Square::E2, Square::E4);
    // pos.do_move(m);
    // pos.print();

    // let pos1 = pos.clone();

    

    // assert_eq!(pos, pos1);
}
