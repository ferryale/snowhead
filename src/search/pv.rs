use crate::movegen::MAX_MOVES;
use cozy_chess::Move;

/*
    PrincipalVariation stores the pv
    in a list of moves
*/
#[derive(Debug, Clone, Copy)]
pub struct PrincipalVariation {
    pub moves: [Option<Move>; MAX_MOVES as usize],
}

/* Principal variation implementation */
impl PrincipalVariation {
    // Constructor
    pub fn new() -> PrincipalVariation {
        PrincipalVariation {
            moves: [None; MAX_MOVES as usize],
        }
    }

    // Update: takes a child pv and inserts new best move at the beginning
    pub fn update(&mut self, mv: &Move, child_pv: &PrincipalVariation) {
        self.moves[0] = Some(*mv);
        for idx in 0..MAX_MOVES as usize {
            if let Some(child_mv) = child_pv.moves[idx] {
                self.moves[idx + 1] = Some(child_mv);
            } else {
                break;
            }
        }
    }

    // Computes the length of the pv
    pub fn len(&self) -> usize {
        let mut len = 0;
        for mv in self.moves {
            if mv.is_none() {
                break;
            }
            len += 1;
        }
        len
    }
}

// Display implementation for PrincipalVariation
impl core::fmt::Display for PrincipalVariation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for mv_option in self.moves {
            match mv_option {
                Some(mv) => write!(f, "{} ", mv)?,
                None => break,
            }
        }
        Ok(())
    }
}
