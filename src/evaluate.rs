use crate::types::piece::WHITE;
use crate::types::bitboard::EMPTY_BB;
use crate::types::r#move::Move;
use crate::types::score::Value;
use crate::position::Position;
use crate::movegen::{ExtMove, generate_legal};


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

        self.st().repetition != 0 && self.st().repetition < ply 

    }
}