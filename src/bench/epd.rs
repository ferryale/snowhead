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
