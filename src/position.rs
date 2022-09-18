use self::builder::PosBuilder;
use crate::evaluate::score::{Score, Value};
use crate::evaluate::Evaluator;
use cozy_chess::{Board, Color, GameStatus, Move, Piece, Rank, Square};

pub mod builder;

/*
    Position struct.
    The board stack is stored to undo moves by popping
    the last board from the stack.
    Undo info and logic not needed.
*/
#[derive(Debug, Clone)]
pub struct Position {
    board: Board,
    board_stack: Vec<Board>,
    evaluator: Evaluator,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            board: Board::default(),
            board_stack: vec![],
            evaluator: Evaluator::default(),
        }
    }
}

impl Position {
    /*
        Position is created by a builder:
        let pos = Position::new().fen(&fen).uci_options(&uci_options).build();
    */
    pub fn new() -> PosBuilder {
        PosBuilder::default()
    }

    // Returns a reference to internal board
    pub fn board(&self) -> &Board {
        &self.board
    }

    // Returns a reference to internal evaluator
    pub fn evaluator(&self) -> &Evaluator {
        &self.evaluator
    }

    // Helper method to read a board from fen: returns a Board!
    fn board_from_fen(fen: &str, chess960: bool) -> Board {
        match Board::from_fen(fen, chess960) {
            Ok(board) => board,
            Err(e) => {
                println!("Error '{e:?}': Invalid fen '{fen}'. Position set to starting position.");
                Board::default()
            }
        }
    }

    // Makes a move and incrementally updates the evaluator
    pub fn do_move(&mut self, mv: Move) {
        self.board_stack.push(self.board.clone());
        self.evaluator.do_move(&self.board, mv);
        self.board.play_unchecked(mv);
    }

    // Unmakes a move by popping last move from the stack
    pub fn undo_move(&mut self) {
        self.board = self.board_stack.pop().unwrap();
        self.evaluator.undo_move();
    }

    // Tries to make a null move and returns true if succeeds
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

    // Evaluates the position through the evaluator
    pub fn evaluate(&self) -> Value {
        self.evaluator.evaluate(&self.board)
    }

    // Returns the psq score
    pub fn psq(&self) -> Score {
        self.evaluator.psq()
    }

    // Computes and returns the psq score
    pub fn compute_psq(&mut self) -> Score {
        self.init_psq();
        self.psq()
    }

    // Inits the psq score
    pub fn init_psq(&mut self) {
        self.evaluator.compute_psq(&self.board);
    }

    // Checks if a move is castling
    pub fn is_castling(&self, mv: Move) -> bool {
        let c = self.board.side_to_move();
        self.board.colors(c).has(mv.to)
    }

    // Checks if a move is enpassant
    pub fn is_enpassant(&self, mv: Move) -> bool {
        let c = self.board.side_to_move();
        let ep_square = self
            .board
            .en_passant()
            .map(|ep| Square::new(ep, Rank::Sixth.relative_to(c)));
        Some(mv.to) == ep_square
    }

    // Returns the side to move
    pub fn side_to_move(&self) -> Color {
        self.board.side_to_move()
    }

    // Returns the piece on a given square, None if square is empty.
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        self.board.piece_on(sq)
    }

    // Counts the number of pieces (both black and white) of a give type
    pub fn count(&self, pc: Piece) -> u32 {
        self.board.pieces(pc).len()
    }

    // Checks if the position is drawn: 50 move rule, 3 fold repetition, insufficient material
    pub fn is_draw(&self, ply: u32) -> bool {
        self.insufficient_material()
            || self.has_repeated(ply)
            || (self.board.halfmove_clock() >= 100
                && (self.board.checkers().is_empty() || self.board.status() != GameStatus::Won))
    }

    // Checks is the current position is a check
    pub fn is_check(&self) -> bool {
        !self.board.checkers().is_empty()
    }

    // Checks is the position is quiet
    pub fn is_quiet(&self) -> bool {
        if self.is_check() {
            return false;
        }

        // Check that there are no captures
        let mut num_captures = 0;
        self.board.generate_moves(|mut piece_moves| {
            piece_moves.to &= self.board.colors(!self.side_to_move());
            num_captures += piece_moves.len();
            false
        });

        num_captures == 0
    }

    /*
        Checks for 3fold repetition
        Implemetation from Black Marlin:
        https://github.com/dsekercioglu/blackmarlin
    */
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

    /*
        Checks for insufficient material
        Implemetation from Black Marlin:
        https://github.com/dsekercioglu/blackmarlin
    */
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
