pub mod movepick;
pub mod movevalue;

// #[cfg(test)]
// mod tests{
//     use super::{MoveValue, MoveValues};
//     #[test]
//     fn insertion_sort_works() {
//         let mut move_values = MoveValues::<5>::new();
//         let mut moves_list = [MoveValue::new_val(5), MoveValue::new_val(3), MoveValue::new_val(-3), MoveValue::new_val(0), MoveValue::new_val(18)];
//         for move_value in moves_list {
//             move_values.push_sort(move_value);
//             //println!("{:?}", move_values)
//         }
//         moves_list.sort();
//         assert_eq!(move_values.list, moves_list)
//     }

// }

// fn pick_best(&mut self) -> MoveValue {
//         let mut best_idx = 0;
//         for idx in 1..self.size {
//             if self.move_values[idx].value > self.move_values[best_idx].value {
//                 best_idx = idx;
//             }
//         }

//         self.move_values.swap(0, best_idx);
//         self.move_values[0]

//     }

// pub struct MovePicker {
//     stage: Stage

// }

// #[derive(Debug)]
// pub enum Stage {
//     Init,
//     PvMove,
//     Killer1,
//     Killer2,
//     CounterMove,
//     GenCaptures,
//     SortCaptures,
//     GoodCaptures,
//     Quiet,
//     BadCaptures,
// }

// impl Stage {
//     pub fn new() -> Stage {
//         Stage::Init

//     }
//     pub fn next(&mut self) {
//         match self {
//             Stage::Init => *self = Stage::PvMove,
//             Stage::PvMove => *self = Stage::Killer1,
//             Stage::Killer1 => *self = Stage::Killer2,
//             Stage::Killer2 => *self = Stage::CounterMove,
//             Stage::CounterMove => *self = Stage::GenCaptures,
//             Stage::GenCaptures => *self = Stage::SortCaptures,
//             Stage::SortCaptures => *self = Stage::GoodCaptures,
//             Stage::GoodCaptures => *self = Stage::Quiet,
//             Stage::Quiet => *self = Stage::BadCaptures,
//             Stage::BadCaptures => {},
//         };
//     }

// }

// impl MovePicker {

//     pub fn new() -> MovePicker {
//         MovePicker {
//             stage: Stage::Init
//         }
//     }

//     pub fn next_move(&mut self) -> Option<Move> {
//         match self.stage {
//             Stage::Init => None,
//             Stage::PvMove => None,
//             Stage::Killer1 => None,
//             Stage::Killer2 => None,
//             Stage::CounterMove => None,
//             Stage::GenCaptures => None,
//             Stage::SortCaptures => None,
//             Stage::GoodCaptures => None,
//             Stage::Quiet => None,
//             Stage::BadCaptures => None,

//         }
//     }
// }

// impl MoveValue {
//     pub fn new() -> MoveValue {
//         MoveValue {mv: Move {
//                 from: Square::A1,
//                 to: Square::A1,
//                 promotion: None,
//             };, value: Value::ZERO}
//     }
// }

// impl<const N: usize> MoveValues<N> {
//     pub fn new(size: usize) -> MoveValues {
//         MoveValues {
//             move_values: []
//         }
//     }
//     fn pick_best(&mut self) -> MoveValue {
//         let mut best_idx = 0;
//         for idx in 1..self.size {
//             if self.move_values[idx].value > self.move_values[best_idx].value {
//                 best_idx = idx;
//             }
//         }

//         self.move_values.swap(0, best_idx);
//         self.move_values[0]

//     }

// }

// // Generate captures
// fn gen_captures(pos: &Position) {
//     let mut move_list: Vec<Move> = vec![];
//     pos.board.generate_moves(|mut piece_moves| {
//         piece_moves.to &= pos.board.colors(!pos.board.side_to_move());
//         for mv in piece_moves {
//             move_list.push(mv);
//         }
//         false
//     });

// }

// // Generate captures
// fn score_captures(pos: &Position, move_list: &Vec<Moves>) -> MoveValues {
//     let mut move_values: MoveValues<100>;
//     for mv in move_list {
//         let pc_from = pos.board.piece_on(mv.from);
//         let pc_to = pos.board.piece_on(mv.from);
//         PIECE_VALUES[pc_to as usize][0] - PIECE_VALUES[pc_to as usize][0]

//     }

// }
