use super::option::UciOptions;
use crate::position::Position;
use cozy_chess::{Board, Color, File, Move, Piece, Square};
use std::collections::HashMap;
use std::io;

// http://wbec-ridderkerk.nl/html/UCIProtocol.html
#[derive(Debug)]
pub enum UciCommand {
    Uci,
    Debug,
    Display,
    IsReady,
    SetOption(UciOptions),
    UciNewGame,
    Position(Position),
    Go(GoOptions),
    Stop,
    Ponderhit,
    Quit,
    Invalid(String),
}

/* Uci command implementation */
impl UciCommand {
    // Parse an uci command string and return an enum result
    pub fn parse(uci_options: &mut UciOptions) -> io::Result<UciCommand> {
        // Read a line from the command line into string cmd
        let reader = std::io::stdin();
        let mut cmd = String::new();
        reader.read_line(&mut cmd)?;

        // Split the string into a vactor of tokens
        let tokens: Vec<&str> = cmd.split_whitespace().collect();

        // First token is comand, other tokens are arguments
        let uci_command = if tokens.len() > 0 {
            let cmd_string = tokens[0];
            let cmd_args = tokens[1..].to_vec();

            // Parse cmd_string into corresponding uci_command
            match cmd_string {
                "uci" => UciCommand::Uci,
                "debug" => UciCommand::Debug,
                "d" => UciCommand::Display,
                "isready" => UciCommand::IsReady,
                "setoption" => uci_options.parse(cmd_args)?,
                "ucinewgame" => UciCommand::UciNewGame,
                "position" => Position::parse(cmd_args, &uci_options),
                "go" => GoOptions::parse(cmd_args),
                "stop" => UciCommand::Stop,
                "ponderhit" => UciCommand::Ponderhit,
                "quit" => UciCommand::Quit,
                _ => UciCommand::Invalid(String::from(cmd_string)),
            } // match
        } else {
            UciCommand::Invalid(String::new())
        }; // if-else

        Ok(uci_command)
    }
}

/* Options for go command */
#[derive(Debug, Clone, Copy)]
pub struct GoOptions {
    pub time: [u64; Color::NUM],
    pub inc: [u64; Color::NUM],
    pub movestogo: u64,
    pub depth: u32,
    pub perft: u32,
    pub nodes: u64,
    pub mate: u32,
    pub movetime: u64,
    pub infinite: bool,
}

/* Go options implmentation */
impl GoOptions {
    pub fn new() -> GoOptions {
        GoOptions {
            time: [0; Color::NUM],
            inc: [0; Color::NUM],
            movestogo: 0,
            depth: 0,
            perft: 0,
            nodes: 0,
            mate: 0,
            movetime: 0,
            infinite: false,
        }
    }

    /*
    Input: vector of string arguments for go command
    Output: go command enum with associated go_options struct.
    */
    fn parse(args: Vec<&str>) -> UciCommand {
        let args_map = args
            .chunks_exact(2) // chunks_exact returns an iterator of slices
            .map(|chunk| (chunk[0], chunk[1])) // map slices to tuples
            .collect::<HashMap<_, _>>(); // collect into a hashmap

        let mut options = GoOptions::new();

        for (key, value) in args_map {
            match key {
                "wtime" => options.time[Color::White as usize] = value.parse().unwrap(),
                "btime" => options.time[Color::Black as usize] = value.parse().unwrap(),
                "winc" => options.inc[Color::White as usize] = value.parse().unwrap(),
                "binc" => options.inc[Color::Black as usize] = value.parse().unwrap(),
                "movestogo" => options.movestogo = value.parse().unwrap(),
                "depth" => options.depth = value.parse().unwrap(),
                "perft" => options.perft = value.parse().unwrap(),
                "nodes" => options.nodes = value.parse().unwrap(),
                "mate" => options.mate = value.parse().unwrap(),
                "movetime" => options.movetime = value.parse().unwrap(),
                "infinite" => options.infinite = true,
                _ => println!("Unknown option '{key}'"),
            } // match
        } // for

        UciCommand::Go(options)
    }
}

/* Position implementation */
impl Position {
    /*
    Input: vector of string arguments for position command
    Output: position command enum with associated position struct.
    */
    fn parse(args: Vec<&str>, uci_options: &UciOptions) -> UciCommand {
        // Find the position of "moves" in args
        let moves_pos = args.iter().position(|&r| r == "moves");

        // Possible args: 1) startpos; 2) fen+fen_str+moves+moves_list
        let mut position = match args[0] {
            // Startpos: return default position
            "startpos" => Position::new().uci_options(&uci_options).build(),
            // Fen
            "fen" => {
                // If moves keyword if found, the fen string precedes "moves"
                if let Some(moves_idx) = moves_pos {
                    let fen_str = &args[1..moves_idx + 1].join(" ");
                    Position::new()
                        .fen(&fen_str)
                        .uci_options(&uci_options)
                        .build()
                // If moves keyword not found, args only contains the fen string
                } else {
                    let fen_str = &args[1..].join(" ");
                    Position::new()
                        .fen(&fen_str)
                        .uci_options(&uci_options)
                        .build()
                } // if-else
            } // fen
            // Any other keyword violates the uci protocol: return defaul position.
            _ => Position::new().uci_options(&uci_options).build(),
        }; // match

        // Parse the list of moves
        let mut mv: Move;
        // If the "moves" keyword is found
        if let Some(moves_idx) = moves_pos {
            // Moves list start after moves_pos
            for mv_str in &args[moves_idx + 1..] {
                // Convert move string into Move
                mv = mv_str.parse().unwrap();
                // This is extra convertion is needed because cozy-chess
                // encodes castling as king capture rook, which is not uci standard.
                convert_move(&mut mv, position.board(), uci_options.chess960);
                // Play the move on the board
                position.do_move(mv);
            }
        }

        UciCommand::Position(position)
    }
}

/*
    Convert internal cozy-chess format to standard uci.
    Implementation from Black Marlin
    https://github.com/dsekercioglu/blackmarlin
*/
// fn convert_move_to_uci(mv: &mut Move, board: &Board, chess960: bool) {
//     if !chess960 && board.color_on(mv.from) == board.color_on(mv.to) {
//         let rights = board.castle_rights(board.side_to_move());
//         let file = if Some(mv.to.file()) == rights.short {
//             File::G
//         } else {
//             File::C
//         };
//         mv.to = Square::new(file, mv.to.rank());
//     }
// }

fn convert_move(mv: &mut Move, board: &Board, chess960: bool) {
    let convert_castle = !chess960
        && board.piece_on(mv.from) == Some(Piece::King)
        && mv.from.file() == File::E
        && matches!(mv.to.file(), File::C | File::G);
    if convert_castle {
        let file = if mv.to.file() == File::C {
            File::A
        } else {
            File::H
        };
        mv.to = Square::new(file, mv.to.rank());
    }
}
