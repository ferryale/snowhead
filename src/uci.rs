use crate::movegen::{ExtMove, generate_legal};
use crate::position::Position;
use crate::search::Thread;
use crate::perft::perft;

use crate::types::r#move::Move;
use crate::types::score::{Depth};


use std;
use std::env;
// use std::sync::{Arc, RwLock};
// use std::time::Instant;

// FEN string of the initial position, normal chess
const START_FEN: &'static str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// position() is called when engine receives the "position" UCI command.
// The function sets up the position described in the given FEN string ("fen")
// or the starting position ("startpos") and then makes the moves given in the
// following move list ("moves").

fn position(pos: &mut Position, args: &str) {
    let fen: &str;

    let moves = match args.find("moves") {
        Some(idx) => idx,
        None => args.len(),
    };

    if &args[0..8] == "startpos" {
        fen = START_FEN;
    } else if &args[0..3] == "fen" {
        fen = (&args[3..moves]).trim();
    } else {
        return;
    }

    pos.init_states();
    pos.set(fen, pos.is_chess960());
    // pos_data.fen = String::from(fen);
    // pos_data.moves = Vec::new();

    if moves == args.len() {
        return;
    }

    // Parse move list
    let moves = &args[moves+5..].trim();
    let iter = moves.split_whitespace();
    for token in iter {
        let m = Move::from_string(pos, token);
        if m == Move::NONE {
            break;
        }
        pos.do_move(m);
    }
}

// go() is called when engine receives the "go" UCI command. The function
// sets the thinking time and other parameters from the input string, then
// starts the search.

fn go(pos: &mut Position, args: &str) {

    let mut do_perft = false;
    let mut depth = 0;
    let mut iter = args.split_whitespace();
    while let Some(token) = iter.next() {
        match token {
            "depth" => depth = iter.next().unwrap().parse().unwrap(),
            "perft" => { 
                depth = iter.next().unwrap().parse().unwrap();
                do_perft = true;
            },
            _ => {}
        }
    }

    if do_perft {
        let nodes = perft::<true>(pos, Depth(depth));
        println!("Total nodes seached: {}", nodes);
    } else {
        let mut thread = Thread::new();
        thread.search(pos, Depth(depth));
    }

    
}

// cmd_loop() waits for a command from stdin, parses it and calls the
// appropriate function. Also intercepts EOF from stdin to ensure a
// graceful exit if the GUI dies unexpectedly. When called with some comand
// line arguments, e.g. to run 'bench', once the command is executed the
// function returns immediately. In addition to the UCI ones, some additional
// debug commands are supported.

pub fn cmd_loop() {
    let mut pos = Box::new(Position::new());

    pos.init_states();
    pos.set(START_FEN, false);

    let mut cmd = String::new();
    for arg in env::args().skip(1) {
        cmd.push_str(&arg);
        cmd.push(' ');
    }

    loop {
        if env::args().len() == 1 {
            cmd = String::new();
            // Block here waiting for input or EOF
            if let Err(_) = std::io::stdin().read_line(&mut cmd) {
                cmd = String::from("quit");
            }
        }
        let cmd_slice = cmd.trim();
        let (token, args) =
            if let Some(idx) = cmd_slice.find(char::is_whitespace) {
                cmd_slice.split_at(idx)
            } else {
                (cmd_slice, "")
            };
        let args = args.trim();

        // The GUI sends 'ponderhit' to tell us the user has played the
        // expected move. So 'ponderhit' will be sent if we were told to
        // ponder on the same move the user has played. We should continue
        // searching but switch from pondering to normal search. In case
        // threads::stop_on_ponderhit() is true, we are waiting for
        // 'ponderhit' to stop the search, for instance if max search depth
        // has been reached.
        match token {
            "ucinewgame" | "quit" | "stop" => {},
            "uci" => {
                println!("id name Snowhead v0.1");
                println!("uciok");
            }
            "go" => go(&mut pos, args),
            "position" =>
                position(&mut pos, args),
            
            "isready" => println!("readyok"),

            // Additional custom non-UCI commands
            "d" => pos.print(),
            _ => println!("Unknown command: {} {}", cmd, args)
        }
        if env::args().len() > 1 || token == "quit" {
            // Command-line args are one-shot
            break;
        }
    }
}

impl Move {

    pub fn from_string(pos: &Position, s: &str) -> Move {
        if s.len() == 5 {
        }

        let mut list = [ExtMove {m: Move::NONE, value: 0}; 200];

        let _num_moves = generate_legal(&pos, &mut list, 0);

        for ext_move in list {
            let m = ext_move.m;
            if s == m.to_string(pos.is_chess960()) {
                return m;
            }
        }

        Move::NONE
    }

}


