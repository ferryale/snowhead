use crate::bench::epd::EpdEntry;
use crate::evaluate::score::Value;
use crate::position::Position;
use crate::uci::option::UciOptions;
use std::fs;

#[derive(Debug)]
pub struct TextelEntry {
    pub fen: String,
    result: Option<f32>,
}

#[derive(Debug)]
pub struct TextelBatch {
    entries: Vec<TextelEntry>,
    k: f32,
}

impl TextelEntry {
    pub fn new(line: &str) -> TextelEntry {
        let epd_entry = EpdEntry::new(line);
        TextelEntry {
            fen: epd_entry.fen,
            result: Self::parse_result(&epd_entry.c0),
        }
    }

    fn parse_result(result: &str) -> Option<f32> {
        match result {
            "1-0" => Some(1.0),
            "1/2-1/2" => Some(0.5),
            "0-1" => Some(0.0),
            _ => None,
        }
    }
}

// Display implementation for TextelEntry
impl core::fmt::Display for TextelEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.result {
            Some(result) => write!(f, "{}; {}", self.fen, result)?,
            None => {}
        }
        Ok(())
    }
}

impl TextelBatch {
    pub fn new(filename: &str, k: f32, quiet: bool) -> TextelBatch {
        let contents = fs::read_to_string(filename);
        let mut entries = vec![];

        for line in contents.expect("File '{filename}' not found").split("\n") {
            let entry = TextelEntry::new(line);
            let mut pos = Position::new().fen(&entry.fen).build();
            if quiet && pos.is_quiet() {
                entries.push(entry)
            }
        }
        TextelBatch {
            entries: entries,
            k: k,
        }
    }

    pub fn evaluate(&self, uci_options: &UciOptions) -> f32 {
        let n = self.entries.len() as f32;
        let mut sum = 0.0;
        for entry in &self.entries {
            if entry.result.is_none() {
                continue;
            }
            let mut pos = Position::new()
                .fen(&entry.fen)
                .uci_options(&uci_options)
                .build();
            pos.init_psq();
            let sig = self.sigmoid(pos.evaluate());
            let result = entry.result.unwrap();
            let sqres = f32::powf(result - sig, 2.0);
            sum += sqres;
        }

        return sum / n;
    }

    fn sigmoid(&self, value: Value) -> f32 {
        let v = value.0 as f32;
        return 1.0 / (1.0 + f32::powf(10.0, -self.k * v / 400.0));
    }
}

// Display implementation for TextelBatch
impl core::fmt::Display for TextelBatch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut cnt = 0;
        for entry in &self.entries {
            write!(f, "{}\n", entry)?;
            cnt += 1;
            if cnt > 20 {
                break;
            }
        }
        Ok(())
    }
}
