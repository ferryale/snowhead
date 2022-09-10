use crate::evaluate::score::{Score, Value};
use crate::evaluate::Evaluator;
use crate::uci::option::UciOptions;
use cozy_chess::{Board, Color, GameStatus, Move, Piece, Rank, Square};

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
        self.board_stack.push(self.board.clone());
        self.evaluator.do_move(&self.board, mv);
        self.board.play_unchecked(mv);
    }

    pub fn undo_move(&mut self) {
        self.board = self.board_stack.pop().unwrap();
        self.evaluator.undo_move();
    }

    pub fn do_null_move(&mut self) -> bool {
        if let Some(new_board) = self.board.null_move() {
            self.board_stack.push(self.board.clone());
            self.evaluator.do_null_move();
            self.board = new_board;
            true
        } else {
            false
        }
    }

    pub fn evaluate(&self) -> Value {
        self.evaluator.evaluate(&self.board)
        // if self.board.side_to_move() == Color::White {
        //     self.psq().values[0]
        // } else {
        //     -self.psq().values[0]
        // }
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

    pub fn is_castling(&self, mv: Move) -> bool {
        let c = self.board.side_to_move();
        self.board.colors(c).has(mv.to)
    }

    pub fn is_enpassant(&self, mv: Move) -> bool {
        let c = self.board.side_to_move();
        let ep_square = self
            .board
            .en_passant()
            .map(|ep| Square::new(ep, Rank::Sixth.relative_to(c)));
        Some(mv.to) == ep_square
    }

    pub fn captured_square(&self, mv: Move) -> Square {
        let c = self.board.side_to_move();
        if self.is_enpassant(mv) {
            Square::new(mv.to.file(), Rank::Fifth.relative_to(c))
        } else {
            mv.to
        }
    }

    pub fn captured_piece(&self, mv: Move) -> Option<Piece> {
        self.board.piece_on(self.captured_square(mv))
    }

    pub fn side_to_move(&self) -> Color {
        self.board.side_to_move()
    }

    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        self.board.piece_on(sq)
    }

    pub fn count(&self, pc: Piece) -> u32 {
        self.board.pieces(pc).len()
    }

    pub fn is_draw(&self, ply: u32) -> bool {
        self.insufficient_material()
            || self.has_repeated(ply)
            || (self.board.halfmove_clock() >= 100
                && (self.board.checkers().is_empty() || self.board.status() != GameStatus::Won))
    }

    pub fn is_check(&self) -> bool {
        !self.board.checkers().is_empty()
    }

    fn has_repeated(&self, ply: u32) -> bool {
        let hash = self.board.hash();
        self.board_stack
            .iter()
            .rev()
            .skip(1)
            .take(ply as usize)
            .any(|board| board.hash() == hash)
            || self
                .board_stack
                .iter()
                .rev()
                .skip(ply as usize + 1)
                .filter(|board| board.hash() == hash)
                .count()
                >= 2
    }

    fn insufficient_material(&self) -> bool {
        let rooks = self.board.pieces(Piece::Rook);
        let queens = self.board.pieces(Piece::Queen);
        let pawns = self.board.pieces(Piece::Pawn);
        match self.board.occupied().len() {
            2 => true,
            3 => (rooks | queens | pawns).is_empty(),
            _ => false,
        }
    }
}
