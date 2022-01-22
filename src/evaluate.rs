use crate::types::piece::{WHITE, BLACK, PAWN, KNIGHT, BISHOP, ROOK, QUEEN};
use crate::types::bitboard::EMPTY_BB;
use crate::types::r#move::Move;
use crate::types::score::{Value, Phase, MAX_MOVES};
use crate::position::Position;
use crate::movegen::{ExtMove, generate_legal};

// pub const MAX_VALUE_MG: Value = max_value_mg();

// const fn max_value_mg() -> Value {
//     Value(16*Value::PAWN_MG.0 + 4*Value::KNIGHT_MG.0 + 4*Value::BISHOP_MG.0 + 4*Value::ROOK_MG.0 + 2*Value::QUEEN_MG.0)
// }

pub fn evaluate(pos: &Position) -> Value {

    let phase = (pos.count(WHITE, PAWN) + pos.count(BLACK, PAWN)) as u32 * Phase::PAWN +
                (pos.count(WHITE, KNIGHT) + pos.count(BLACK, KNIGHT)) as u32 * Phase::KNIGHT +
                (pos.count(WHITE, BISHOP) + pos.count(BLACK, BISHOP)) as u32 * Phase::BISHOP +
                (pos.count(WHITE, ROOK) + pos.count(BLACK, ROOK)) as u32 * Phase::ROOK +
                (pos.count(WHITE, QUEEN) + pos.count(BLACK, QUEEN)) as u32 * Phase::QUEEN;

    

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

            let mut list = [ExtMove {m: Move::NONE, value: Value::ZERO}; MAX_MOVES];
            let num_moves = generate_legal(&self, &mut list, 0);

            if self.checkers() == EMPTY_BB || num_moves != 0 {
                return true;

            }
        }

        self.st().repetition != 0 && self.st().repetition < ply 

    }
}