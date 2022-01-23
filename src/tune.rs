
use crate::types::r#move::Move;

use crate::position::Position;

use crate::types::square::*;
use crate::types::piece::*;
use crate::types::score::*;
use crate::psqt::min_file;
use crate::psqt;
use crate::uci::START_FEN;


type Bonus = [[[Score; 4]; FILE_NB]; 6];
type PieceValue = [[Value; 6]; 2];
type Psqt = [[Score; SQUARE_NB]; PIECE_NB];

pub const fn init_psq(bonus: &Bonus, piece_value: &PieceValue) -> Psqt {
    
    let mut psq_array = [[Score::ZERO; SQUARE_NB]; PIECE_NB];
    let mut pc_idx = 1;

    while pc_idx < PIECE_TYPE_NB - 1 {

        let pc = Piece(pc_idx as u32);
        let bpc = Piece(pc.0 ^ 8);
        let score = Score::make(piece_value[MG][pc.0 as usize-1], piece_value[EG][pc.0 as usize-1]);

        let mut s_idx = 0;
        while s_idx < SQUARE_NB {
            let s = Square(s_idx as u32);
            let bs = Square(s.0 ^ Square::A8.0);

            let f = min_file(s.file(), File(FILE_H.0 - s.file().0));

            psq_array[pc.0 as usize][s.0 as usize] = Score(score.0
                + bonus[(pc.0 - 1) as usize][s.rank().0 as usize][f.0 as usize].0);
            psq_array[bpc.0 as usize][bs.0 as usize] = 
                Score(-psq_array[pc.0 as usize][s.0 as usize].0);

            s_idx += 1;
        }

        pc_idx += 1;
    }

    psq_array
}

pub fn eval(args: &str) {
    let fen: &str;

    let params = match args.find("params") {
        Some(idx) => idx,
        None => args.len(),
    };


    if &args[0..8] == "startpos" {
        fen = START_FEN;
    } else if &args[0..3] == "fen" {
        fen = (&args[3..params]).trim();
    } else {
        return;
    }

    println!("{:?}", params);

    if params == args.len() {
        return;
    }

    let mut piece_value = [[Value(0); 6]; 2];
    let mut bonus = [[[Score(0); 4]; FILE_NB]; 6];

    // Parse move list
    let params = &args[params+6..].trim();
    let mut iter = params.split_whitespace();
    
    let mut idx = 0;
    while let Some(val) = iter.next() {
        if idx < 12 {
            let pc_idx = idx % 6;
            let phase = idx / 6;
            piece_value[phase][pc_idx] = Value(val.parse().unwrap());
        } else {
            let mg_value = Value(val.parse().unwrap());
            let eg_value = Value(iter.next().unwrap().parse().unwrap());
            let sq_idx = (idx - 12) % 32;
            let file_idx = sq_idx % 4;
            let rank_idx = sq_idx / 4;
            let pc_idx = (idx - 12) / 32;
            bonus[pc_idx][rank_idx][file_idx] = Score::make(mg_value, eg_value);
        }  
        idx += 1;
    }

    let psq_table = init_psq(&bonus, &piece_value);
    assert_eq!(psq_table, psqt::PSQ);

    // eval startpos params 100 300 300 500 900 0 100 300 300 500 900 0
    // println!("{:?}", piece_value);

    // if arg.len() == 

    // // let params = 

    
    // let mut array: [i32; 10] = [0i32; 10];
    // while let Some(param) = iter.next() {
    //     match param {
    //         "position" => { break; },
    //         _ => {
    //             let mut vals = param.split_whitespace();

    //             for (idx, val) in vals.enumerate() {
    //                 array[idx] = val.parse().unwrap();
    //             }
    //             println!("{:?}", array);
    //         }
    //         // "winc" => limits.inc[WHITE] =
    //         //     iter.next().unwrap().parse().unwrap(),
    //         // "binc" => limits.inc[BLACK] =
    //         //     iter.next().unwrap().parse().unwrap(),
    //         // "movestogo" => limits.movestogo =
    //         //     iter.next().unwrap().parse().unwrap(),
    //         // "depth" => limits.depth = iter.next().unwrap().parse().unwrap(),
    //         // "nodes" => limits.nodes = iter.next().unwrap().parse().unwrap(),
    //         // "movetime" => limits.movetime =
    //         //     iter.next().unwrap().parse().unwrap(),
    //         // "mate" => limits.mate = iter.next().unwrap().parse().unwrap(),
    //         // "perft" => limits.perft = iter.next().unwrap().parse().unwrap(),
    //         // "infinite" => limits.infinite = true,
    //         // "ponder" => {},
    //         _ => {}
    //     }
    // }

    // let mut iter = params.split_whitespace();

    // while let Some(param) = iter.next() {
    //     println!("{}", );
    // }

    // let moves = match args.find("moves") {
    //     Some(idx) => idx,
    //     None => args.len(),
    // };

    // if &args[0..8] == "startpos" {
    //     fen = START_FEN;
    // } else if &args[0..3] == "fen" {
    //     fen = (&args[3..moves]).trim();
    // } else {
    //     return;
    // }

    // pos.init_states();
    // pos.set(fen, pos.is_chess960());
    // // pos_data.fen = String::from(fen);
    // // pos_data.moves = Vec::new();

    // if moves == args.len() {
    //     return;
    // }

    // // Parse move list
    // let moves = &args[moves+5..].trim();
    // let iter = moves.split_whitespace();
    // for token in iter {
    //     let m = Move::from_string(pos, token);
    //     if m == Move::NONE {
    //         break;
    //     }
    //     pos.do_move(m);
    // }
}
