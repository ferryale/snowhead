use super::UciCommand;
use crate::evaluate::psqt::{PIECE_VALUES, PSQ_BONUS};
use crate::evaluate::score::Phase;
use cozy_chess::{File, Piece, Rank};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

/*
    Uci options can be converted to and read from a json file
    via serde Serialize and Deserialize.
    This allows to pass material tables through uci for tuning with
    external optimizers.
*/
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UciOptions {
    pub hash_size: u32,
    pub move_overhead: u64,
    pub chess960: bool,
    pub psq_bonus: [[[[i16; Phase::NUM]; File::NUM]; Rank::NUM]; Piece::NUM],
    pub piece_values: [[i16; Phase::NUM]; Piece::NUM],
}

/* Uci options implementation */
impl Default for UciOptions {
    fn default() -> UciOptions {
        UciOptions {
            hash_size: 8,
            move_overhead: 10,
            chess960: false,
            psq_bonus: PSQ_BONUS,
            piece_values: PIECE_VALUES,
        }
    }
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

    // Creates a uci options struct from a json file input
    pub fn from_file(filename: &str) -> io::Result<UciOptions> {
        if let Ok(f) = fs::File::open(filename) {
            let options: UciOptions = serde_json::from_reader(&f)?;
            Ok(options)
        } else {
            println!("Filename '{filename}' does not exist. UCI options set to default.");
            Ok(UciOptions::new())
        }
    }

    // Loads uci options from json file
    pub fn load(&mut self, filename: &str) -> io::Result<()> {
        *self = UciOptions::from_file(filename)?;
        Ok(())
    }

    // Dumps uci options to json file
    pub fn dump(&self, filename: &str) -> io::Result<()> {
        let f = fs::File::create(filename)?;
        serde_json::to_writer(&f, &self)?;
        Ok(())
    }

    // Parses uci options from a vector of strings
    pub fn parse(&mut self, args: Vec<&str>) -> io::Result<UciCommand> {
        // Option args: ["name", opt_name, "value", opt_value]
        if args[0] != "name" || args[2] != "value" || args.len() != 4 {
            println!("Invalid option arguments: {}", args.join(" "));
            return Ok(UciCommand::SetOption(*self));
        }

        // Assign values to valid options
        let (key, value) = (args[1], args[3]);
        match key {
            "Hash" => self.hash_size = value.parse().unwrap(),
            "ClearHash" => {}
            "MoveOverhead" => self.move_overhead = value.parse().unwrap(),
            "chess960" => self.chess960 = value.parse::<bool>().unwrap(),
            "load" => self.load(value)?,
            "dump" => self.dump(value)?,
            _ => println!("Invalid option '{key}'"),
        } // match

        Ok(UciCommand::SetOption(*self))
    }
}
