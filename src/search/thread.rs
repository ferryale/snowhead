use super::alphabeta;
use super::pv::PrincipalVariation;
use super::rootmoves::RootMoves;
use crate::evaluate::score::Value;
use crate::movegen::movevalue::MoveValue;
use crate::position::Position;
use crate::timeman::TimeManager;
use crate::uci::command::GoOptions;
use cozy_chess::Move;
use std::time::{Duration, SystemTime};

pub const MAX_PLY: u32 = 128;

/* Thread that launches a search */
#[derive(Debug)]
pub struct SearchThread {
    pv: PrincipalVariation,
    ss: Vec<SearchStack>,
    go_options: GoOptions,
    time_manager: TimeManager,
    eval: Value,
    root_moves: RootMoves,
}

/* SearchStack entry */
#[derive(Debug)]
pub struct SearchStack {
    node_count: u64,
}

/* Search stack implementation */
impl SearchStack {
    // Constructor
    pub fn new() -> SearchStack {
        SearchStack { node_count: 0 }
    }

    // Increments node count
    fn incr_node_count_by(&mut self, incr: u64) {
        self.node_count += incr;
    }

    // Increments node count
    fn decr_node_count_by(&mut self, incr: u64) {
        self.node_count -= incr;
    }
}

/* Search thread implementation */
impl SearchThread {
    // Constructor needs go_options and time manager, passed by uci module
    pub fn new(go_options: GoOptions, time_manager: TimeManager) -> SearchThread {
        SearchThread {
            pv: PrincipalVariation::new(),
            ss: vec![],
            go_options: go_options,
            time_manager: time_manager,
            eval: Value::ZERO,
            root_moves: RootMoves::new(),
        }
    }

    // Updates root moves
    pub fn update_root_moves(&mut self, mv: Move, alpha: Value, depth: i32) {
        self.root_moves.insert(MoveValue::new(mv, alpha), depth);
    }

    // Returns the next root move
    pub fn next_root_move(&mut self) -> Option<Move> {
        self.root_moves.next_move()
    }

    // Push stack inserts a new search stack into the vector
    pub fn push_new_stack(&mut self) {
        self.ss.push(SearchStack::new());
    }

    // Increments node count at given ply
    pub fn incr_node_count(&mut self, ply: u32) {
        self.ss[ply as usize].incr_node_count_by(1)
    }

    // Decrements node count at given ply
    pub fn decr_node_count(&mut self, ply: u32) {
        self.ss[ply as usize].decr_node_count_by(1)
    }

    // Returns the stack size
    pub fn ss_len(&self) -> usize {
        self.ss.len()
    }

    /* Helper methods */

    // Resets the stack vector
    fn init_stacks(&mut self) {
        self.ss = vec![];
    }

    // Returns the depth searched
    fn depth(&self) -> usize {
        self.pv.len()
    }

    // Returns the seldepth searched
    fn seldepth(&self) -> usize {
        self.ss.len()
    }

    // Returns the number of nodes searched
    fn nodes(&self) -> u64 {
        let mut cnt = 0;
        for stack in &self.ss {
            if stack.node_count == 0 {
                break;
            }
            cnt += stack.node_count;
        }
        cnt
    }

    // Returns the time elapsed since search start
    fn elapsed_time(&self) -> Duration {
        self.time_manager.elapsed()
    }

    // Returns the number of nodes per second searched
    fn nps(&self, iter_time: Duration) -> u64 {
        1_000_000 * self.nodes() / iter_time.as_micros() as u64
    }

    // Returns the search score to display to uci
    fn score(&self) -> i32 {
        self.eval.0 as i32
    }

    // Returns the PrincipalVariation as a string
    fn pv(&self) -> String {
        format!("{}", self.pv)
    }

    // Returns the best move as string
    fn best_move(&self) -> String {
        if self.pv.len() > 1 {
            format!(
                "bestmove {} ponder {}",
                self.pv.moves[0].unwrap(),
                self.pv.moves[1].unwrap()
            )
        } else {
            format!("bestmove {}", self.pv.moves[0].unwrap())
        }
    }

    // Returns the uci info as string
    fn info(&self, elapsed_time: Duration) -> String {
        format!(
            "info depth {} seldepth {} time {} nodes {} nps {} score {} pv {}",
            self.depth(),
            self.seldepth(),
            elapsed_time.as_millis(),
            self.nodes(),
            self.nps(elapsed_time),
            self.score(),
            self.pv()
        )
    }

    /* Main method: iterative deepening search on Position pos */
    pub fn search(&mut self, pos: &mut Position) {
        // Variables init
        let alpha = -Value::INFINITE;
        let beta = Value::INFINITE;
        let mut pv = PrincipalVariation::new();
        let mut prev_nodes = 1;
        let mut depth = 1;

        // Declare helper variables
        let mut start_time: SystemTime;
        let mut iter_time: Duration;
        let mut next_time: Duration;
        let mut elapsed_time: Duration;
        let mut node_ratio: u64;

        /*
            Computes max_depth based on go options.
            If we use time management, the depth is set to max,
            and time managing will decide when to stop searching
        */
        let max_depth = if self.go_options.use_time_management() {
            MAX_PLY as i32
        } else {
            self.go_options.depth as i32
        };

        while depth <= max_depth {
            // Get the time at the beginning of the iteration
            start_time = TimeManager::current();

            // Reset the stacks
            self.init_stacks();

            // Resets the position psq for incremental updates
            pos.init_psq();

            // Launch alphabeta algorithm which returns the evaluation
            self.eval = alphabeta(pos, 0, depth, alpha, beta, &mut pv, self);

            // Stores the pv
            self.pv = pv;

            // Sort the root moves for next iteration
            self.root_moves.sort();

            /* Time management */

            // Elapsed times
            iter_time = TimeManager::elapsed_since(start_time);
            elapsed_time = self.elapsed_time();

            // Ratio between number of nodes of this versus previous iteration
            node_ratio = std::cmp::min(self.nodes() / prev_nodes, 5);

            // Estimates the time for next iteration
            next_time = iter_time * node_ratio as u32;

            // Display uci info
            println!("{}", self.info(elapsed_time));

            // Stop searching if the estimated time for next iter exceeds allocated thinking time
            if (next_time + self.elapsed_time() >= self.time_manager.optimum())
                & self.go_options.use_time_management()
            {
                break;
            }

            // Update search variables
            prev_nodes = self.nodes();
            depth += 1;
        } // while

        // Disply best move to uci
        println!("{}", self.best_move());
    }
}
