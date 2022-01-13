#[cfg(test)]
//#[macro_use]
mod movegen_test;


use crate::types::bitboard::*;
use crate::types::square::*;
use crate::types::piece::*;
use crate::types::r#move::*;
use crate::attacks::attack_bb::*;


use crate::position::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenType(u32);

pub const CAPTURES: GenType = GenType(0);
pub const QUIETS: GenType = GenType(1);
pub const QUIET_CHECKS: GenType = GenType(2);
pub const EVASIONS: GenType = GenType(3);
pub const NON_EVASIONS: GenType = GenType(4);
pub const LEGAL: GenType = GenType(5);

#[derive(Debug, Clone, Copy)]
pub struct ExtMove {
    pub m: Move,
    pub value: i32,

}

impl ExtMove {
    pub fn new() -> ExtMove {
        ExtMove {
            m: Move::NONE,
            value: 0
        }
    }
}

// trait GenTypeTrait {}
// impl GenTypeTrait for GenType {}

// pub fn gen_moves<DO: bool>(pos: Position){
//     QUIETS;
// }


fn generate_moves(us: Color, pt: PieceType, checks: bool,
    pos: &Position, list: &mut [ExtMove], mut idx: usize, 
    target: Bitboard
) -> usize {
    debug_assert!(pt != KING && pt != PAWN);

    let bb = pos.pieces_cp(us, pt);
    // if pt == ROOK {
    //         println!("{}", pretty(bb));
    //         println!("{}", pretty(target));
    //     }

    //println!("{:?}", pt);

    for from in bb {
        //println!("{}", pt.to_char());
        let mut b = pos.attacks_from(pt, from) & target;
        // if pt == ROOK {
        //     println!("{}", pretty(target));
        // }

        // To check, you either move freely a blocker or make a direct check.
        if checks && (pt == QUEEN || (pos.blockers_for_king(!us) & from) == EMPTY_BB) {
            b &= pos.check_squares(pt);
        }

        for to in b {
            list[idx].m = Move::make(from, to);
            idx += 1;
        }
    }

    idx
}

fn generate_pawn_moves(us: Color, gen_type: GenType,
    pos: &Position, list: &mut [ExtMove], mut idx: usize, target: Bitboard) -> usize {

    let them = !us;
    //let trank_8bb = if us == WHITE { RANK_8_BB } else { RANK_1_BB };
    let trank_7bb = if us == WHITE { RANK_7_BB } else { RANK_2_BB };
    let trank_3bb = if us == WHITE { RANK_3_BB } else { RANK_6_BB };
    let up    = if us == WHITE { NORTH      } else { SOUTH      };
    let up_right = if us == WHITE { NORTH_EAST } else { SOUTH_WEST };
    let up_left  = if us == WHITE { NORTH_WEST } else { SOUTH_EAST };

    let empty_squares = !pos.pieces();
    let enemies = if gen_type == EVASIONS { pos.checkers() } else { pos.pieces_c(them) };

    let pawns_on_7     = pos.pieces_cp(us, PAWN) &  trank_7bb;
    let pawns_not_on_7 = pos.pieces_cp(us, PAWN) & !trank_7bb;

    // Single and double pawn pushes, no promotions
    if gen_type != CAPTURES {

        let mut b1 = pawns_not_on_7.shift(up)   & empty_squares;
        let mut b2 = (b1 & trank_3bb).shift(up) & empty_squares;

        if gen_type == EVASIONS { // Consider only blocking squares 
            b1 &= target;
            b2 &= target;
        }

        if gen_type == QUIET_CHECKS {
            // To make a quiet check, you either make a direct check by pushing a pawn
            // or push a blocker pawn that is not on the same file as the enemy king.
            // Discovered check promotion has been already generated amongst the captures.
            let ksq = pos.square(them, KING);
            let dc_candidate_pawns = pos.blockers_for_king(them) & !ksq.file_bb();
            b1 &= pawn_attacks_bb(them, ksq) | (dc_candidate_pawns).shift(up);
            b2 &= pawn_attacks_bb(them, ksq) | (dc_candidate_pawns).shift(up).shift(up);
        }

        for to in b1 {
            list[idx].m = Move::make(to - up, to);
            idx += 1;
        }

        for to in b2 {
            list[idx].m = Move::make(to - up - up, to);
            idx += 1;
        }

    }

    // Promotions and underpromotions
    if pawns_on_7 != EMPTY_BB
    {
        let b1 = pawns_on_7.shift(up_right) & enemies;
        let b2 = pawns_on_7.shift(up_left) & enemies;
        let mut b3 = pawns_on_7.shift(up) & empty_squares;

        if gen_type == EVASIONS {
            b3 &= target;
        }

        for to in b1 {
            idx = make_promotions(gen_type, up_right, list, idx, to);
        }

        for to in b2 {
            idx = make_promotions(gen_type, up_left, list, idx, to);
        }

        for to in b3 {
            idx = make_promotions(gen_type, up, list, idx, to);
        }

    }

    // Standard and en passant captures
    if gen_type == CAPTURES || gen_type == EVASIONS || gen_type == NON_EVASIONS
    {

        let mut b1 = pawns_not_on_7.shift(up_right) & enemies;
        let b2 = pawns_not_on_7.shift(up_left) & enemies;

        for to in b1 {
            list[idx].m = Move::make(to - up_right, to);
            idx += 1;
        }

        for to in b2 {
            list[idx].m = Move::make(to - up_left, to);
            idx += 1;
        }

        if pos.ep_square() != Square::NONE {
            debug_assert!(pos.ep_square().rank() == relative_rank(us, RANK_6));

            // An en passant capture cannot resolve a discovered check
            if gen_type == EVASIONS && (target & (pos.ep_square() + up)) != 0 {
                return idx;
            }

            b1 = pawns_not_on_7 & pawn_attacks_bb(them, pos.ep_square());

            debug_assert!(b1 != EMPTY_BB);

            for from in b1 {
                list[idx].m = Move::make_special(EN_PASSANT, from, pos.ep_square());
                idx += 1;
            }
        }
    }

    idx
}

fn make_promotions(gen_type: GenType, d: Direction,
    list: &mut [ExtMove], mut idx: usize, to: Square) -> usize {

    if gen_type == CAPTURES || gen_type == EVASIONS || gen_type == NON_EVASIONS
    {
        list[idx].m = Move::make_prom(to - d, to, QUEEN);
        idx += 1;
    }

    if gen_type == QUIETS || gen_type == EVASIONS || gen_type == NON_EVASIONS
    {
        list[idx    ].m = Move::make_prom(to - d, to, ROOK);
        list[idx + 1].m = Move::make_prom(to - d, to, BISHOP);
        list[idx + 2].m = Move::make_prom(to - d, to, KNIGHT);
        idx += 3;
    }

    idx
}

fn generate_all(us: Color, gen_type: GenType, 
    pos: &Position, list: &mut [ExtMove], mut idx: usize) -> usize {

    debug_assert!(gen_type != LEGAL );
    let checks = gen_type == QUIET_CHECKS;

    //println!("{}", !more_than_one(pos.checkers()));

    let ksq = pos.square(us, KING);
    let target = match gen_type {
            EVASIONS =>      between_bb(ksq, lsb(pos.checkers())),
            NON_EVASIONS => !pos.pieces_c( us),
            CAPTURES =>      pos.pieces_c(!us), 
            _ =>            !pos.pieces(     )

    } ;

    // Skip generating non-king moves when in double check
    if gen_type != EVASIONS || !more_than_one(pos.checkers())
    {
        //println!("Here");
        idx = generate_pawn_moves(us, gen_type, pos, list, idx, target);
        idx = generate_moves(us, KNIGHT, checks, pos, list, idx, target);
        idx = generate_moves(us, BISHOP, checks, pos, list, idx, target);
        idx = generate_moves(us, ROOK,   checks, pos, list, idx, target);
        idx = generate_moves(us, QUEEN,  checks, pos, list, idx, target); 
        //println!("{:?}", list[0].m.to_string(false));
    }

    if !checks || (pos.blockers_for_king(!us) & ksq) != EMPTY_BB
    {
        //println!("Here {}", gen_type == EVASIONS);
        let mut b = pseudo_attacks(KING, ksq);
        b &= if gen_type == EVASIONS { !pos.pieces_c(us) } else { target };

        if checks { 
            b &= !pseudo_attacks(QUEEN, pos.square(!us, KING));
        }

        for to in b {
            //println!("Here {}", gen_type == EVASIONS);
            list[idx].m = Move::make(ksq, to);
            //println!("{}", list[idx].m.to_string(pos.is_chess960()));
            //println!("{:?}", list);
            idx += 1;

        }

        if gen_type == QUIETS || gen_type == NON_EVASIONS && pos.has_castling_right(castling_right_c(us, ANY_CASTLING)) {
            for cr in [castling_right_c(us, KING_SIDE), castling_right_c(us, QUEEN_SIDE)] {
                if !pos.castling_impeded(cr) && pos.has_castling_right(cr) {
                    list[idx].m = Move::make_special(CASTLING, ksq, pos.castling_rook_square(cr));
                    //println!("{}", list[idx].m.to_string(pos.is_chess960()));
                    idx += 1;
                }
            }
        }
    }

    idx

}

pub fn generate(gen_type: GenType, 
    pos: &Position, list: &mut [ExtMove], idx: usize) -> usize {

    debug_assert!(gen_type != LEGAL );
    debug_assert!((gen_type == EVASIONS) == (pos.checkers() != EMPTY_BB));


    generate_all(pos.side_to_move(), gen_type, pos, list, idx)

}

// generate_legal() generates all the legal moves in the given position
pub fn generate_legal(
    pos: &Position, list: &mut [ExtMove], idx: usize
) -> usize {
    let us = pos.side_to_move();
    let pinned = pos.blockers_for_king(us) & pos.pieces_c(us);
    let ksq = pos.square(us, KING);

    let pseudo = if pos.checkers() != 0 {
        generate(EVASIONS, pos, list, idx)
    } else {
        generate(NON_EVASIONS, pos, list, idx)
    };

    // println!("{:?}", list);
    // let mut legal = pseudo;
    let mut legal = idx;
    for i in idx..pseudo {
        let m = list[i].m;
        //print!("{} {}\n", pos.fen(), m.to_string(false));
        if (pinned == EMPTY_BB && m.from() != ksq && m.move_type() != EN_PASSANT)
            || pos.legal(m)

        // if pos.legal(m)
        {   
            //print!("{} {}\n", pos.fen(), m.to_string(false));
            list[legal].m = m;
            legal += 1;
        }
        else {
            //println!("Shit");
        }
        // if ( (pinned != 0) && ((pinned & m.from()) != 0)) || m.from() == ksq || m.move_type() == EN_PASSANT 
        //   && !pos.legal(m)
        // {
        //     //println!("Shit");
        // }
        // else {
        //     list[legal].m = m;
        //     legal += 1;
        // }


    }

    for i in legal..pseudo+1{
        list[i].m = Move::NONE;
    }



    //println!("{:?}", list);


    legal
}

