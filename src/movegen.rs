use self::movevalue::{MoveValue, MoveValues};
use crate::evaluate::score::Value;
use crate::position::Position;
use cozy_chess::{Move, Piece};

pub mod movepick;
pub mod movevalue;

pub const MAX_MOVES: usize = 256;
pub const LVA_MVV: [i16; Piece::NUM] = [100, 320, 330, 500, 900, 20000];

// Generates and sorts captures based on score: returns MoveValues
pub fn generate_captures(pos: &Position) -> MoveValues<MAX_MOVES> {
    let mut move_values = MoveValues::<MAX_MOVES>::new();
    let mut move_value = MoveValue::default();

    pos.board().generate_moves(|mut piece_moves| {
        piece_moves.to &= pos.board().colors(!pos.side_to_move());
        for mv in piece_moves {
            move_value = MoveValue::new(mv, score_capture(pos, &mv));
            move_values.insert_sort(move_value);
        }
        false
    });

    move_values
}

// Generates and sorts quiet based on score: returns MoveValues
pub fn generate_quiet(pos: &Position) -> MoveValues<MAX_MOVES> {
    let mut move_values = MoveValues::<MAX_MOVES>::new();
    let mut move_value = MoveValue::default();

    // TODO: implement scoring for quiet moves
    pos.board().generate_moves(|mut piece_moves| {
        piece_moves.to &= !pos.board().colors(!pos.side_to_move());
        for mv in piece_moves {
            move_value = MoveValue::new(mv, Value::ZERO);
            // TODO: replace push by insert_sort when scoring is implemented
            move_values.push(move_value);
        }
        false
    });

    move_values
}

// Scores a capture based on LVA_MVV values
fn score_capture(pos: &Position, mv: &Move) -> Value {
    let pc_from = pos.piece_on(mv.from).unwrap();
    let pc_to = pos.piece_on(mv.to).unwrap();
    Value(LVA_MVV[pc_to as usize] - LVA_MVV[pc_from as usize])
}
