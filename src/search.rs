use crate::evaluate::score::Value;
use crate::position::Position;
use crate::timeman::TimeManager;
use crate::uci::command::GoOptions;
use crate::movegen::movepick::MovePicker;
use cozy_chess::{Board, Move};
use std::time::{Duration, SystemTime};

pub const MAX_PLY: u32 = 128;
pub const MAX_MOVES: u32 = 128;

#[derive(Debug, Clone, Copy)]
pub struct PrincipalVariation {
    pub moves: [Option<Move>; MAX_MOVES as usize],
}

impl PrincipalVariation {
    pub fn new() -> PrincipalVariation {
        PrincipalVariation {
            moves: [None; MAX_MOVES as usize],
        }
    }

    pub fn update(&mut self, mv: &Move, child_pv: &PrincipalVariation) {
        self.moves[0] = Some(*mv);
        for idx in 0..MAX_MOVES as usize {
            if let Some(child_mv) = child_pv.moves[idx] {
                self.moves[idx + 1] = Some(child_mv);
            } else {
                break;
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for mv in self.moves {
            if mv.is_none() {
                break;
            }
            len += 1;
        }
        len
    }
}

impl core::fmt::Display for PrincipalVariation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for mv_option in self.moves {
            match mv_option {
                Some(mv) => write!(f, "{} ", mv)?,
                None => break,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SearchThread {
    pv: PrincipalVariation,
    ss: Vec<SearchStack>,
    go_options: GoOptions,
    time_manager: TimeManager,
    eval: Value,
}

#[derive(Debug)]
pub struct SearchStack {
    node_count: u64,
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
        }
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
            iter_time.as_millis(),
            self.nodes(),
            self.nps(iter_time),
            self.score(),
            self.pv()
        )
    }

    pub fn search(&mut self, pos: &mut Position) {
        let alpha = -Value(10000);
        let beta = Value(10000);
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
            self.eval = alphabeta(pos, 0, depth, alpha, beta, &mut pv, self);
            self.pv = pv;

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

pub fn alphabeta(
    pos: &mut Position,
    ply: u32,
    depth: i32,
    mut alpha: Value,
    beta: Value,
    pv: &mut PrincipalVariation,
    thread: &mut SearchThread,
) -> Value {
    let mut eval: Value;
    let mut child_pv = PrincipalVariation::new();
    
    // Increment node counter
    if thread.ss.len() <= ply as usize {
        thread.ss.push(SearchStack::new());
    }
    thread.ss[ply as usize].node_count += 1;

    // Return eval for depth 0
    if depth <= 0 {
        thread.ss[ply as usize].node_count -= 1;
        return qsearch(pos, ply, depth, alpha, beta, &mut child_pv, thread);
    }

    // Generate all moves
    // let mut move_list: Vec<Move> = vec![];
    // pos.board.generate_moves(|piece_moves| {
    //     for mv in piece_moves {
    //         move_list.push(mv);
    //     }
    //     false
    // });
    let mut mpick = MovePicker::new();
    
    // Iterate through the moves
    //for mv in move_list {
    while let Some(mv) = mpick.next_move(pos, false) {
        pos.do_move(mv);
        eval = -alphabeta(
            pos,
            ply + 1,
            depth - 1,
            -beta,
            -alpha,
            &mut child_pv,
            thread,
        );
        pos.undo_move();

        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
            pv.update(&mv, &child_pv);
        }
    }
    alpha
}

pub fn qsearch(
    pos: &mut Position,
    ply: u32,
    depth: i32,
    mut alpha: Value,
    beta: Value,
    pv: &mut PrincipalVariation,
    thread: &mut SearchThread,
) -> Value {
    let mut child_pv = PrincipalVariation::new();

    // Increment node counter
    if thread.ss.len() <= ply as usize {
        thread.ss.push(SearchStack::new());
    }
    thread.ss[ply as usize].node_count += 1;

    // Evaluate the position
    let mut eval = pos.eval();
    if eval >= beta {
        return beta;
    }
    if eval > alpha {
        alpha = eval;
    }

    // Generate captures
    // let mut move_list: Vec<Move> = vec![];
    // pos.board.generate_moves(|mut piece_moves| {
    //     piece_moves.to &= pos.board.colors(!pos.board.side_to_move());
    //     for mv in piece_moves {
    //         move_list.push(mv);
    //     }
    //     false
    // });

    let mut mpick = MovePicker::new();
    
    // Iterate through the moves
    //for mv in move_list {
    

    // Iterate through the moves
    while let Some(mv) = mpick.next_move(pos, true) {
        pos.do_move(mv);
        eval = -qsearch(
            pos,
            ply + 1,
            depth - 1,
            -beta,
            -alpha,
            &mut child_pv,
            thread,
        );
        pos.undo_move();

        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
            pv.update(&mv, &child_pv);
        }
    }
    alpha
}

pub fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut nodes = 0;
        board.generate_moves(|moves| {
            for mv in moves {
                let mut board = board.clone();
                board.play_unchecked(mv);
                nodes += perft(&board, depth - 1);
            }
            false
        });
        nodes
    }
}

pub fn perft_bulk(board: &Board, depth: u8) -> u64 {
    let mut nodes = 0;

    match depth {
        0 => nodes += 1,
        1 => {
            board.generate_moves(|moves| {
                nodes += moves.len() as u64;
                false
            });
        }
        _ => {
            board.generate_moves(|moves| {
                for mv in moves {
                    let mut board = board.clone();
                    board.play_unchecked(mv);
                    let child_nodes = perft_bulk(&board, depth - 1);
                    nodes += child_nodes;
                }
                false
            });
        }
    }
    nodes
}
