use crate::types::piece::{COLOR_NB};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub struct UCILimits {
    pub time: [i64; COLOR_NB],
    pub inc: [i64; COLOR_NB],
    pub movestogo: i32,
    pub depth: u32,
    pub movetime: i64,
    pub mate: u32,
    pub perft: u32,
    pub infinite: bool,
    pub nodes: u64,
    pub start_time: SystemTime,
}

impl UCILimits {
    pub fn new() -> UCILimits {
        UCILimits {
            time: [0; COLOR_NB],
            inc: [0; COLOR_NB],
            movestogo: 0,
            depth: 0,
            movetime: 0,
            mate: 0,
            perft: 0,
            infinite: false,
            nodes: 0,
            start_time: SystemTime::now()
        }
    }

    pub fn use_time_management(&self) -> bool {
        self.mate == 0 && self.movetime == 0 && self.depth == 0
        && self.nodes == 0 && self.perft == 0 && !self.infinite
    }
}