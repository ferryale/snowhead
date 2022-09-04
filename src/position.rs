use crate::evaluate::score::{Value, Score};
use crate::evaluate::Evaluator;
use crate::uci::option::UciOptions;
use cozy_chess::{Color, Board, Move};

#[derive(Debug, Clone)]
pub struct Position {
    pub board: Board,
    board_stack: Vec<Board>,
    pub evaluator: Evaluator,
}

impl Position {
    pub fn new(fen: &str, uci_options: &UciOptions) -> Position {
        Position {
            board: Self::from_fen(fen, uci_options.chess960),
            board_stack: vec![],
            evaluator: Evaluator::new(&uci_options.piece_values, &uci_options.psq_bonus),
        }
    }

    pub fn default(uci_options: &UciOptions) -> Position {
        Position {
            board: Board::default(),
            board_stack: vec![],
            evaluator: Evaluator::new(&uci_options.piece_values, &uci_options.psq_bonus),
        }
    }

    pub fn default_uci() -> Position {
        Position {
            board: Board::default(),
            board_stack: vec![],
            evaluator: Evaluator::default(),
        }
    }

    pub fn new_uci(fen: &str) -> Position {
        Position {
            board: Self::from_fen(fen, false),
            board_stack: vec![],
            evaluator: Evaluator::default(),
        }
    }

    pub fn set(fen: &str, chess960: bool) -> Position {
        Position {
            board: Self::from_fen(fen, chess960),
            board_stack: vec![],
            evaluator: Evaluator::default(),
        }
    }

    fn from_fen(fen: &str, chess960: bool) -> Board {
        match Board::from_fen(fen, chess960) {
            Ok(board) => board,
            Err(e) => {
                println!("Error '{e:?}': Invalid fen '{fen}'. Position set to starting position.");
                Board::default()
            }
        }
    }

    pub fn do_move(&mut self, mv: Move) {
        let board = self.board.clone();
        self.board.play_unchecked(mv);
        self.evaluator.do_move(&board, mv, &self.board);
        self.board_stack.push(board);
    }

    pub fn undo_move(&mut self) {
        self.board = self.board_stack.pop().unwrap();
        self.evaluator.undo_move();
    }

    pub fn evaluate(&self) -> Value {
        self.evaluator.evaluate(&self.board).values[0]
    }

    pub fn eval(&self) -> Value {
        if self.board.side_to_move() == Color::White {
            self.psq().values[0]
        } else {
            -self.psq().values[0]
        }
        
    }

    pub fn psq(&self) -> Score {
        self.evaluator.psq
    }

    pub fn eval_psq(&mut self) -> Score {
        self.evaluator.eval_psq(&self.board);
        self.psq()
    }

    pub fn init_psq(&mut self) {
        self.evaluator.eval_psq(&self.board);
    }
}
