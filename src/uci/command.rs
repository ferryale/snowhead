use super::option::UciOptions;
use crate::position::Position;
use cozy_chess::{Color, Move};
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub enum UciCommand {
    Uci,
    Debug,
    Display,
    IsReady,
    SetOption(UciOptions),
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
                "quit" => UciCommand::Quit,
                "d" => UciCommand::Display,
                "position" => Position::parse(cmd_args, &uci_options),
                "go" => GoOptions::parse(cmd_args),
                "setoption" => uci_options.parse(cmd_args)?,
                "stop" => UciCommand::Stop,
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
                    let fen_str = &args[1..moves_idx+1].join(" ");
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
                mv = Move::from_str(mv_str).unwrap();
                position.board.play(mv);
            }
        }

        UciCommand::Position(position)
    }
}
