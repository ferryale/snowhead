#[cfg(test)]
//#[macro_use]
mod game_test;

use super::*;

impl Position {

    pub fn do_move(&mut self, m: Move) {
        self._do_move(m, self.gives_check(m));
    }

    /// do_move() makes a move and saves all information necessary to a
    /// StateInfo object. The move is assumed to be legal. Pseudo-legal
    /// moves should be filtered out before this function is called.
    fn _do_move(&mut self, m: Move, gives_check: bool) {
        
        debug_assert!(m.is_ok());

        let mut k = self.st().key ^ self.zobrist.side;

        // Copy some fields of the old state to our new StateInfo object
        // except the ones which are going to be recalculated from scratch
        // anyway.
        let st_copy = self.st().copy();
        self.states.push(st_copy);

        // Increment ply counters. The rule50 field will be reset to zero
        // later on in case of a capture or a pawn move.
        self.game_ply += 1;
        self.st_mut().rule50 += 1;
        self.st_mut().plies_from_null += 1;

        let us = self.side_to_move;
        let them = self.opposite_side();
        let from = m.from();
        let mut to = m.to();
        let pc = self.piece_on(from);
        let mut captured = if m.move_type() == EN_PASSANT {
            Piece::make(them, PAWN)
        } else {
            self.piece_on(to)
        };

        debug_assert!(pc.color() == us);
        debug_assert!(
            captured == NO_PIECE
                || captured.color() == if m.move_type() != CASTLING { them } else { us }
        );
        debug_assert!(captured.piece_type() != KING);

        // Castling
        if m.move_type() == CASTLING {
            debug_assert!(pc == Piece::make(us, KING));
            debug_assert!(captured == Piece::make(us, ROOK));

            let mut rfrom = Square::NONE;
            let mut rto = Square::NONE;
            self.do_castling::<true>(us, from, &mut to, &mut rfrom, &mut rto);

            self.st_mut().psq.0 += psqt::psq(captured, rto).0 - psqt::psq(captured, rfrom).0;

            k ^= self.zobrist.psq[captured][rfrom] ^ self.zobrist.psq[captured][rto];
            captured = NO_PIECE;
        }

        // Captures
        if captured != NO_PIECE {
            let mut capsq = to;

            // If the captured piece is a pawn, update pawn hash key, otherwise
            // update non-pawn material.
            if captured.piece_type() == PAWN {
                if m.move_type() == EN_PASSANT {
                    capsq -= pawn_push(us);

                    debug_assert!(pc == Piece::make(us, PAWN));
                    debug_assert!(to == self.st_mut().ep_square);
                    debug_assert!(to.relative_rank(us) == RANK_6);
                    debug_assert!(self.piece_on(to) == NO_PIECE);
                    debug_assert!(self.piece_on(capsq) == Piece::make(them, PAWN));

                    //self.board[capsq] = NO_PIECE; Thhis has to be done AFTER the piece is removed: bug catched!
                }
            } 

            // Update board and piece lists
            self.remove_piece(capsq);

            if m.move_type() == EN_PASSANT {
                self.board[capsq] = NO_PIECE;
            }

            // Update hash
            k ^= self.zobrist.psq[captured][capsq];

            // Update incremental scores
            self.st_mut().psq.0 -= psqt::psq(captured, capsq).0;

            // Reset rule 50 counter
            self.st_mut().rule50 = 0;
        }

        // Update hash key
        k ^= self.zobrist.psq[pc][from] ^ self.zobrist.psq[pc][to];

        // Reset en passant square
        if self.st_mut().ep_square != Square::NONE {
            k ^= self.zobrist.en_passant[self.st().ep_square.file()];
            self.st_mut().ep_square = Square::NONE;
        }

        // Update castling rights if needed
        if self.st_mut().castling_right != NO_CASTLING
            && self.castling_rights_mask[from] | self.castling_rights_mask[to]
                != NO_CASTLING
        {
            let cr = self.castling_rights_mask[from]
                | self.castling_rights_mask[to];
            k ^= self.zobrist.castling[self.st().castling_right];
            self.st_mut().castling_right &= !cr;
            k ^= self.zobrist.castling[self.st().castling_right];
        }

        // Move the piece. The tricky Chess960 castling is handled earlier
        if m.move_type() != CASTLING {
            self.move_piece(from, to);
        }

        // If the moving piece is a pawn do some special extra work
        if pc.piece_type() == PAWN {
            // Set en-passant square if the moved pawn can be captured
            if to.0 ^ from.0 == 16
                && pawn_attacks_bb(us, to - pawn_push(us)) & self.pieces_cp(them, PAWN) != 0
            {
                self.st_mut().ep_square = to - pawn_push(us);
                k ^= self.zobrist.en_passant[self.st().ep_square.file()];

            } else if m.move_type() == PROMOTION {
                let promotion = Piece::make(us, m.promotion_type());

                debug_assert!(to.relative_rank(us) == RANK_8);
                debug_assert!(promotion.piece_type() >= KNIGHT && promotion.piece_type() <= QUEEN);

                self.remove_piece(to);
                self.put_piece(promotion, to);

                // Update hash keys
                k ^= self.zobrist.psq[pc][to] ^ self.zobrist.psq[promotion][to];

                // Update incremental score
                self.st_mut().psq.0 +=
                    psqt::psq(promotion, to).0 - psqt::psq(pc, to).0;
            }

            // Reset rule 50 draw counter
            self.st_mut().rule50 = 0;
        }

        // Update incremental scores
        self.st_mut().psq.0 += psqt::psq(pc, to).0 - psqt::psq(pc, from).0;

        // Set captured piece
        self.st_mut().captured_piece = captured;

        // Update the key with the final value
        self.st_mut().key = k;

        // Calculate checkers bitboard (if move gives check)
        self.st_mut().checkers_bb = if gives_check {
            self.attackers_to(self.square(them, KING)) & self.pieces_c(us)
        } else {
            EMPTY_BB
        };

        self.side_to_move = them;

        // Update king attacks used for fast check detection
        self.set_check_info();

        debug_assert!(self.is_ok());
    }

    /// Position::undo_move() unmakes a move. When it returns, the position should
    /// be restored to exactly the same state as before the move was made.
    pub fn undo_move(&mut self, m: Move) {
        
        assert!(m.is_ok());
        
        self.side_to_move = !self.side_to_move;
        
        let us = self.side_to_move;
        //let them = !us;
        let from = m.from();
        let mut to = m.to();
        let mut pc = self.piece_on(to);

        debug_assert!(self.empty(from) || m.move_type() == CASTLING);
        debug_assert!(self.st().captured_piece.piece_type() != KING);

        if m.move_type() == PROMOTION {

            debug_assert!(to.relative_rank(us) == RANK_8);
            debug_assert!(pc.piece_type() == m.promotion_type());
            debug_assert!(
                pc.piece_type() >= KNIGHT && pc.piece_type() <= QUEEN);

            self.remove_piece(to);
            pc = Piece::make(us, PAWN);
            self.put_piece(pc, to);
        }

        if  m.move_type() == CASTLING {
            let mut rfrom = Square::NONE;
            let mut rto = Square::NONE;
            self.do_castling::<false>(us, from, &mut to, &mut rfrom, &mut rto);

        } else {
            self.move_piece(to, from);

            if self.st().captured_piece != NO_PIECE {

                let mut capsq = to;

                if m.move_type() == EN_PASSANT {
                    capsq -= pawn_push(us);

                    debug_assert!(pc.piece_type() == PAWN);
                    //debug_assert!(to == st->previous->epSquare);
                    debug_assert!(to.relative_rank(us) == RANK_6);
                    debug_assert!(self.piece_on(capsq) == NO_PIECE);
                    debug_assert!(
                    self.st().captured_piece == Piece::make(!us, PAWN));
                }
                self.put_piece(self.st().captured_piece, capsq);
            }
        }

        // Finally point our state pointer back to the previous state
        let new_len = self.states.len() - 1;
        self.states.truncate(new_len);
        self.game_ply -= 1;

        debug_assert!(self.is_ok());

    }

    /// Position::is_ok() performs some consistency checks for the
    /// position object and raises an asserts if something wrong is detected.
    /// This is meant to be helpful when debugging.
    pub fn is_ok(&self) -> bool {

        if self.side_to_move != WHITE && self.side_to_move != BLACK
            || self.piece_on(self.square(WHITE, KING)) != W_KING
            || self.piece_on(self.square(BLACK, KING)) != B_KING
            || self.ep_square() != Square::NONE && self.ep_square().relative_rank(self.side_to_move) != RANK_6 {

            panic!("pos: Default");
        }
        true
    }

    // legal() tests whether a pseudo-legal move is legal

    pub fn legal(&self, m: Move) -> bool {
        debug_assert!(m.is_ok());

        let us = self.side_to_move;
        let from = m.from();
        let mut to = m.to();

        debug_assert!(self.moved_piece(m).color() == us);
        debug_assert!(
            self.piece_on(self.square(us, KING)) == Piece::make(us, KING)
        );

        // En passant captures are a tricky special case. Because they are
        // uncommon, we do it simply by testing whether the king is attacked
        // after the move is made.
        if m.move_type() == EN_PASSANT {
            let ksq = self.square(us, KING);
            let capsq = to - pawn_push(us);
            let occupied = (self.pieces() ^ from ^ capsq) | to;

            debug_assert!(to == self.ep_square());
            debug_assert!(self.moved_piece(m) == Piece::make(us, PAWN));
            debug_assert!(self.piece_on(capsq) == Piece::make(!us, PAWN));
            debug_assert!(self.piece_on(to) == NO_PIECE);

            return
                attacks_bb(ROOK, ksq, occupied)
                    & self.pieces_cpp(!us, QUEEN, ROOK) == 0
                && attacks_bb(BISHOP, ksq, occupied)
                    & self.pieces_cpp(!us, QUEEN, BISHOP) == 0;
        }

        // If the moving piece is a king, check whether the destination
        // square is attacked by the opponent. Castling moves are checked
        // for legality during move generation.
        // if self.piece_on(from).piece_type() == KING {
        //     return m.move_type() == CASTLING
        //         || self.attackers_to(m.to()) & self.pieces_c(!us) == EMPTY_BB;
        // }


        if m.move_type() == CASTLING
        {
            // After castling, the rook and king final positions are the same in
            // Chess960 as they would be in standard chess.
            to = relative_square(us, if to > from { Square::G1 } else { Square::C1 });
            let step = if to > from { WEST } else { EAST };

            let mut s = to;
                while s != from {
                if self.attackers_to(s) & self.pieces_c(!us) != EMPTY_BB { return false; }
                s += step;
            }


            // In case of Chess960, verify if the Rook blocks some checks
            // For instance an enemy queen in SQ_A1 when castling rook is in SQ_B1.
            return !self.chess960 || self.blockers_for_king(us) & m.to() == EMPTY_BB;
        }

        if self.piece_on(from).piece_type() == KING { 
            return self.attackers_to_occ(to, self.pieces() ^ from) & self.pieces_c(!us) == EMPTY_BB;
        }

        // A non-king move is legal if and only if it is not pinned or it
        // is moving along the ray towards or away from the king.
        // self.blockers_for_king(us) & from == EMPTY_BB
        // || aligned(from, m.to(), self.square(us, KING))
        self.blockers_for_king(us) & from == EMPTY_BB
        || aligned(from, m.to(), self.square(us, KING))
    }

    pub fn gives_check(&self, m: Move) -> bool {

        //print!("{} {}\n", self.fen(), m.to_string(false));
        debug_assert!(m.is_ok());
        debug_assert!(self.moved_piece(m).color() == self.side_to_move());

        let from = m.from();
        let to = m.to();

        // Is there a direct check?
        if self.st().check_squares[self.piece_on(from).piece_type()]
            & to != 0
        {
            return true;
        }

        // Is there a discovered check?
        if self.blockers_for_king(!self.side_to_move()) & from != 0
            && !aligned(from, to, self.square(!self.side_to_move(), KING))
        {
            return true;
        }

        match m.move_type() {

            NORMAL => false,

            PROMOTION => {
                attacks_bb(m.promotion_type(), to, self.pieces() ^ from)
                & self.square(!self.side_to_move(), KING) != 0
            }

            // En passant capture with check? We have already handled the
            // case of direct checks and ordinary discovered check, so the
            // only case we need to handle is the unusual case of a
            // discovered check through the captured pawn.
            EN_PASSANT => {
                let capsq = Square::make(to.file(), from.rank());
                let b = (self.pieces() ^ from ^ capsq) | to;

                (attacks_bb(ROOK, self.square(!self.side_to_move(), KING), b)
                    & self.pieces_cpp(self.side_to_move(), QUEEN, ROOK))
                | (attacks_bb(BISHOP,
                              self.square(!self.side_to_move(), KING),
                              b)
                    & self.pieces_cpp(self.side_to_move(), QUEEN, BISHOP)) != 0
            }

            CASTLING => {
                let kfrom = from;
                let rfrom = to; // Castling is encoded as king captures rook
                let kto = relative_square(self.side_to_move(),
                    if rfrom > kfrom { Square::G1 } else { Square::C1 });
                let rto = relative_square(self.side_to_move(),
                    if rfrom > kfrom { Square::F1 } else { Square::D1 });

                (pseudo_attacks(ROOK, rto)
                    & self.square(!self.side_to_move(), KING)) != 0
                && (attacks_bb(ROOK, rto,
                        (self.pieces() ^ kfrom ^ rfrom) | rto | kto)
                    & self.square(!self.side_to_move(), KING)) != 0
            }

            _ => {
                debug_assert!(false);
                false
            }
        }
    }


    // do_castling() is a helper used to do/undo a castling move. This is
    // a bit tricky in Chess960 where from/to squares can overlap.
    fn do_castling<const DO: bool>(
        &mut self,
        us: Color,
        from: Square,
        to: &mut Square,
        rfrom: &mut Square,
        rto: &mut Square,
    ) {
        let king_side = *to > from;
        *rfrom = *to; // Castling is encoded as king captures rook
        *rto = relative_square(us, if king_side { Square::F1 } else { Square::D1 });
        *to = relative_square(us, if king_side { Square::G1 } else { Square::C1 });

        // Remove both pieces first since squares could overlap in Chess960
        self.remove_piece(if DO { from } else { *to });
        self.remove_piece(if DO { *rfrom } else { *rto });
        self.board[if DO { from } else { *to }] = NO_PIECE;
        self.board[if DO { *rfrom } else { *rto }] = NO_PIECE;
        self.put_piece(Piece::make(us, KING), if DO { *to } else { from });
        self.put_piece(Piece::make(us, ROOK), if DO { *rto } else { *rfrom });
    }

}