#[derive(Debug)]
pub struct EpdEntry {
    pub fen: String,
    acd: u32,
    acn: u32,
    c0: String,
}

impl EpdEntry {
    pub fn new(line: &str) -> EpdEntry {
        let split = line.split(";");
        let keywords = ["acd", "acn", "c0"];
        let mut entry = EpdEntry {
            fen: String::new(),
            acd: 0,
            acn: 0,
            c0: String::new(),
        };
        for (idx, el) in split.enumerate() {
            if idx == 0 {
                entry.fen = String::from(el);
                continue;
            }
            for key in keywords {
                if el.contains(key) {
                    let pos = el.find(key).unwrap();
                    let val = el[pos + key.len()..].trim();
                    match key {
                        "acd" => entry.acd = val.parse::<u32>().unwrap(),
                        "acn" => entry.acn = val.parse::<u32>().unwrap(),
                        "c0" => entry.c0 = String::from(val.replace('"', "")),
                        _ => {}
                    }
                }
            }
        }
        entry
    }
}

#[cfg(test)]
mod tests {
    use super::EpdEntry;
    use crate::position::Position;
    use cozy_chess::{Color, Move, Piece, Square};
    use std::fs;

    #[test]
    fn read_epd_file() {
        let contents = fs::read_to_string("./tuner/data/textel_batch1_10000_games_2moves_v2.epd");

        for line in contents
            .expect("File 'textel_batch1_10000_games_2moves_v2.epd' not found")
            .split("\n")
        {
            let epd_entry = EpdEntry::new(line);
            let mut pos = Position::new().fen(&epd_entry.fen).build();
            pos.init_psq();
        }
    }
}
