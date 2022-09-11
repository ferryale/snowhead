use super::Position;
use crate::evaluate::Evaluator;
use crate::uci::option::UciOptions;

pub const STARTFEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/* Builder for position */
#[derive(Debug)]
pub struct PosBuilder {
    pub fen: Option<String>,
    pub uci_options: Option<UciOptions>,
}

/* Default builder with all fields set to None */
impl Default for PosBuilder {
    fn default() -> PosBuilder {
        PosBuilder {
            fen: None,
            uci_options: None,
        }
    }
}

/* PosBuilder implementation */
impl PosBuilder {
    pub fn fen(&mut self, fen: &str) -> &mut Self {
        self.fen = Some(String::from(fen));
        self
    }
    pub fn uci_options(&mut self, uci_options: &UciOptions) -> &mut Self {
        self.uci_options = Some(*uci_options);
        self
    }

    pub fn build(&mut self) -> Position {
        Position {
            board: Position::board_from_fen(
                &self.fen.as_ref().unwrap_or(&STARTFEN.to_string()),
                self.uci_options.unwrap_or_default().chess960,
            ),
            board_stack: vec![],
            evaluator: Evaluator::new(
                &self.uci_options.unwrap_or_default().piece_values,
                &self.uci_options.unwrap_or_default().psq_bonus,
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Position;
    use crate::uci::option::UciOptions;

    #[test]
    fn pos_builder() {
        let fen = "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1";
        let uci_options = UciOptions::default();
        let pos = Position::new().fen(fen).build();
        assert_eq!(
            pos.board,
            Position::board_from_fen(&fen, uci_options.chess960)
        );
    }
}
