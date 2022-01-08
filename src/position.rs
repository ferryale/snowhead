pub mod inline;
pub mod fen;
pub mod game;


use crate::attacks::attack_bb::*;
use crate::types::square::*;
use crate::types::piece::*;
use crate::types::r#move::*;
use crate::types::bitboard::*;
use crate::zobrist::*;
//use crate::rng;
//use self::inline::*;

#[derive(Debug, Clone, PartialEq)]
pub struct StateInfo {
    // Copied when making a move
    pub castling_right: CastlingRight,
    pub rule50: i32,
    pub plies_from_null: i32,
    //pub psq: Score,
    pub ep_square: Square,

    // Not copied when making a move (will be recomputed anyhow)
    pub key: Key,
    pub checkers_bb: Bitboard,
    pub captured_piece: Piece,
    pub blockers_for_king: [Bitboard; COLOR_NB],
    pub pinners: [Bitboard; COLOR_NB],
    pub check_squares: [Bitboard; PIECE_TYPE_NB],
}

impl StateInfo {
    pub fn new() -> StateInfo {
        StateInfo {
            castling_right: NO_CASTLING,
            rule50: 0,
            plies_from_null: 0,
            ep_square: Square::NONE,
            key: KEY_ZERO,
            checkers_bb: EMPTY_BB,
            captured_piece: NO_PIECE,
            blockers_for_king: [EMPTY_BB; COLOR_NB],
            pinners: [EMPTY_BB; COLOR_NB],
            check_squares: [EMPTY_BB; PIECE_TYPE_NB],
        }
    }

    pub fn copy(&self) -> StateInfo {
        StateInfo {
            // Copied
            castling_right: self.castling_right,
            rule50: self.rule50,
            plies_from_null: self.plies_from_null,
            //psq: self.psq,
            ep_square: self.ep_square,
            // Reset
            key: KEY_ZERO,
            checkers_bb: EMPTY_BB,
            captured_piece: NO_PIECE,
            blockers_for_king: [EMPTY_BB; COLOR_NB],
            pinners: [EMPTY_BB; COLOR_NB],
            check_squares: [EMPTY_BB; PIECE_TYPE_NB],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub zobrist: Zobrist,
    pub board: [Piece; SQUARE_NB],
    pub by_color_bb: [Bitboard; COLOR_NB],
    pub by_type_bb: [Bitboard; PIECE_TYPE_NB],
    pub piece_count: [i32; PIECE_NB],
    pub castling_rights_mask: [CastlingRight; SQUARE_NB],
    pub castling_rook_square: [Square; CASTLING_RIGHT_NB],
    pub castling_path: [Bitboard; CASTLING_RIGHT_NB],
    pub game_ply: i32,
    pub side_to_move: Color,
    pub states: Vec<StateInfo>,
    pub chess960: bool,
}


impl Position {
    pub fn new() -> Position {
        let mut pos = Position {
            zobrist: Zobrist::new(),
            board: [NO_PIECE; SQUARE_NB],
            by_color_bb: [EMPTY_BB; COLOR_NB],
            by_type_bb: [EMPTY_BB; PIECE_TYPE_NB],
            piece_count: [0; PIECE_NB],
            castling_rights_mask: [NO_CASTLING; SQUARE_NB],
            castling_rook_square: [Square::NONE; CASTLING_RIGHT_NB],
            castling_path: [EMPTY_BB; CASTLING_RIGHT_NB],
            game_ply: 0,
            side_to_move: WHITE,
            states: Vec::new(),
            chess960: false,
        };
        pos.init();
        pos
    }

    fn init(&mut self) {
        self.zobrist.init();
    }

    /// FIX ME: reverse iterator for File
    pub fn print(&self) {
        println!("\n +---+---+---+---+---+---+---+---+");
        for r_idx in (0..8).rev() {
            let r = Rank(r_idx as u32);
            for f in VALID_FILES {
                print!( " | {}", self.piece_on(Square::make(f, r)).to_char());
            }
            println!(" |\n +---+---+---+---+---+---+---+---+");
        }

        println!(
            "\nFen: {}\nKey: {}\nCheckers: {}\n{:?}\n",
            self.fen(),
            self.key(),
            self.checkers(),
            self.side_to_move()
        );
    }

    fn clear(&mut self) {

        self.board = [NO_PIECE; SQUARE_NB]; 
        self.by_color_bb = [EMPTY_BB; COLOR_NB];
        self.by_type_bb = [EMPTY_BB; PIECE_TYPE_NB];
        self.piece_count = [0; PIECE_NB];
            
        self.castling_rights_mask = [NO_CASTLING; SQUARE_NB];
        self.castling_rook_square = [Square::NONE; CASTLING_RIGHT_NB];
        self.castling_path = [EMPTY_BB; CASTLING_RIGHT_NB];
        self.game_ply = 0;
        self.side_to_move = WHITE;
        self.states = Vec::new();
        self.chess960 = false;

        self.init_states();

    }

    fn st(&self) -> &StateInfo {
        self.states.last().unwrap()
    }

    fn st_mut(&mut self) -> &mut StateInfo {
        self.states.last_mut().unwrap()
    }

    // set_state() computes the hash keys of the position, and other data
    // that once computed is updated incrementally as moves are made.
    // The function is used only when a new position is set up, and to verify
    // the correctness of the StateInfo data when running in debug mode.
    fn set_state(&mut self) {
        self.st_mut().key = KEY_ZERO;
        // TO DO: ADD PSQT
        // self.st_mut().psq = 0;

        self.st_mut().checkers_bb = self.attackers_to(self.square(self.side_to_move, KING))
            & self.pieces_c(self.opposite_side());

        self.set_check_info();

        for s in self.pieces() {
            let pc: Piece = self.piece_on(s);
            self.st_mut().key ^= self.zobrist.psq[pc][s];
        }

        if self.st().ep_square != Square::NONE {
            self.st_mut().key ^= self.zobrist.en_passant[self.st().ep_square.file()];
        }

        if self.side_to_move == BLACK {
            self.st_mut().key ^= self.zobrist.side;
        }

        self.st_mut().key ^= self.zobrist.castling[self.st().castling_right];
    }

    /// Position::set_check_info() sets king attacks to detect if a move gives check
    fn set_check_info(&mut self) {
        // self.st_mut().blockers_for_king[WHITE] = self.slider_blockers(self.pieces_c(BLACK), self.square(WHITE, KING), &mut self.pinners(BLACK));
        // self.st_mut().blockers_for_king[BLACK] = self.slider_blockers(self.pieces_c(WHITE), self.square(BLACK, KING), &mut self.pinners(WHITE));
        let mut pinners = EMPTY_BB;
        self.st_mut().blockers_for_king[WHITE] = self.slider_blockers(self.pieces_c(BLACK), self.square(WHITE, KING), &mut pinners);
        self.st_mut().pinners[WHITE] = pinners;

        self.st_mut().blockers_for_king[BLACK] = self.slider_blockers(self.pieces_c(WHITE), self.square(BLACK, KING), &mut pinners);
        self.st_mut().pinners[BLACK] = pinners;

        let ksq: Square = self.square(!self.side_to_move, KING);

        self.st_mut().check_squares[PAWN] = self.attacks_from_pawn(!self.side_to_move, ksq);
        self.st_mut().check_squares[KNIGHT] = pseudo_attacks(KNIGHT, ksq);
        self.st_mut().check_squares[BISHOP] = attacks_bb(BISHOP, ksq, self.pieces());
        self.st_mut().check_squares[ROOK] = attacks_bb(ROOK, ksq, self.pieces());
        self.st_mut().check_squares[QUEEN] = self.st().check_squares[BISHOP] | self.st().check_squares[ROOK];
        self.st_mut().check_squares[KING] = EMPTY_BB;
    }

    // // set_check_info() sets king attacks to detect if a move gives cehck

    // fn set_check_info(&mut self) {
    //     let mut pinners = Bitboard(0);
    //     self.st_mut().blockers_for_king[WHITE.0 as usize] =
    //         self.slider_blockers(self.pieces_c(BLACK),
    //             self.square(WHITE, KING), &mut pinners);
    //     self.st_mut().pinners[WHITE.0 as usize] = pinners;
    //     self.st_mut().blockers_for_king[BLACK.0 as usize] =
    //         self.slider_blockers(self.pieces_c(WHITE),
    //             self.square(BLACK, KING), &mut pinners);
    //     self.st_mut().pinners[BLACK.0 as usize] = pinners;

    //     let ksq = self.square(!self.side_to_move(), KING);

    //     self.st_mut().check_squares[PAWN.0 as usize] =
    //         self.attacks_from_pawn(!self.side_to_move, ksq);
    //     self.st_mut().check_squares[KNIGHT.0 as usize] =
    //         self.attacks_from(KNIGHT, ksq);
    //     self.st_mut().check_squares[BISHOP.0 as usize] =
    //     self.attacks_from(BISHOP, ksq);
    //     self.st_mut().check_squares[ROOK.0 as usize] =
    //         self.attacks_from(ROOK, ksq);
    //     self.st_mut().check_squares[QUEEN.0 as usize] =
    //         self.st().check_squares[BISHOP.0 as usize]
    //         | self.st().check_squares[ROOK.0 as usize];
    //     self.st_mut().check_squares[KING.0 as usize] = Bitboard(0);
    // }

    // slider_blockers() returns a bitboard of all the pieces (both colors)
    // that are blocking attacks on the square 's' from 'sliders'. A piece
    // blocks a slider if removing that piece from the board would result
    // in a position where square 's'is attacked. For example, a king attack
    // blocking piece can be either a pinned or a discovered check piece,
    // depending on whether its color is the opposite of or the same as the
    // color of the slider.

    pub fn slider_blockers(
        &self, sliders: Bitboard, s: Square, pinners: &mut Bitboard
    ) -> Bitboard {
        let mut blockers = Bitboard(0);
        *pinners = Bitboard(0);

        // Snipers are sliders that attack 's' when a piece is removed
        let snipers =
            ((pseudo_attacks(ROOK, s) & self.pieces_pp(QUEEN, ROOK))
                | (pseudo_attacks(BISHOP, s) & self.pieces_pp(QUEEN, BISHOP)))
            & sliders;

        let occupancy = self.pieces() ^ snipers;

        for sniper_sq in snipers {
            let b = between_bb(s, sniper_sq) & occupancy;

            if b != 0 && !more_than_one(b) {
                blockers |= b;
                if b & self.pieces_c(self.piece_on(s).color()) != 0 {
                    *pinners |= sniper_sq;
                }
            }
        }
        blockers
    }

    // set_castling_right() is a helper function used to set castling rights
    // given the corresponding color and the rook starting square.
    fn set_castling_right(&mut self, c: Color, rfrom: Square) {
        let kfrom = self.square(c, KING);
        let cs = if kfrom < rfrom {
            KING_SIDE
        } else {
            QUEEN_SIDE
        };
        let cr = castling_right_c(c, cs);

        self.st_mut().castling_right |= cr;
        self.castling_rights_mask[kfrom] |= cr;
        self.castling_rights_mask[rfrom] |= cr;
        self.castling_rook_square[cr] = rfrom;

        let kto = relative_square(
            c,
            if cs == KING_SIDE {
                Square::G1
            } else {
                Square::C1
            },
        );
        let rto = relative_square(
            c,
            if cs == KING_SIDE {
                Square::F1
            } else {
                Square::D1
            },
        );

        self.castling_path[cr] = (between_bb(rfrom, rto) | between_bb(kfrom, kto)) & 
                                 !(square_bb(kfrom) | square_bb(rfrom));
    }

    fn put_piece(&mut self, pc: Piece, s: Square) {
        self.board[s] = pc;
        self.by_type_bb[ALL_PIECES] |= s;
        self.by_type_bb[pc.piece_type()] |= s;
        self.by_color_bb[pc.color()] |= s;
        self.piece_count[pc] += 1;
        self.piece_count[Piece::make(pc.color(), ALL_PIECES)] += 1;
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        let pc: Piece = self.board[from];
        let from_to_bb = from.bb() | to.bb();
        self.by_type_bb[ALL_PIECES] ^= from_to_bb;
        self.by_type_bb[pc.piece_type()] ^= from_to_bb;
        self.by_color_bb[pc.color()] ^= from_to_bb;
        self.board[from] = NO_PIECE;
        self.board[to] = pc;
    }

    fn remove_piece(&mut self, s: Square) {
        let pc: Piece = self.board[s];
        self.by_type_bb[ALL_PIECES] ^= s;
        self.by_type_bb[pc.piece_type()] ^= s;
        self.by_color_bb[pc.color()] ^= s;
        self.board[s] = NO_PIECE;
        self.piece_count[pc] -= 1;
        self.piece_count[Piece::make(pc.color(), ALL_PIECES)] -= 1;
    }


}