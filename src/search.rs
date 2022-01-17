use crate::types::r#move::Move;
use crate::types::score::{Depth, Value, mated_in, MAX_PLY, MAX_MOVES};
use crate::movegen::ExtMove;
use crate::position::Position;
use crate::evaluate::evaluate;
use crate::movepick::MovePicker;
use crate::tt::{TranspositionTable, TTFlag};

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    // ply: usize,
    pv: [Move; MAX_PLY as usize],
    pub killers: [Move; 2],
    node_count: u32
}


impl Stack {
    // pub fn new(ply: usize) -> Stack {
    //     Stack {
    //         // ply: ply,
    //         pv: [Move::NONE; MAX_PLY as usize],
    //         killers: [Move::NONE; 2],
    //         node_count: 0
    //     }
    // }

    pub fn new() -> Stack {
        Stack {
            // ply: ply,
            pv: [Move::NONE; MAX_PLY as usize],
            killers: [Move::NONE; 2],
            node_count: 0
        }
    }
}

// Rust won't let you own a mut reference to the ttable.
// Just own the ttable for now.
// TODO: figure how how to handle a mut reference to ttable in multi-threading.
// The table needs to be shared by multiple threads.
#[derive(Debug, Clone)]
pub struct Thread {
    pub ss: [Stack; MAX_PLY as usize],
    pub value: Value,
    root_moves: [ExtMove; MAX_MOVES],
    root_idx: usize,
    ttable: TranspositionTable
}

impl Thread {
    pub fn new(tt_size_mb: usize) -> Thread {

        let mut thread = Thread {
            ss: [Stack::new(); MAX_PLY as usize],
            value: Value(0),
            root_moves: [ExtMove::new(); MAX_MOVES],
            root_idx: 0,
            ttable: TranspositionTable::new(tt_size_mb)
        };

        thread.init_stacks();
        thread
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

    pub fn pv_string(&self) -> String {
        let mut ret = String::new();
        for m in &self.pv() {
            if *m == Move::NONE { break; }
            ret = format!("{} {}", ret, m.to_string(false));
        }
        ret
    }

    pub fn score_cp(&self) -> i32 {
        self.value.0
    }

    // pub fn score(&self) {
    //     self.root_moves
        
    // }

    pub fn info(&self) -> String {

        format!("depth {} seldepth {} nodes {} score cp {} pv{}", 
            self.depth(), self.seldepth(), self.nodes(), self.score_cp(), self.pv_string())
        
    }

    pub fn best_move(&self) -> String {
        let best_move_str = self.pv()[0].to_string(false);
        let ponder_str = self.pv()[1].to_string(false);
        format!("bestmove {} ponder {}", best_move_str, ponder_str)

    }

    pub fn print_info(&self) {
        println!("info {}", self.info());
    }

    pub fn print_best_move(&self) {
        println!("{}", self.best_move());
    }

    fn init_stacks(&mut self) {

        for ply in 0..MAX_PLY as usize {
            self.ss[ply] = Stack::new();
        }

    }

    // fn sort_root_moves(&mut self) {

    //     for ext_move in self.root_moves.iter().enumerate() {
    //         if ext_move
    //     }
    // }

    pub fn search(&mut self, pos: &mut Position, depth: Depth) {

       
        let alpha = -Value::INFINITE;
        let beta = Value::INFINITE;
        let ply = 0;

        for curr_depth in 1..depth.0+1 {
            self.value = search(pos, ply, alpha, beta, Depth(curr_depth), self);
            
            self.print_info();

            self.root_moves.sort();
            self.root_idx = 0;

            if curr_depth < depth.0 {
                self.init_stacks();
            }

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

    let mut value;

    let (tt_hit, tt_value, tt_flag, tt_depth, tt_move) = thread.ttable.probe(pos.key());

    // If tt_hit return the move immediately
    if tt_hit && tt_move != Move::NONE && tt_depth >= depth {
        if tt_flag == TTFlag::LOWER && tt_value >= beta {
            return beta;
        }
        if tt_flag == TTFlag::EXACT {
            update_pv(&mut thread.ss, ply, tt_move);
            return tt_value;
        }
        if tt_flag == TTFlag::UPPER && tt_value <= alpha {
            return alpha;
        }
    }

    // Checks for 50 rule count and repetition draw. Stalemate is handled later.
    if pos.is_draw(ply as i32) {
        return Value::DRAW;
    }

    if depth == Depth(0) {
        //return evaluate(pos);
        thread.ss[ply].node_count -= 1;
        return qsearch(pos, ply, alpha, beta, Depth(0), thread);
        // value = qsearch(pos, ply, alpha, beta, Depth(0), thread);
        // thread.ttable.save(pos.key(), value, TTFlag::EXACT, depth, Move::NONE);
        // return value;
    }

    let mut mp = MovePicker::new(pos, tt_move, ply, depth, &mut thread.ss);
    let mut num_legal = 0;
    let root_node = ply == 0;

    loop {

        let m = if root_node && depth > Depth(1) {
            thread.root_moves[thread.root_idx].m
            
        } else { 
            mp.next_move(pos, false)
        };

        if m == Move::NONE { break; }
        if !pos.legal(m) { continue; }
        num_legal += 1;

        pos.do_move(m);
        
        value =  -search(pos, ply+1, -beta, -alpha, depth-1, thread);

        pos.undo_move(m);


        if value >= beta { // Fail high.
            // Store move as killer.
            // No need to check if it is a capture because movepicker already does it.
            thread.ttable.save(pos.key(), beta, TTFlag::LOWER, depth, m);
            update_killers(&mut thread.ss, ply, m); 
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
            //println!("depth {}, move {}", depth, m.to_string(false));
            thread.root_moves[thread.root_idx] = ExtMove { m: m, value: value};
            thread.root_idx += 1; 
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

    let mut mp = MovePicker::new(pos, Move::NONE, ply, depth, &mut thread.ss);
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

