use self::pv::PrincipalVariation;
use self::thread::SearchThread;
use crate::evaluate::score::Value;
use crate::movegen::movepick::MovePicker;
use crate::position::Position;

pub mod perft;
pub mod pv;
pub mod rootmoves;
pub mod thread;

/* Alphabeta recursive algorithm */
pub fn alphabeta(
    pos: &mut Position,
    ply: u32,
    depth: i32,
    mut alpha: Value,
    beta: Value,
    pv: &mut PrincipalVariation,
    thread: &mut SearchThread,
) -> Value {
    // Init search variables
    let mut eval: Value;
    let mut child_pv = PrincipalVariation::new();
    let mut num_legal = 0;
    let root_node = ply == 0;

    // Return eval for depth 0
    if depth <= 0 {
        return qsearch(pos, ply, depth, alpha, beta, &mut child_pv, thread);
    }

    // Increment node counter
    thread.incr_node_count(ply);

    // Null move pruning
    let played_null = pos.do_null_move();
    if played_null {
        eval = -alphabeta(
            pos,
            ply + 1,
            depth - 1,
            -beta,
            -beta + 1,
            &mut child_pv,
            thread,
        );
        pos.undo_move();
        if eval >= beta {
            return beta;
        }
    }

    // Init movepicker
    let mut mpick = MovePicker::new();

    /* Iterate through the moves */
    loop {
        /*
            At root nodes and depth one, next move is takes from sorted root_moves.
            In other cases, next move comes from movepicker.
        */
        let mv_option = if root_node && depth > 1 {
            thread.next_root_move()
            //mpick.next_move(pos, false)
        } else {
            mpick.next_move(pos, false)
        };

        // Break when there are no moves left
        if mv_option.is_none() {
            break;
        }

        // Unwrap the move option
        let mv = mv_option.unwrap();

        // Increment legal move counter
        num_legal += 1;

        // Play+eval+undo
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

        // Fail high: return beta
        if eval >= beta {
            return beta;
        }

        // New best move: update alpha and pv
        if eval > alpha {
            alpha = eval;
            pv.update(&mv, &child_pv);
        }

        // At rootnodes, insert the move into the root_moves list
        if root_node {
            thread.update_root_moves(mv, alpha, depth);
        }
    } // loop

    // If there are no legal moves at this point, it is either checkmate or stalemate
    if num_legal == 0 {
        if !pos.is_check() {
            // Stalemate
            return Value::DRAW;
        } else {
            // Checkmate
            return Value::mated_in(ply);
        }
    }

    alpha
}

/* Quiescent search algorithm */
pub fn qsearch(
    pos: &mut Position,
    ply: u32,
    depth: i32,
    mut alpha: Value,
    beta: Value,
    pv: &mut PrincipalVariation,
    thread: &mut SearchThread,
) -> Value {
    // Init search variables
    let mut child_pv = PrincipalVariation::new();
    let mut num_moves = 0;

    // Increment node counter
    thread.incr_node_count(ply);

    // Evaluate the position
    let mut eval = pos.evaluate();

    // Fail high: return beta
    if eval >= beta {
        return beta;
    }

    // New best move: update alpha
    if eval > alpha {
        alpha = eval;
    }

    // Init movepicker
    let mut mpick = MovePicker::new();

    // Iterate through the moves
    while let Some(mv) = mpick.next_move(pos, true) {
        // Update move counter
        num_moves += 1;

        // Play+eval+undo
        pos.do_move(mv);
        eval = -qsearch(
            pos,
            ply + 1,
            depth - 3,
            -beta,
            -alpha,
            &mut child_pv,
            thread,
        );
        pos.undo_move();

        // Fail high: return beta
        if eval >= beta {
            return beta;
        }
        // New best move: update alpha and pv
        if eval > alpha {
            alpha = eval;
            pv.update(&mv, &child_pv);
        }
    }

    /*
        If there are no moves at this point and we are in check, it is checkmate,
        since all evasions have been generated.
        If there are no moves and we are not in check, it is not necessarily stalemate,
        since not all moves are generated in qsearch.
    */
    if num_moves == 0 && pos.is_check() {
        // Checkmate
        return Value::mated_in(ply);
    }

    alpha
}
