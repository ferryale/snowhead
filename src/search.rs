use crate::types::square::{SQUARE_NB};
use crate::types::piece::{WHITE, BLACK, PIECE_NB, Color};
use crate::types::r#move::Move;
use crate::types::score::{Depth, Value, mated_in, mate_in, MAX_PLY, MAX_MOVES};
use crate::movegen::ExtMove;
use crate::position::Position;
use crate::evaluate::evaluate;
use crate::movepick::MovePicker;
use crate::tt::{TranspositionTable, TTFlag};
use crate::uciset::{UCILimits};
use crate::timeman::{TimeManager};

#[derive(Debug, Clone, Copy)]
pub struct RootMoves {
    ext_moves: [ExtMove; MAX_MOVES],
    cur: usize,
}

impl RootMoves {
    pub fn new() -> RootMoves {
        RootMoves {
            ext_moves: [ExtMove::new(); MAX_MOVES],
            cur: 0,
        }
        
    }

    pub fn sort(&mut self) {
        self.ext_moves.sort();
        self.cur = 0;
    }

    pub fn next_move(&mut self) -> Move {
        let next_move = self.ext_moves[self.cur].m;
        //self.cur += 1;
        next_move
    }

    pub fn next(&mut self) -> ExtMove {
        let next_move = self.ext_moves[self.cur];
        self.cur += 1;
        next_move
    }

    pub fn push(&mut self, ext_move: ExtMove) {
        self.ext_moves[self.cur] = ext_move;
        self.cur += 1;
    } 
}


#[derive(Debug, Clone, Copy)]
pub struct Stack {
    // ply: usize,
    pv: [Move; MAX_PLY as usize],
    pub killers: [Move; 2],
    node_count: u32
}


impl Stack {
    pub fn new() -> Stack {
        Stack {
            pv: [Move::NONE; MAX_PLY as usize],
            killers: [Move::NONE; 2],
            node_count: 0
        }
    }
}

pub type History = [[Value; SQUARE_NB]; PIECE_NB];
pub const HISTORY_ZERO: History = [[Value(0); SQUARE_NB]; PIECE_NB];

// Rust won't let you own a mut reference to the ttable.
// Just own the ttable for now.
// TODO: figure how how to handle a mut reference to ttable in multi-threading.
// The table needs to be shared by multiple threads.
#[derive(Debug, Clone)]
pub struct Thread {
    pub ss: [Stack; MAX_PLY as usize],
    pub value: Value,
    root_moves: RootMoves,
    ttable: TranspositionTable,
    history: History,
    limits: UCILimits,
    time: TimeManager,
    iter_time: i64,
}

impl Thread {
    pub fn new(tt_size_mb: usize) -> Thread {
    //pub fn new(ttable: TranspositionTable, limits: UCILimits, us: Color, ply: i32) -> Thread {

        let mut thread = Thread {
            ss: [Stack::new(); MAX_PLY as usize],
            value: Value(0),
            root_moves: RootMoves::new(),
            ttable: TranspositionTable::new(tt_size_mb),
            history: HISTORY_ZERO,
            limits: UCILimits::new(),
            time: TimeManager::new(),
            iter_time: 0i64,
        };

        thread.init_stacks();
        thread
    }

    pub fn init(&mut self) {
        self.init_root_moves();
        self.init_stacks();
    }

    pub fn init_time(&mut self, limits: UCILimits, us: Color, ply: i32) {
        self.time.init(&limits, us, ply);
        self.limits = limits;
    }

    pub fn clear_ttable(&mut self) {
        self.ttable.clear();
    }
    

    pub fn depth(&self) -> usize {

        let mut ret: usize = 0;

        for ply in 0..MAX_PLY as usize {
            if self.ss[ply].pv[0] == Move::NONE {
                ret = ply;
                break;
            }
             
        }

        ret
    }

    pub fn seldepth(&self) -> usize {
        
        let mut ret: usize = 0;
        for ply in 0..MAX_PLY as usize {
            if self.ss[ply].node_count == 0 {
                ret = ply;
                break;
            }
        }
        if ret > 0 { ret-1 } else { 0 } 
    }

    pub fn nodes(&self) -> u32 {
        let mut cnt = 0;
        for ply in 0..MAX_PLY as usize {
            if self.ss[ply].node_count == 0 {
                break;
            }
            cnt += self.ss[ply].node_count;
        }
        cnt
    }

    pub fn pv(&self) -> [Move; MAX_PLY as usize] {
        self.ss[0].pv
    }

    fn plies_from_mate(&self) -> i32 {
        Value::MATE.0 - self.value.0.abs()
    }

    pub fn pv_string(&self) -> String {
        let mut ret = String::new();
        let mut ply = 0;
        for m in &self.pv() {
            if *m == Move::NONE { break; }
            if ply >= self.plies_from_mate() { break; }
            ret = format!("{} {}", ret, m.to_string(false));
            ply += 1;
        }
        ret
    }

    pub fn score(&self) -> String {
        let mut ret = String::new();
        if self.value > mate_in(MAX_PLY) {
            ret = format!("mate {}", 1 + self.plies_from_mate()/2) 
        } else if self.value < mated_in(MAX_PLY) {
            ret = format!("mate -{}", self.plies_from_mate()/2) 
        } else {
            ret = format!("cp {}", self.value.0)
        }
        ret
    }

    pub fn time(&self) -> i64 {
        self.time.elapsed()
    }

    pub fn nps(&self) -> u32 {
        let iter_time = std::cmp::max(1, self.iter_time);
         (self.nodes() as f32 / iter_time as f32 * 1000.0) as u32

    }

    // fn branching_factor(&self) -> f32 {
    //     let num_root = (self.root_moves.cur + 1) as f32;
    //     let num_nodes = self.nodes() as f32;
    //     num_root.ln()/num_nodes.ln()
    // }

    // fn branching_factor(&self) -> f32 {
    //     let num_root = (self.root_moves.cur + 1) as f32;
    //     let num_nodes = self.nodes() as f32;
    //     num_root.ln()/num_nodes.ln()
    // }

    // pub fn score(&self) {
    //     self.root_moves
        
    // }

    pub fn info(&self, depth: i32) -> String {

        format!("depth {} seldepth {} time {} nodes {} score {} nps {} pv{}", 
            depth, self.seldepth(), self.time(), self.nodes(), self.score(),
            self.nps(), self.pv_string())
        
    }

    pub fn best_move(&self) -> String {
        let best_move_str = self.pv()[0].to_string(false);
        let ponder_str = self.pv()[1].to_string(false);
        format!("bestmove {} ponder {}", best_move_str, ponder_str)

    }

    pub fn print_info(&self, depth: i32) {
        println!("info {}", self.info(depth));
    }

    pub fn print_best_move(&self) {
        println!("{}", self.best_move());
    }

    fn init_stacks(&mut self) {

        for ply in 0..MAX_PLY as usize {
            self.ss[ply] = Stack::new();
        }

    }

    fn init_root_moves(&mut self) {
        self.root_moves = RootMoves::new();
    }

    fn clear_history(&mut self) {

       self.history = HISTORY_ZERO

    }

    fn print_root_moves(&mut self) {
        loop {
            let ext_move = self.root_moves.next();
            if ext_move.m == Move::NONE { break };
            println!("{} {:?}", ext_move.m.to_string(false), ext_move.value);
        } 
    }

    // fn sort_root_moves(&mut self) {

    //     for ext_move in self.root_moves.iter().enumerate() {
    //         if ext_move
    //     }
    // }

    pub fn search(&mut self, pos: &mut Position) {

       
        let alpha = -Value::INFINITE;
        let beta = Value::INFINITE;
        let ply = 0;
        let mut prev_nodes = 1;
        let mut next_time = 0;
        let mut prev_time = 0;
        let mut elapsed;
        let mut ebf;
        let mut nps;

        let max_depth = if self.limits.infinite || self.limits.use_time_management() {
            MAX_PLY
        } else {
            self.limits.depth as i32
        };
        
        let mut curr_depth = 1;
        //println!("{} {}", self.time.optimum(), self.limits.use_time_management());
        while (curr_depth <= max_depth && !self.limits.use_time_management()) || 
        (self.limits.use_time_management() && next_time < self.time.optimum() || curr_depth <= 5) {
            
            self.clear_history();
            self.init_stacks();

            self.value = search(pos, ply, alpha, beta, Depth(curr_depth), self);

            

            self.root_moves.sort();

            

            // Can we do another iteration?
            elapsed = self.time();
            self.iter_time = elapsed - prev_time;
            nps = std::cmp::max(self.nps(), 1_000_000);

            ebf = self.nodes() as f32 / prev_nodes as f32;
            // next_nodes = self.nodes() as f32 * ebf;

            // next_time = elapsed + (next_nodes as f32 / nps as f32 * 1000.0) as i64;
            next_time = elapsed + (self.iter_time as f32 * ebf) as i64;

            // println!("ebf {} next_nodes {} next_time {} nodes {} prev_nodes {} iter time {}\n", ebf, next_nodes, next_time,
            //     self.nodes(), prev_nodes, self.iter_time);

            prev_nodes = self.nodes();
            
            prev_time = elapsed;

            self.print_info(curr_depth);

            curr_depth += 1;

            

        }

        self.print_best_move();
        

    }
}

// fn update_pv(pv: &mut[Move], m: Move, child_pv: &[Move]){
//     pv[0] = m;
//     let mut idx = 0;
//     while child_pv[idx] != Move::NONE {
//         pv[idx+1] = child_pv[idx];
//         idx += 1;
//     }
    
// }

fn update_pv(ss: &mut [Stack], ply: usize, m: Move){
    ss[ply].pv[0] = m;
    let mut idx = 0;
    while ss[ply+1].pv[idx] != Move::NONE {
        ss[ply].pv[idx+1] = ss[ply+1].pv[idx];
        idx += 1;
    }
    
}

fn update_killers(ss: &mut [Stack], ply: usize, m: Move) {
    if m != ss[ply].killers[0]{
        ss[ply].killers[1] = ss[ply].killers[0];
        ss[ply].killers[0] = m;
    }
}


fn search(pos: &mut Position, ply: usize, mut alpha: Value, beta: Value, depth: Depth, thread: &mut Thread) -> Value {

    thread.ss[ply].node_count += 1;

    let mut num_legal = 0;
    let mut num_played = 0;
    let mut red = 0;
    let root_node = ply == 0;
    let pv_node = beta - alpha > Value(1);

    let mut value;

    let (tt_hit, tt_value, tt_flag, tt_depth, tt_move) = thread.ttable.probe(pos.key());

    // If tt_hit return the move immediately
    if tt_hit && tt_move != Move::NONE && tt_depth >= depth && pos.legal(tt_move) {
        if tt_flag == TTFlag::LOWER && tt_value >= beta {
            return beta;
        }
        // This is unstable!
        // if tt_flag == TTFlag::EXACT && tt_depth == depth {
        //     //thread.ss[ply].pv[ply] = tt_move;
        //     //update_pv(&mut thread.ss, ply, tt_move);
        //     return tt_value;
        // }
        if tt_flag == TTFlag::UPPER && tt_value <= alpha {
            return alpha;
        }
    }

    // Checks for 50 rule count and repetition draw. Stalemate is handled later.
    if pos.is_draw(ply as i32) {
        return Value::DRAW;
    }

    // For depth <=0 go in quiescent search
    if depth <= Depth(0) {
        thread.ss[ply].node_count -= 1;
        return qsearch(pos, ply, alpha, beta, Depth(0), thread);
    }

    // Null move pruning
    if pos.checkers() == 0 {
        pos.do_null_move();
        value = -search(pos, ply+2, -beta, -beta + 1, depth - 3, thread);
        pos.undo_null_move();
        if value >= beta {
            return beta;
        }
    }
    
    // Init movepicker

    let mut mp = MovePicker::new(pos, tt_move, ply, depth, &mut thread.ss, thread.history);

    loop {


        let m = if root_node && depth > Depth(1) {
            // At a root_node, get the next move from root_moves, sorted based on previous iteration
            let tmp = thread.root_moves.next_move();
            //println!("{}", tmp.to_string(false));
            tmp
        } else { 
            // At a non root_node, het the next_move from the movepicker.
            mp.next_move(pos, false)
        };

        if m == Move::NONE { break; }
        if !pos.legal(m) { continue; }
        num_legal += 1;

        // TODO: Add some pruning here

        pos.do_move(m);

        num_played += 1;

        // PVS, first node of PV line with full window, other nodes with null-window.
        if pv_node && num_played == 1 {
            value =  -search(pos, ply+1, -beta, -alpha, depth-1, thread);
        } else {
            red = 2;
            value =  -search(pos, ply+1, -alpha-1, -alpha, depth-1-red, thread);
            if value > alpha && red > 0 {
                value =  -search(pos, ply+1, -alpha-1, -alpha, depth-1, thread);
            }
            if value > alpha && (root_node || value < beta) {
                value =  -search(pos, ply+1, -beta, -alpha, depth-1, thread);
            }

        }
        
        pos.undo_move(m);

        if value >= beta { // Fail high.
            // Update TT
            thread.ttable.save(pos.key(), beta, TTFlag::LOWER, depth, m);

            if !pos.capture(m) {

                // Store move as killer. Even if movepick checks that the move in a non capture,
                // we only update the killer when the move is a capture to avoid wasting killer slots.
                update_killers(&mut thread.ss, ply, m);

                // Update history
                thread.history[pos.piece_on(m.from())][m.to()] += Value(depth.0 * depth.0);
            }

            return beta;
        }

        if value > alpha { // New PV move
            alpha = value;
            thread.ttable.save(pos.key(), value, TTFlag::EXACT, depth, m);
            update_pv(&mut thread.ss, ply, m);
        } else { // fail low
            thread.ttable.save(pos.key(), alpha, TTFlag::UPPER, depth, Move::NONE);
        }

        if root_node { 
            thread.root_moves.push(ExtMove{m: m, value: alpha});
        }

    }


    // If there are no legal moves at this point, it is either checkmate or stalemate
    if num_legal == 0 {
        // Stalemate
        if pos.checkers() == 0 {
            return Value::DRAW;
        } else {
            return mated_in(ply as i32);
        } 
    }

    alpha

}

fn qsearch(pos: &mut Position, ply: usize, mut alpha: Value, beta: Value, depth: Depth, thread: &mut Thread) -> Value {
    thread.ss[ply].node_count += 1;

    // Checks for 50 rule count and repetition draw. Stalemate is handled later.
    if pos.is_draw(ply as i32) {
        return Value::DRAW;
    }

    let mut value = evaluate(pos);
    if value >= beta {
        return beta;
    }
    if value > alpha {
        alpha = value;
    }

    let mut mp = MovePicker::new(pos, Move::NONE, ply, depth, &mut thread.ss, thread.history);
    let mut num_moves = 0;

    // let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];
    
    // if pos.checkers() == 0 {
    //     num_moves = generate(CAPTURES, &pos, &mut list, num_moves);
    //     num_moves = generate(QUIET_CHECKS, &pos, &mut list, num_moves);

    // } else {
    //     num_moves = generate(EVASIONS, &pos, &mut list, num_moves);
    // }
    

    loop {
        let m = mp.next_move(pos, true);
        if m == Move::NONE { break; }
        if !pos.legal(m) { continue; }
        num_moves += 1;

        pos.do_move(m);

        value = -qsearch(pos, ply+1, -beta, -alpha, depth-1, thread);

        pos.undo_move(m);

        if value >= beta {
            update_killers(&mut thread.ss, ply, m);
            return beta;
        }
        if value > alpha {
            alpha = value;
        }

    }

    // If there are no moves at this point ans we are in check, it is checkmate, 
    // since all evasions have been generated
    // If there are no moves and we are not in check, it is not necessarily stalemate,
    // since not all moves are generated in qsearch
    if num_moves == 0 && pos.checkers() != 0 {
        return mated_in(ply as i32);
    } 

    alpha

}

