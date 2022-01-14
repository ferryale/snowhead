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
use crate::evaluate::*;
use crate::position::inline::*;
use crate::position::game::*;
use crate::types::*;
use crate::search;

use std::ops;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stage(pub u32);

impl Stage {
    pub const MAIN_TT:          Stage = Stage(0);
    pub const CAPTURES_INIT:    Stage = Stage(1);
    pub const GOOD_CAPTURES:    Stage = Stage(2);
    pub const KILLER_ONE:       Stage = Stage(3);
    pub const KILLER_TWO:       Stage = Stage(4);
    pub const QUIET_INIT:       Stage = Stage(5);
    pub const QUIET:            Stage = Stage(6);
    pub const BAD_CAPTURES:     Stage = Stage(7);
    pub const EVASION_TT:       Stage = Stage(8);
    pub const EVASION_INIT:     Stage = Stage(9);
    pub const ALL_EVASIONS:     Stage = Stage(10);

    pub const QSEARCH_TT:       Stage = Stage(11);
    pub const QCAPTURES_INIT:   Stage = Stage(12);
    pub const QCAPTURES:        Stage = Stage(13);
    pub const QCHECKS_INIT:     Stage = Stage(14);
    pub const QCHECKS :         Stage = Stage(15);
    
}

enable_base_operations_for_u32_on!(Stage);


/// partial_insertion_sort() sorts moves in descending order up to and
/// including a given limit.
fn partial_insertion_sort(list: &mut [ExtMove], limit: i32) {
    let mut sorted_end = 0;
    for p in 1..list.len() {
        if list[p].value >= limit {
            let tmp = list[p];
            sorted_end += 1;
            list[p] = list[sorted_end];
            let mut q = sorted_end;
            while q > 0 && list[q-1].value < tmp.value {
                list[q] = list[q - 1];
                q -= 1;
            }
            list[q] = tmp;
        }
    }
}

/// pick_best() finds the best move in the list and moves it to the front.
/// Calling pick_best() is faster than sorting all the moves in advance if
/// there are few moves, e.g. the possible captures.
fn pick_best(list: &mut [ExtMove]) -> ExtMove {
    let mut q = 0;
    for p in 1..list.len() {
        if list[p].value > list[q].value {
            q = p;
        }
    }

    list.swap(0, q);
    list[0]
}

// MovePicker structs are used to pick one pseudo-legal move at a time from
// the current position. The most important method is next_move(), which
// returns a new pseudo-legal move each time it is called, until there are
// no moves left, when MOVE_NONE is returned. In order to improve the
// efficiency of the alpha beta algorithm, MovePicker attempts to return the
// moves which are most likely to get a cut off first.

pub struct MovePicker {
    cur: usize,
    end_moves: usize,
    end_bad_captures: usize,
    stage: Stage,
    depth: i32,
    tt_move: Move,
    killers: [Move; 2],
    list: [ExtMove; MAX_MOVES as usize],
}


/// Implementations of the MovePicker classes. As arguments we pass information
/// to help it return the (presumably) good moves first, to decide which moves
/// to return (in the quiescence search, for instance, we only want to search
/// captures, promotions and some checks) and how important good move ordering
/// is at the current node.

impl MovePicker {
    pub fn new(pos: &Position, ttm: Move, depth: i32, ss: &[search::Stack]) -> MovePicker {
        let mut stage = if pos.checkers() != 0 { Stage::EVASION_TT } else {
            if depth > 0 {
                Stage::MAIN_TT
            } else {
                Stage::QSEARCH_TT
            }
        };
        
        if ttm == Move::NONE {
            stage += 1;
        }

        MovePicker {
            cur: 0,
            end_moves: 0,
            end_bad_captures: 0,
            stage: stage,
            tt_move: ttm,
            killers: [ss[0].killers[0], ss[0].killers[1]],
            depth: depth,
            list: [ExtMove {m: Move::NONE, value: 0}; MAX_MOVES as usize],
        }
    }

    pub fn next_move(&mut self, pos: &Position, skip_quiets: bool) -> Move {
        loop { match self.stage {
            Stage::MAIN_TT | Stage::EVASION_TT | Stage::QSEARCH_TT => {
                self.stage += 1;
                return self.tt_move;
            }

            Stage::CAPTURES_INIT | Stage::QCAPTURES_INIT => {
                self.end_moves = generate(CAPTURES, pos, &mut self.list, 0);
                self.score_captures(pos);
                self.stage += 1;
            }

            Stage::GOOD_CAPTURES => {
                while self.cur < self.end_moves {
                    let ext_move = self.pick_best();
                    if ext_move.m != self.tt_move {
                        // Good capture
                        if ext_move.value > 0 {
                            return ext_move.m;
                        }

                        // Losing capture. Move it to the beginning of the
                        // array.
                        self.list[self.end_bad_captures].m = ext_move.m;
                        self.end_bad_captures += 1;
                    }
                }

                self.stage += 1;
            }
          
            Stage::KILLER_ONE => {
                self.stage += 1;
                let m = self.killers[0];
                if m != Move::NONE
                    && m != self.tt_move
                    && !pos.capture(m)
                {
                    return m;
                }
            }

            Stage::KILLER_TWO => {
                self.stage += 1;
                let m = self.killers[1];
                if m != Move::NONE
                    && m != self.tt_move
                    && !pos.capture(m)
                {
                    return m;
                }
            }

            Stage::QUIET_INIT => {
                self.cur = self.end_bad_captures;
                self.end_moves = generate(QUIETS, pos, &mut self.list, self.cur);
                //score_quiets(pos, self);
                //partial_insertion_sort(&mut self.list[self.cur..self.end_moves], -4000);
                self.stage += 1;
            }

            Stage::QUIET => {
                if !skip_quiets {
                    while self.cur < self.end_moves {
                        let m = self.pick_next();
                        if m != self.tt_move
                            && m != self.killers[0]
                            && m != self.killers[1]
                        {
                            return m;
                        }
                    }
                }
                self.stage += 1;
                self.cur = 0; // Point to beginning of bad captures
            }

            Stage::BAD_CAPTURES => {
                if self.cur < self.end_bad_captures {
                    return self.pick_next();
                }
                break;
            }

            Stage::EVASION_INIT => {
                self.cur = 0;
                self.end_moves = generate(EVASIONS, pos, &mut self.list, 0);
                //score_evasions(pos, &mut self.list[..self.end_moves]);
                self.stage += 1;
            }

            Stage::ALL_EVASIONS => {
                while self.cur < self.end_moves {
                    let m = self.pick_next();
                    if m != self.tt_move {
                        return m;
                    }
                }
                break;
            }

            Stage::QCAPTURES => {
                while self.cur < self.end_moves {
                    return self.pick_best().m; 
                }
                self.stage += 1;   
            }

            Stage::QCHECKS_INIT => {
                self.end_moves = generate(QUIET_CHECKS, pos, &mut self.list, self.cur);
                self.stage += 1;
                
            }

            Stage::QCHECKS => {
                while self.cur < self.end_moves {
                    return self.pick_next(); 
                }
                break;  
            }

            _ => { panic!("movepick") }
        } }

        Move::NONE
    }

    fn pick_best(&mut self) -> ExtMove {
        let m = pick_best(&mut self.list[self.cur..self.end_moves]);
        self.cur += 1;
        m
    }

    fn pick_next(&mut self) -> Move {
        let m = self.list[self.cur].m;
        self.cur += 1;
        m
    }

    fn score_captures(&mut self, pos: &Position) {

        for ext_move in self.list[..self.end_moves].iter_mut() {
            let pc_from = pos.piece_on(ext_move.m.from());
            let pc_to = pos.piece_on(ext_move.m.to());
            ext_move.value = piece_value(MG, pc_to).0 - piece_value(MG, pc_from).0;
        }

    }
}