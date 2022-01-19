use crate::uciset::UCILimits;
use crate::types::piece::Color;
use std::time::SystemTime;

pub const MAX_MOVES_TO_GO: i32 = 50;

#[derive(Debug, Clone, Copy)]
pub struct TimeManager {
    start_time: SystemTime,
    opt_time: i64,
    max_time: i64,
}

impl TimeManager {
    pub fn new(limits: &UCILimits, us: Color, ply: i32) -> TimeManager { 
        let mut time = TimeManager {
            start_time: limits.start_time,
            opt_time: 0,
            max_time: 0,
        };
        time.init(limits, us, ply);
        time
    }

    fn init(&mut self, limits: &UCILimits, us: Color, ply: i32) {

        let move_num = ply/2;
        let mtg = if limits.movestogo > 0 { 
            limits.movestogo } else { 
                MAX_MOVES_TO_GO 
            };

        let time_left = std::cmp::max(1, limits.time[us] + limits.inc[us] * (mtg as i64 - 1) - 100 * (mtg as i64));

        let max_start = 10;
        let max_end = 30;

        let moves_max = max_start - max_end;

        let scale_diff = 2.0 / (moves_max + mtg) as f32;
        let scale_min = 0.01;
        let scale_max = scale_min + scale_diff;

        let scale = if move_num < max_start {
            scale_min + move_num as f32 * (scale_diff/max_start as f32)
        } else if move_num > max_end {
            scale_max - move_num as f32 * (scale_diff / (mtg - max_end) as f32)  
        } else {
            scale_max
        };

        self.opt_time = (scale * time_left as f32) as i64;
        self.max_time = std::cmp::max((0.75*time_left as f32) as i64, (1.25*self.opt_time as f32) as i64);

    }

    // Returns time in ms
    pub fn elapsed(&self) -> i64 {
        let duration = self.start_time.elapsed().unwrap();
        (duration.as_secs() * 1000 + (duration.subsec_nanos() / 1000000) as u64)
            as i64
    }

    // Returns time in ms
    pub fn optimum(&self) -> i64 {
        std::cmp::max(self.opt_time, 10)

    }

    // Returns time in ms
    pub fn maximum(&self) -> i64 {
        self.max_time

    }

}