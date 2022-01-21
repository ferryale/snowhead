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
    pub fn new() -> TimeManager { 
        TimeManager {
            start_time: SystemTime::now(),
            opt_time: 0,
            max_time: 0,
        }
    }

    fn factor(x: f64, mtg: f64, b: f64, k: f64, x0: f64) -> f64 {
        let num = (1.0 - mtg*b) * k;
        let den = 1.0 + f64::exp(-k*(x-x0));
        b + num/den 

    }

    pub fn init(&mut self, limits: &UCILimits, us: Color, ply: i32) {
        self.start_time = limits.start_time;
        let move_num = (ply/2) as f64;
        let mtg = if limits.movestogo > 0 { 
            limits.movestogo } else { 
                MAX_MOVES_TO_GO 
            } as f64;

        let time_left = std::cmp::max(1, limits.time[us] + limits.inc[us] * (mtg as i64 - 1) - 10 * (mtg as i64));
        let factor = Self::factor(move_num, mtg, 0.01, 0.2, 12.0);

        // let max_start = 15;
        // let max_end = 20;

        // let moves_max = max_start - max_end;

        // let scale_diff = 2.0 / (moves_max + mtg) as f32;
        // let scale_min = 0.01;
        // let scale_max = scale_min + scale_diff;

        // // println!("{} {}", scale_min, scale_max);

        // let scale = if move_num < max_start {
        //     scale_min + move_num as f32 * (scale_diff/max_start as f32)
        // } else if move_num > max_end {
        //     scale_max - move_num as f32 * (scale_diff / (mtg - max_end) as f32)  
        // } else {
        //     scale_max
        // };

        self.opt_time = (factor * time_left as f64) as i64;
        self.max_time = std::cmp::max((0.75*time_left as f64) as i64, (1.25*self.opt_time as f64) as i64);

    }

    // Returns time in ms
    pub fn elapsed(&self) -> i64 {
        let duration = self.start_time.elapsed().unwrap();
        (duration.as_secs() * 1000 + (duration.subsec_nanos() / 1000000) as u64)
            as i64
    }

    // Returns time in ms
    pub fn optimum(&self) -> i64 {
        std::cmp::max(self.opt_time, 1)

    }

    // Returns time in ms
    pub fn maximum(&self) -> i64 {
        self.max_time

    }

}
