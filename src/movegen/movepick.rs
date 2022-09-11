use super::movevalue::MoveValues;
use super::MAX_MOVES;
use crate::evaluate::score::Value;
use crate::position::Position;
use cozy_chess::Move;

/* Enum for staged move generation */
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

/* Stage implementation */
impl Stage {
    // Constructor
    pub fn new() -> Stage {
        Stage::Init
    }

    // Next stage, depending on some boolean flags
    pub fn next(&mut self, is_check: bool, skip_quiets: bool) {
        match self {
            Stage::Init => *self = Stage::PvMove,
            Stage::PvMove => *self = Stage::KillerOne,
            Stage::KillerOne => *self = Stage::KillerTwo,
            Stage::KillerTwo => *self = Stage::CounterMove,
            Stage::CounterMove => *self = Stage::GenCaptures,
            Stage::GenCaptures => *self = Stage::GoodCaptures,
            Stage::GoodCaptures => {
                if skip_quiets && !is_check {
                    *self = Stage::BadCaptures;
                } else {
                    *self = Stage::GenQuiet;
                }
            }
            Stage::GenQuiet => *self = Stage::Quiet,
            Stage::Quiet => *self = Stage::BadCaptures,
            Stage::BadCaptures => {}
        };
    }
}

/* MovePicker struct */
pub struct MovePicker {
    stage: Stage,
    move_values: MoveValues<MAX_MOVES>,
    start_bad_captures: usize,
    start_quiet: usize,
}

/* MovePicker implemetation */
impl MovePicker {
    // Constructor
    pub fn new() -> MovePicker {
        MovePicker {
            stage: Stage::Init,
            move_values: MoveValues::<MAX_MOVES>::new(),
            start_bad_captures: 0,
            start_quiet: 0,
        }
    }

    // Returns the move generation current stage
    pub fn stage(&self) -> Stage {
        self.stage
    }

    // Implements a state machine returning next move
    pub fn next_move(&mut self, pos: &Position, skip_quiets: bool) -> Option<Move> {
        loop {
            match self.stage {
                Stage::Init => {
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::PvMove => {
                    // TODO: add Transposition Tables
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::KillerOne => {
                    // TODO: add killer moves
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::KillerTwo => {
                    // TODO: add killer moves
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::CounterMove => {
                    // TODO: add countermoves
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::GenCaptures => {
                    self.move_values = super::generate_captures(&pos);
                    self.start_quiet = self.move_values.len();
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::GoodCaptures => {
                    match self.move_values.next() {
                        None => self.next_stage(pos.is_check(), skip_quiets),
                        Some(move_value) => {
                            if move_value.value() >= Value::ZERO {
                                return Some(move_value.chess_move());
                            } else {
                                // Decrement current since the bad capture will not be returned
                                self.decr_current(1);
                                self.start_bad_captures = self.current();
                                self.next_stage(pos.is_check(), skip_quiets);
                            } // if-else
                        } // Some
                    }; // match
                }

                Stage::GenQuiet => {
                    self.move_values.extend(&super::generate_quiet(&pos));
                    self.set_current(self.start_quiet);
                    self.next_stage(pos.is_check(), skip_quiets);
                }

                Stage::Quiet => {
                    match self.move_values.next() {
                        None => {
                            self.set_current(self.start_bad_captures);
                            self.next_stage(pos.is_check(), skip_quiets);
                        }
                        Some(move_value) => {
                            return Some(move_value.chess_move());
                        }
                    };
                }

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
                }
            } // match
        } // loop

        None
    }

    /* Helper methods */

    // Returns the next stage based on boolean flags
    fn next_stage(&mut self, is_check: bool, skip_quiets: bool) {
        self.stage.next(is_check, skip_quiets);
    }

    // Returns the current index of move_values
    fn current(&mut self) -> usize {
        self.move_values.idx()
    }

    // Sets the current index of move_values
    fn set_current(&mut self, idx: usize) {
        self.move_values.set_idx(idx);
    }

    // Decrements index of move_values
    fn decr_current(&mut self, idx: usize) {
        self.move_values.decr_idx(idx);
    }
}
