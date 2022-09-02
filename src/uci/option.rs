use super::UciCommand;
use crate::evaluate::psqt::{PIECE_VALUES, PSQ_BONUS};
use crate::evaluate::score::Phase;
use cozy_chess::{File, Piece, Rank};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

//use crate::evaluate::{Score,Phase,SqTable,PieceTable,PsqTable,Evaluator, PSQ_BONUS, PIECE_VALUES};
// use crate::position::{Position};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UciOptions {
    pub hash_size: u32,
    pub move_overhead: u64,
    pub chess960: bool,
    pub psq_bonus: [[[[i16; Phase::NUM]; File::NUM]; Rank::NUM]; Piece::NUM],
    pub piece_values: [[i16; Phase::NUM]; Piece::NUM],
}

impl UciOptions {
    pub fn new() -> UciOptions {
        UciOptions {
            hash_size: 8,
            move_overhead: 10,
            chess960: false,
            psq_bonus: PSQ_BONUS,
            piece_values: PIECE_VALUES,
        }
    }

    pub fn from_file(filename: &str) -> io::Result<UciOptions> {
        if let Ok(f) = fs::File::open(filename) {
            let options: UciOptions = serde_json::from_reader(&f)?;
            Ok(options)
        } else {
            println!("Filename '{filename}' does not exist. UCI options set to default.");
            Ok(UciOptions::new())
        }
    }

    pub fn load(&mut self, filename: &str) -> io::Result<()> {
        *self = UciOptions::from_file(filename)?;
        Ok(())
    }

    pub fn dump(&self, filename: &str) -> io::Result<()> {
        let f = fs::File::create(filename)?;
        serde_json::to_writer(&f, &self)?;
        Ok(())
    }

    pub fn parse(&mut self, args: Vec<&str>) -> io::Result<UciCommand> {
        let args_map = args
            .chunks_exact(2) // chunks_exact returns an iterator of slices
            .map(|chunk| (chunk[0], chunk[1])) // map slices to tuples
            .collect::<HashMap<_, _>>(); // collect into a hashmap

        for (key, value) in args_map {
            match key {
                "Hash" => self.hash_size = value.parse().unwrap(),
                "ClearHash" => {}
                "MoveOverhead" => self.move_overhead = value.parse().unwrap(),
                "chess960" => self.chess960 = value.parse::<bool>().unwrap(),
                "load" => self.load(value)?,
                "dump" => self.dump(value)?,
                _ => println!("Invalid option '{key}'"),
            }
        }

        Ok(UciCommand::SetOption(*self))
    }
}
