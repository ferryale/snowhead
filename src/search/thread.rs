use super::alphabeta;
use super::pv::PrincipalVariation;
use super::rootmoves::RootMoves;
use crate::evaluate::score::Value;
use crate::movegen::movevalue::MoveValue;
use crate::position::Position;
use crate::timeman::TimeManager;
use crate::uci::command::GoOptions;
use cozy_chess::Move;
use std::time::Duration;

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
    node_count: u64,
    aborted: bool,
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
            node_count: 0,
            aborted: false,
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

    // Increments node count at given ply
    pub fn incr_node_count(&mut self, ply: u32) {
        if self.ss.len() <= ply as usize {
            self.push_new_stack();
        }
        self.ss[ply as usize].incr_node_count_by(1);
        if ply > 0 {
            self.node_count += 1;
        }
    }

    // Check if we can do another iteration
    pub fn can_search_deeper(&self, node_ratio: u64) -> bool {
        if !self.go_options.use_time_management() {
            return true;
        }

        let elapsed_time = self.elapsed_time();

        if elapsed_time >= self.time_manager.optimum() / 2 {
            return false;
        }

        // // Elapsed time
        // let iter_time = TimeManager::elapsed_since(iter_start);

        // Estimates the time for next iteration
        let next_time = elapsed_time * node_ratio as u32;

        if next_time >= self.time_manager.maximum() {
            return false;
        }

        true
    }

    // Check if we must stop searching
    pub fn must_stop_searching(&mut self) -> bool {
        if self.aborted {
            return true;
        }

        self.aborted = self.go_options.use_time_management()
            && self.nodes() % 1024 == 0
            && self.time_manager.must_stop();

        self.aborted
    }

    // Check if search was aborted
    pub fn aborted(&self) -> bool {
        self.aborted
    }

    /* Helper methods */

    // Push stack inserts a new search stack into the vector
    fn push_new_stack(&mut self) {
        self.ss.push(SearchStack::new());
    }

    // Resets the stack vector
    // fn init_stacks(&mut self) {
    //     self.ss = vec![];
    // }

    // // Returns the number of nodes searched at given depth
    // fn depth_nodes(&self, depth: i32) -> u64 {
    //     if depth <= 1 {
    //         return
    //     }
    //     let prev_nodes = self.ss[depth - 1 as usize]
    // }

    // // Returns the node ratio between current and previous iterations
    // fn node_ratio(&self, prev_nodes: u64) -> u32 {

    // }

    // Returns the depth searched
    fn depth(&self) -> usize {
        self.pv.len()
    }

    // Returns the seldepth searched
    fn seldepth(&self) -> usize {
        // Remove depth 0: substract - 1
        self.ss.len() - 1
    }

    // Returns the number of nodes searched
    fn nodes(&self) -> u64 {
        // let mut cnt = 0;
        // // Start at idx=1 to remove depth 0 nodes
        // for stack in &self.ss[1..] {
        //     if stack.node_count == 0 {
        //         break;
        //     }
        //     cnt += stack.node_count;
        // }
        // cnt
        self.node_count
    }

    // Returns the time elapsed since search start
    fn elapsed_time(&self) -> Duration {
        self.time_manager.elapsed()
    }

    // Returns the number of nodes per second searched
    fn nps(&self) -> u64 {
        1_000_000 * self.nodes() / self.elapsed_time().as_micros() as u64
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
    fn info(&self) -> String {
        format!(
            "info depth {} seldepth {} time {} nodes {} nps {} score {} pv {}",
            self.depth(),
            self.seldepth(),
            self.elapsed_time().as_millis(),
            self.nodes(),
            self.nps(),
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
        //let mut iter_start: SystemTime;
        // let mut iter_time: Duration;
        // let mut next_time: Duration;
        // let mut elapsed_time: Duration;
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
            //iter_start = TimeManager::current();

            // Reset the stacks: FIND OUT -> report total or iter node count?
            //self.init_stacks();

            // Resets the position psq for incremental updates
            pos.init_psq();

            // Launch alphabeta algorithm which returns the evaluation
            self.eval = alphabeta(pos, 0, depth, alpha, beta, &mut pv, self);

            // If search was aborted, return result from previous iteration
            if self.aborted() {
                break;
            }

            // Stores the pv
            self.pv = pv;

            // Sort the root moves for next iteration
            self.root_moves.sort();

            // Debug
            //println!("{:?}", self.ss);

            /* Time management */

            // Ratio between number of nodes of this versus previous iteration
            node_ratio = std::cmp::min(self.nodes() / prev_nodes + 1, 20);
            //println!("{}", node_ratio);

            // Display uci info
            println!("{}", self.info());

            // println!("{:?} vs {:?}", next_time + self.elapsed_time(), self.time_manager.optimum());

            // Stop searching if the estimated time for next iter exceeds allocated thinking time
            if !self.can_search_deeper(node_ratio) {
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
