use super::*;

impl Position {

    // set() initializes the position objection with the given FEN string.
    // This function is not very robust - make sure that input FENs are
    // correct. This is assumed to be the responsibility of the GUI.
    pub fn set(&mut self, fen_str: &str, is_chess960: bool) {

        self.clear();
        
        let mut iter = fen_str.split_whitespace();

        // 1. Piece placement
        let pieces = iter.next().unwrap();
        let mut sq = Square::A8;
        for c in pieces.chars() {
            if let Some(d) = c.to_digit(10) {
                sq += (d as i32) * EAST; // Advance the given number of files
            } else if c == '/' {
                sq += 2 * SOUTH;
            } else if let Some(idx) = String::from_iter(PIECE_TO_CHAR).find(c) {
                self.put_piece(Piece(idx as u32), sq);
                sq += EAST;
            }
        }

        // 2. Active color
        let color = iter.next().unwrap();
        self.side_to_move = if color == "b" { BLACK } else { WHITE };

        // 3. Castling availability. Compatible with 3 standards: Normal FEN
        // standard, Shredder-FEN that uses the letters of the columns on
        // which the rooks began the game instead of KQkq and also X-FEN
        // standard that, in case of Chess960, if an inner rook is associated
        // with the castling right, the castling tag is replaced by the file
        // letter of the involved rook, as for the Shredder-FEN.
        let castling = iter.next().unwrap();
        if castling != "-" {
            for c in castling.chars() {
                let color = if c.is_lowercase() { BLACK } else { WHITE };
                let rook = Piece::make(color, ROOK);
                let side = c.to_uppercase().next().unwrap();
                let mut rsq;
                if side == 'K' {
                    rsq = Square::H1.relative(color);
                    while self.piece_on(rsq) != rook {
                        rsq += WEST;
                    }
                } else if side == 'Q' {
                    rsq = Square::A1.relative(color);
                    while self.piece_on(rsq) != rook {
                        rsq += EAST;
                    }
                } else if side >= 'A' && side <= 'H' {
                    let f = side.to_digit(18).unwrap() - 10;
                    let file = File(f as u32);
                    rsq = Square::make(file, relative_rank(color, RANK_1));
                } else {
                    continue;
                }
                self.set_castling_right(color, rsq);
            }
        }

        // 4. En passant square. Ignore if no pawn capture is possible
        let enpassant = iter.next().unwrap();
        self.st_mut().ep_square = Square::NONE;
        if enpassant != "-" {
            let file = enpassant.chars().nth(0).unwrap();
            let file = file.to_digit(18).unwrap() - 10;
            let rank = if self.side_to_move == WHITE { 5 } else { 2 };
            let ep_sq = Square::make(File(file as u32), Rank(rank as u32));
            if self.attackers_to(ep_sq)
                    & self.pieces_cp(self.side_to_move, PAWN) != 0
                && self.pieces_cp(!self.side_to_move, PAWN)
                    & (ep_sq + pawn_push(!self.side_to_move)) != 0
            {
                self.st_mut().ep_square = ep_sq;
            }
        }

        // 5-6. Halfmove clock and fullmove number
        if let Some(halfmove) = iter.next() {
            self.st_mut().rule50 = halfmove.parse().unwrap();
        } else {
            self.st_mut().rule50 = 0;
        }

        // Convert from fullmove starting from 1 to game_ply starting from 0.
        // Handle also common incorrect FEN with fullmove = 0.
        if let Some(fullmove) = iter.next() {
            let fullmove = fullmove.parse::<i32>().unwrap();
            self.game_ply = std::cmp::max(2 * (fullmove - 1), 0);
        } else {
            self.game_ply = 0;
        }
        if self.side_to_move == BLACK {
            self.game_ply += 1;
        }

        self.chess960 = is_chess960;
        self.set_state();

        debug_assert!(self.is_ok());
    }

    // fen() returns a FEN representation of the position. In case of Chess960
    // the Shredder-FEN notation is used.

    pub fn fen(&self) -> String {
        let mut ss = String::new();

        for r in (0..8).rev() {
            let mut f = 0;
            while f < 8 {
                let mut empty_cnt = 0u8;
                while f < 8 && self.empty(Square::make(File(f as u32), Rank(r as u32))) {
                    empty_cnt += 1;
                    f += 1;
                }
                if empty_cnt > 0 {
                    ss.push((48u8 + empty_cnt) as char);
                }
                if f < 8 {
                    let c = self.piece_on(Square::make(File(f as u32), Rank(r as u32))).to_char();
                    ss.push(c);
                    f += 1;
                }
            }
            if r > 0 {
                ss.push('/');
            }
        }

        ss.push_str(if self.side_to_move == WHITE { " w " } else { " b " });

        self.castle_helper(&mut ss, WHITE_OO, 'K');
        self.castle_helper(&mut ss, WHITE_OOO, 'Q');
        self.castle_helper(&mut ss, BLACK_OO, 'k');
        self.castle_helper(&mut ss, BLACK_OOO, 'q');

        if !self.has_castling_right(ANY_CASTLING) {
            ss.push('-');
        }

        if self.ep_square() == Square::NONE {
            ss.push_str(" - ");
        } else {
            ss.push(' ');
            ss.push_str(&self.ep_square().to_string());
            ss.push(' ');
        }

        ss.push_str(&self.rule50_count().to_string());
        ss.push(' ');
        ss.push_str(&(1 + self.game_ply() / 2).to_string());

        ss
    }

    fn castle_helper(&self, ss: &mut String, cr: CastlingRight, c: char) {
        if !self.has_castling_right(cr) {
            return;
        }

        if !self.chess960 {
            ss.push(c);
        } else {
            let f = self.castling_rook_square(cr).file();
            let r = self.castling_rook_square(cr).rank();
            let mut c = 65 + f.0;
            if r == RANK_8 {
                c += 32;
            }
            ss.push((c as u8) as char);
        }
    }

    
}