use crate::uci::command::GoOptions;
use crate::uci::option::UciOptions;
use cozy_chess::Color;
use std::time::{Duration, SystemTime};

pub const MAX_MOVES_TO_GO: u64 = 20;

/* Time Manager */
#[derive(Debug, Clone, Copy)]
pub struct TimeManager {
    start_time: SystemTime,
    opt_time: Duration,
    max_time: Duration,
}

/* Time Manager implementation */
impl TimeManager {
    // Constructor
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

    // Init helper for constructor
    fn init(&mut self, go_options: &GoOptions, uci_options: &UciOptions, us: Color) {
        if !go_options.use_time_management() {
            return;
        }

        /* Determine the time to allocate to next move */
        //If movetime is given in go_options, take movetime
        let movetime = if go_options.movetime > 0 {
            Duration::from_millis(std::cmp::max(
                go_options.movetime - uci_options.move_overhead,
                0,
            ))
        } else {
            // Else calculate it from time left and increments
            let mtg = std::cmp::max(go_options.movestogo, MAX_MOVES_TO_GO);
            let time_left = go_options.time[us as usize] + go_options.inc[us as usize] * mtg
                - uci_options.move_overhead;

            Duration::from_millis(time_left / mtg)
        };

        // // Optimal time is 9/10 if movetime, max time 100%
        // if go_options.movetime > 0 {
        //     self.opt_time = Duration::from_millis(std::cmp::max(
        //         9 * go_options.movetime / 10 - uci_options.move_overhead,
        //         0,
        //     ));
        //     self.max_time = Duration::from_millis(std::cmp::max(
        //         go_options.movetime - uci_options.move_overhead,
        //         0,
        //     ));
        // } else {
        //     let mtg = std::cmp::max(go_options.movestogo, MAX_MOVES_TO_GO);
        //     let time_left = go_options.time[us as usize] + go_options.inc[us as usize] * mtg
        //         - uci_options.move_overhead;

        //     self.opt_time = Duration::from_millis(time_left / mtg);
        //     self.max_time =
        //         Duration::from_millis(std::cmp::max(6 / 5 * time_left / mtg, 3 / 5 * time_left));
        // }

        // // Optimal time is 9/10 if movetime, max time 100%
        // if go_options.movetime > 0 {
        //     self.opt_time = Duration::from_millis(std::cmp::max(
        //         9 * go_options.movetime / 10 - uci_options.move_overhead,
        //         0,
        //     ));
        //     self.max_time = Duration::from_millis(std::cmp::max(
        //         go_options.movetime - uci_options.move_overhead,
        //         0,
        //     ));
        // } else {
        //     let mtg = std::cmp::max(go_options.movestogo, MAX_MOVES_TO_GO);
        //     let time_left = go_options.time[us as usize] + go_options.inc[us as usize] * mtg
        //         - uci_options.move_overhead;

        //     self.opt_time = Duration::from_millis(time_left / mtg);
        //     self.max_time =
        //         Duration::from_millis(std::cmp::max(6 / 5 * time_left / mtg, 3 / 5 * time_left));
        // }

        self.opt_time = 9 * movetime / 10;
        self.max_time = movetime;
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

    // Returns duration since previous time
    pub fn elapsed_since(previous_time: SystemTime) -> Duration {
        SystemTime::now().duration_since(previous_time).unwrap()
    }

    // Check if optimum search time has exceeded
    pub fn should_stop(&self) -> bool {
        self.elapsed() >= self.optimum()
    }

    // Check if max search time has exceeded
    pub fn must_stop(&self) -> bool {
        self.elapsed() >= self.maximum()
    }
}

impl GoOptions {
    // Choose if time management of fixed depth are implementedss
    pub fn use_time_management(self) -> bool {
        self.mate == 0 && self.depth == 0 && self.nodes == 0 && self.perft == 0 && !self.infinite
    }
}
