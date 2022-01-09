use super::*;
use crate::attacks::attack_bb::*;

impl Position {   

    pub fn init_states(&mut self) {
        self.states.truncate(0);
        self.states.push(StateInfo::new());
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn opposite_side(&self) -> Color {
        !self.side_to_move
    }

    pub fn empty(&self, s: Square) -> bool {
        self.board[s] == NO_PIECE
    }

    pub fn piece_on(&self, s: Square) -> Piece {
        self.board[s]
    }

    pub fn moved_piece(&self, m: Move) -> Piece {
        self.board[m.from()]
    }

    pub fn pieces(&self) -> Bitboard {
        self.by_type_bb[ALL_PIECES]
    }

    pub fn pieces_p(&self, pt: PieceType) -> Bitboard {
        self.by_type_bb[pt]
    }

    pub fn pieces_pp(&self, pt1: PieceType, pt2: PieceType) -> Bitboard {
        self.pieces_p(pt1) | self.pieces_p(pt2)
    }

    pub fn pieces_c(&self, c: Color) -> Bitboard {
        debug_assert!(c.is_ok());
        self.by_color_bb[c]
    }

    pub fn pieces_cp(&self, c: Color, pt: PieceType) -> Bitboard {
        self.pieces_c(c) & self.pieces_p(pt)
    }

    pub fn pieces_cpp(&self, c: Color, pt1: PieceType, pt2: PieceType) -> Bitboard {
        self.pieces_c(c) & self.pieces_pp(pt1, pt2)
    }

    pub fn count(&self, c: Color, pt: PieceType) -> i32 {
        self.piece_count[Piece::make(c, pt)]
    }

    pub fn square(&self, c: Color, pt: PieceType) -> Square {
        lsb(self.pieces_cp(c, pt))
    }

    pub fn ep_square(&self) -> Square {
        self.st().ep_square
    }

    pub fn has_castling_right(&self, cr: CastlingRight) -> bool {
        self.st().castling_right & cr != NO_CASTLING
    }

    pub fn castling_right(&self, c: Color) -> CastlingRight {
        self.st().castling_right & CastlingRight(3 << (2 * c.0))
    }

    pub fn can_castle(&self, c: Color) -> bool {
        self.castling_right(c) != NO_CASTLING
    }

    pub fn castling_impeded(&self, cr: CastlingRight) -> bool {
        self.pieces() & self.castling_path[cr] != EMPTY_BB
    }

    pub fn castling_rook_square(&self, cr: CastlingRight) -> Square {
        self.castling_rook_square[cr]
    }

    pub fn attacks_from_pawn(&self, c: Color, s: Square) -> Bitboard {
        pawn_attacks_bb(c, s)
    }

    pub fn attacks_from(&self, pt: PieceType, s: Square) -> Bitboard {
        debug_assert!(pt != PAWN);
        attacks_bb(pt, s, self.pieces())
    }

    pub fn attackers_to_occ(&self, s: Square, occ: Bitboard) -> Bitboard {
        (self.attacks_from_pawn(BLACK, s) & self.pieces_cp(WHITE, PAWN))
            | (self.attacks_from_pawn(WHITE, s) & self.pieces_cp(BLACK, PAWN))
            | (self.attacks_from(KNIGHT, s) & self.pieces_p(KNIGHT))
            | (attacks_bb(ROOK, s, occ) & self.pieces_pp(ROOK, QUEEN))
            | (attacks_bb(BISHOP, s, occ) & self.pieces_pp(BISHOP, QUEEN))
            | (self.attacks_from(KING, s) & self.pieces_p(KING))
    }

    pub fn attackers_to(&self, s: Square) -> Bitboard {
        self.attackers_to_occ(s, self.by_type_bb[ALL_PIECES])
    }

    pub fn checkers(&self) -> Bitboard {
        self.st().checkers_bb
    }

    pub fn blockers_for_king(&self, c: Color) -> Bitboard {
        self.st().blockers_for_king[c]
    }

    pub fn pinners(&self, c: Color) -> Bitboard {
        self.st().pinners[c]
    }

    pub fn check_squares(&self, pt: PieceType) -> Bitboard {
        self.st().check_squares[pt]
    }

    pub fn pawn_passed(&self, c: Color, s: Square) -> bool {
        self.pieces_cp(!c, PAWN) & passed_pawn_span(c, s) == 0
    }

    pub fn advanced_pawn_push(&self, m: Move) -> bool {
        self.moved_piece(m).piece_type() == PAWN
            && relative_rank(self.side_to_move, m.from().rank()) > RANK_4
    }

    pub fn key(&self) -> Key {
        self.st().key
    }

    pub fn psq_value(&self) -> psqt::Value {
        self.st().psq
    }

    pub fn game_ply(&self) -> i32 {
        self.game_ply
    }

    pub fn rule50_count(&self) -> i32 {
        self.st().rule50
    }

    pub fn opposite_bishops(&self) -> bool {
        self.piece_count[W_BISHOP] == 1
            && self.piece_count[B_BISHOP] == 1
            && opposite_colors(self.square(WHITE, BISHOP), self.square(BLACK, BISHOP))
    }

    pub fn is_chess960(&self) -> bool {
        self.chess960
    }

}