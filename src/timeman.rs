use crate::uci::command::GoOptions;
use crate::uci::option::UciOptions;
use cozy_chess::Color;
use std::time::{Duration, SystemTime};

pub const MAX_MOVES_TO_GO: u64 = 20;

#[derive(Debug, Clone, Copy)]
pub struct TimeManager {
    start_time: SystemTime,
    opt_time: Duration,
    max_time: Duration,
}

impl TimeManager {
    pub fn new(
        start_time: SystemTime,
        go_options: &GoOptions,
        uci_options: &UciOptions,
        us: Color,
    ) -> TimeManager {
        let mut time_manager = TimeManager {
            start_time: start_time,
            opt_time: Duration::from_millis(0),
            max_time: Duration::from_millis(0),
        };
        time_manager.init(go_options, uci_options, us);
        time_manager
    }

    fn init(&mut self, go_options: &GoOptions, uci_options: &UciOptions, us: Color) {
        if !go_options.use_time_management() {
            return;
        }
        if go_options.movetime > 0 {
            self.opt_time = Duration::from_millis(std::cmp::max(
                9 * go_options.movetime / 10 - uci_options.move_overhead,
                0,
            ));
            self.max_time = Duration::from_millis(std::cmp::max(
                go_options.movetime - uci_options.move_overhead,
                0,
            ));
        } else {
            let mtg = std::cmp::max(go_options.movestogo, MAX_MOVES_TO_GO);
            let time_left = go_options.time[us as usize] + go_options.inc[us as usize] * mtg
                - uci_options.move_overhead;

            self.opt_time = Duration::from_millis(time_left / mtg);
            self.max_time =
                Duration::from_millis(std::cmp::max(6 / 5 * time_left / mtg, 3 / 5 * time_left));
        }
    }

    // Returns duration
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed().unwrap()
    }

    // Returns duration
    pub fn optimum(&self) -> Duration {
        self.opt_time
    }

    // Returns duration
    pub fn maximum(&self) -> Duration {
        self.max_time
    }

    // Returns duration
    pub fn current() -> SystemTime {
        SystemTime::now()
    }

    pub fn elapsed_since(previous_time: SystemTime) -> Duration {
        SystemTime::now().duration_since(previous_time).unwrap()
    }
}

impl GoOptions {
    pub fn use_time_management(self) -> bool {
        self.mate == 0 && self.depth == 0 && self.nodes == 0 && self.perft == 0 && !self.infinite
    }
}
