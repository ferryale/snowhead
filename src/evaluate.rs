use crate::attacks::attack_bb::*;
use crate::types::square::*;
use crate::types::piece::*;
use crate::types::r#move::*;
use crate::types::bitboard::*;
use crate::types::score::*;
use crate::zobrist::*;
use crate::psqt;
use crate::movegen::*;
use crate::position::*;
use crate::position::inline::*;

pub fn evaluate(pos: &Position) -> Value {
    if pos.side_to_move() == WHITE { 
        return pos.psq_score().mg(); 
    } 
        else { 
        return -pos.psq_score().mg();
    }
}

impl Position {
    // is_draw() tests whether the position is drawn by 50-move rule or by
    // repetition. It does not detect stalemates.

    pub fn is_draw(&self, ply: i32) -> bool {
        if self.rule50_count() > 99 {

            let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];
            let num_moves = generate_legal(&self, &mut list, 0);

            if self.checkers() == EMPTY_BB || num_moves != 0 {
                return true;

            }
        }

        false

        // TODO: implement repetition
 
    }
}