use self::pv::PrincipalVariation;
use self::thread::{SearchStack, SearchThread};
use crate::evaluate::score::Value;
use crate::movegen::movepick::MovePicker;
use crate::movegen::movevalue::MoveValue;
use crate::position::Position;
use cozy_chess::Board;

pub mod pv;
pub mod rootmoves;
pub mod thread;

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
    let mut num_legal = 0;
    let root_node = ply == 0;

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

    // Null move pruning
    let played_null = pos.do_null_move();
    if played_null {
        eval = -alphabeta(
            pos,
            ply + 1,
            depth - 4,
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

    // Iterate through the moves
    loop {
        let mv_option = if root_node && depth > 1 {
            thread.root_moves.next_move()
            //mpick.next_move(pos, false)
        } else {
            mpick.next_move(pos, false)
        };
        if mv_option.is_none() {
            break;
        }
        let mv = mv_option.unwrap();
        num_legal += 1;

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
        if root_node {
            thread.root_moves.insert(MoveValue::new(mv, alpha), depth);
        }
    }

    // If there are no legal moves at this point, it is either checkmate or stalemate
    if num_legal == 0 {
        // Stalemate
        if !pos.is_check() {
            return Value::DRAW;
        } else {
            return Value::mated_in(ply);
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
    let mut num_moves = 0;

    // Increment node counter
    if thread.ss.len() <= ply as usize {
        thread.ss.push(SearchStack::new());
    }
    thread.ss[ply as usize].node_count += 1;

    // Evaluate the position
    let mut eval = pos.evaluate();
    if eval >= beta {
        return beta;
    }
    if eval > alpha {
        alpha = eval;
    }

    let mut mpick = MovePicker::new();

    // Iterate through the moves
    while let Some(mv) = mpick.next_move(pos, true) {
        num_moves += 1;
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

    /*  If there are no moves at this point and we are in check, it is checkmate,
        since all evasions have been generated.
        If there are no moves and we are not in check, it is not necessarily stalemate,
        since not all moves are generated in qsearch.
    */
    if num_moves == 0 && pos.is_check() {
        return Value::mated_in(ply);
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
