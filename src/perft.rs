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