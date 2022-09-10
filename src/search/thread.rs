use super::alphabeta;
use super::pv::PrincipalVariation;
use super::rootmoves::RootMoves;
use crate::evaluate::score::Value;
use crate::position::Position;
use crate::timeman::TimeManager;
use crate::uci::command::GoOptions;
use std::time::{Duration, SystemTime};

pub const MAX_PLY: u32 = 128;

#[derive(Debug)]
pub struct SearchThread {
    pv: PrincipalVariation,
    pub ss: Vec<SearchStack>,
    go_options: GoOptions,
    time_manager: TimeManager,
    eval: Value,
    pub root_moves: RootMoves,
}

#[derive(Debug)]
pub struct SearchStack {
    pub node_count: u64,
}

impl SearchStack {
    pub fn new() -> SearchStack {
        SearchStack { node_count: 0 }
    }
}

impl SearchThread {
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

    fn init_stacks(&mut self) {
        self.ss = vec![];
    }

    fn depth(&self) -> usize {
        self.pv.len()
    }

    fn seldepth(&self) -> usize {
        self.ss.len()
    }

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

    fn elapsed_time(&self) -> Duration {
        self.time_manager.elapsed()
    }

    fn nps(&self, iter_time: Duration) -> u64 {
        1_000_000 * self.nodes() / iter_time.as_micros() as u64
    }

    fn score(&self) -> i32 {
        self.eval.0 as i32
    }

    fn pv(&self) -> String {
        format!("{}", self.pv)
    }

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

    pub fn info(&self, iter_time: Duration) -> String {
        format!(
            "info depth {} seldepth {} time {} nodes {} nps {} score {} pv {}",
            self.depth(),
            self.seldepth(),
            self.elapsed_time().as_millis(),
            self.nodes(),
            self.nps(self.elapsed_time()),
            self.score(),
            self.pv()
        )
    }

    pub fn search(&mut self, pos: &mut Position) {
        let alpha = -Value::INFINITE;
        let beta = Value::INFINITE;
        let mut pv = PrincipalVariation::new();
        let mut prev_nodes = 1;
        let mut start_time: SystemTime;
        let mut iter_time: Duration;
        let mut next_time: Duration;
        let mut node_ratio: u64;

        let max_depth = if self.go_options.use_time_management() {
            MAX_PLY as i32
        } else {
            self.go_options.depth as i32
        };

        let mut depth = 1;

        while depth <= max_depth {
            start_time = TimeManager::current();
            pos.init_psq();
            //self.init_stacks();

            self.eval = alphabeta(pos, 0, depth, alpha, beta, &mut pv, self);
            self.pv = pv;

            self.root_moves.sort();

            //self.root_moves.print();

            iter_time = TimeManager::elapsed_since(start_time);
            node_ratio = std::cmp::min(self.nodes() / prev_nodes, 5);
            next_time = iter_time * node_ratio as u32;

            println!("{}", self.info(iter_time));

            if (next_time + self.elapsed_time() >= self.time_manager.optimum())
                & self.go_options.use_time_management()
            {
                break;
            }

            prev_nodes = self.nodes();
            depth += 1;
        }

        println!("{}", self.best_move());
    }
}
