use super::option::UciOptions;
use crate::position::Position;
use cozy_chess::{Color, Move, Square, Board, File, Piece};
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
// http://wbec-ridderkerk.nl/html/UCIProtocol.html
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

impl UciCommand {
    pub fn parse(uci_options: &mut UciOptions) -> io::Result<UciCommand> {
        let reader = std::io::stdin();
        let mut cmd = String::new();
        reader.read_line(&mut cmd)?;
        let tokens: Vec<&str> = cmd.split_whitespace().collect();

        let uci_command = if tokens.len() > 0 {
            let cmd_string = tokens[0];
            let cmd_args = tokens[1..].to_vec();

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
            }
        } else {
            UciCommand::Invalid(String::new())
        };

        Ok(uci_command)
    }
}

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
            }
        }

        UciCommand::Go(options)
    }
}

impl Position {
    fn parse(args: Vec<&str>, uci_options: &UciOptions) -> UciCommand {
        // Find the position of "moves" in args
        let moves_pos = args.iter().position(|&r| r == "moves");

        let mut position = match args[0] {
            "startpos" => Position::default(&uci_options),
            "fen" => {
                if let Some(moves_idx) = moves_pos {
                    let fen_str = &args[1..moves_idx + 1].join(" ");
                    Position::new(&fen_str, &uci_options)
                } else {
                    let fen_str = &args[1..].join(" ");
                    Position::new(&fen_str, &uci_options)
                }
            }
            _ => Position::default(&uci_options),
        };

        let mut mv: Move;
        if let Some(moves_idx) = moves_pos {
            for mv_str in &args[moves_idx + 1..] {
                //mv = Move::from_str(mv_str).unwrap();
                mv = mv_str.parse().unwrap();
                convert_move(&mut mv, &position.board, uci_options.chess960);
                position.board.play(mv);
            }
        }

        UciCommand::Position(position)
    }
}

pub fn convert_move_to_uci(make_move: &mut Move, board: &Board, chess960: bool) {
    if !chess960 && board.color_on(make_move.from) == board.color_on(make_move.to) {
        let rights = board.castle_rights(board.side_to_move());
        let file = if Some(make_move.to.file()) == rights.short {
            File::G
        } else {
            File::C
        };
        make_move.to = Square::new(file, make_move.to.rank());
    }
}

fn convert_move(make_move: &mut Move, board: &Board, chess960: bool) {
    let convert_castle = !chess960
        && board.piece_on(make_move.from) == Some(Piece::King)
        && make_move.from.file() == File::E
        && matches!(make_move.to.file(), File::C | File::G);
    if convert_castle {
        let file = if make_move.to.file() == File::C {
            File::A
        } else {
            File::H
        };
        make_move.to = Square::new(file, make_move.to.rank());
    }
}
