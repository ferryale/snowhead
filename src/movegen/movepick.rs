use crate::evaluate::psqt::PIECE_VALUES;
use crate::evaluate::score::Value;
use crate::position::Position;
use cozy_chess::Move;
use super::movevalue::{MoveValue,MoveValues};



pub const MAX_MOVES: usize = 256;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    Init,
    PvMove,
    KillerOne,
    KillerTwo,
    CounterMove,
    GenCaptures,
    GoodCaptures,
    GenQuiet,
    Quiet,
    BadCaptures,
}

impl Stage {
    pub fn new() -> Stage {
        Stage::Init

    }
    pub fn next(&mut self) {
        match self {
            Stage::Init => *self = Stage::PvMove,
            Stage::PvMove => *self = Stage::KillerOne,
            Stage::KillerOne => *self = Stage::KillerTwo,
            Stage::KillerTwo => *self = Stage::CounterMove,
            Stage::CounterMove => *self = Stage::GenCaptures,
            Stage::GenCaptures => *self = Stage::GoodCaptures,
            Stage::GoodCaptures => *self = Stage::GenQuiet,
            Stage::GenQuiet => *self = Stage::Quiet,
            Stage::Quiet => *self = Stage::BadCaptures,
            Stage::BadCaptures => {},
        };
    }

}

pub struct MovePicker {
    stage: Stage,
    move_values: MoveValues<MAX_MOVES>,
    //num_picked: usize,
    start_bad_captures: usize,
    start_quiet: usize,

}

impl MovePicker {

    pub fn new() -> MovePicker {
        MovePicker {
            stage: Stage::Init,
            move_values: MoveValues::<MAX_MOVES>::new(),
            //num_picked: 0,
            start_bad_captures: 0,
            start_quiet: 0,
        }
    }

    pub fn next_move(&mut self, pos: &Position) -> Option<Move> {
        loop { 
            match self.stage {
                Stage::Init => {
                    println!("{:?}", self.stage);
                    self.next_stage();
                },
                Stage::PvMove => {
                    println!("{:?}", self.stage);
                    self.next_stage();
                },
                Stage::KillerOne => {
                    println!("{:?}", self.stage);
                    self.next_stage();
                },
                Stage::KillerTwo => {
                    println!("{:?}", self.stage);
                    self.next_stage();
                },
                Stage::CounterMove => {
                    println!("{:?}", self.stage);
                    self.next_stage();
                },
                Stage::GenCaptures => {
                    println!("{:?}", self.stage);
                    self.move_values = generate_captures(&pos);
                    // println!("{:?}", self.num_moves());

                    self.start_quiet = self.move_values.size();
                    self.next_stage();
                },
                Stage::GoodCaptures => {
                    // println!("{:?}", self.stage);
                    // println!("{}", self.move_values);
                    match self.move_values.next() {
                        None => self.next_stage(),
                        Some(move_value) => {
                            //println!("{}", move_value);
                            if move_value.value() >= Value::ZERO {
                                //self.num_picked += 1;
                                return Some(move_value.chess_move());
                            } else {
                                // Decrement current since the bad capture will not be returned
                                self.decr_current(1);
                                self.start_bad_captures = self.current();
                                self.next_stage();
                            }
                        }
                    };
                },
                
                Stage::GenQuiet => {
                    self.move_values.extend(&generate_quiet(&pos));
                    self.set_current(self.start_quiet);
                    self.next_stage();
                },
                
                Stage::Quiet => {
                    match self.move_values.next() {
                        None => {
                            self.set_current(self.start_bad_captures);
                            self.next_stage();
                        }
                        Some(move_value) => {
                            return Some(move_value.chess_move());
                        }
                    };
                },
                Stage::BadCaptures => {
                    if self.current() >= self.start_quiet {
                        break;
                    }
                    match self.move_values.next() {
                        None => break,
                        Some(move_value) => {
                            return Some(move_value.chess_move());
                        }
                    };
                },
            } // match
        } // loop

        None
    }

    fn next_stage(&mut self) {
        self.stage.next();
    }

    fn current(&mut self) -> usize {
        self.move_values.idx()
    }

    fn set_current(&mut self, idx: usize) {
        self.move_values.set_idx(idx);
    }

    fn incr_current(&mut self, idx: usize) {
        self.move_values.incr_idx(idx);
    }

    fn decr_current(&mut self, idx: usize) {
        self.move_values.decr_idx(idx);
    }

    // pub fn num_moves(&self) -> usize {
    //     self.move_values.size()
    // }

    pub fn stage(&self) -> Stage {
        self.stage
    }

}

pub fn generate_captures(pos: &Position) -> MoveValues<MAX_MOVES> {
    let mut move_values = MoveValues::<MAX_MOVES>::new();
    let mut move_value = MoveValue::default();
    pos.board.generate_moves(|mut piece_moves| {
        piece_moves.to &= pos.board.colors(!pos.board.side_to_move());
        for mv in piece_moves {
            move_value = MoveValue::new(mv, score_capture(pos, &mv));
            move_values.push_sort(move_value);
        }
        false
    });
    move_values

}

pub fn generate_quiet(pos: &Position) -> MoveValues<MAX_MOVES> {
    let mut move_values = MoveValues::<MAX_MOVES>::new();
    let mut move_value = MoveValue::default();
    pos.board.generate_moves(|mut piece_moves| {
        piece_moves.to &= !pos.board.colors(!pos.board.side_to_move());
        for mv in piece_moves {
            move_value = MoveValue::new(mv, Value(0));
            move_values.push_sort(move_value);
        }
        false
    });
    move_values

}

fn score_capture(pos: &Position, mv: &Move) -> Value {
    let pc_from = pos.board.piece_on(mv.from).unwrap();
    let pc_to = pos.board.piece_on(mv.to).unwrap();
    Value(PIECE_VALUES[pc_to as usize][0] - PIECE_VALUES[pc_from as usize][0])
}

