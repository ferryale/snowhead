pub mod types;
pub mod attacks;

use types::bitboard::*;
use types::square::*;
use types::piece::*;
use attacks::{sliding_attacks, square_bb};
use attacks::magics::*;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// A few comments on the different scenarios this file handles (and does not) with the corresponding compile commands
// Native compile for own machine : set RUSTFLAGS=-C target-cpu=native; cargo run --release (works)
// Host compile for target machine: set RUSTFLAGS=-C target-cpu=<your-target-machine-cpu>; cargo rustc --release --bin scam --target <your_target>
// In the case that the Host does not have BMI2, while the target-cpu wants BMI2 instructions, this build script will fail.
// Due to https://github.com/rust-lang/cargo/issues/4423 (build.rs can't be given any build flags), we sadly can not detect
// whether the host has the bmi2 instruction set or not. For now, we will just assume it has.
// For cross-compiling purposes, we extract the target-feature from the env var CARGO_CFG_TARGET_FEATURE instead of
// using #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))] since that is the config for the host, and has nothing to do
// with the target in case of cross compilation. Additionally, if --target is supplied during compilation,
// #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))] will always evaluate to false due to above issue.
pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let magic_path = Path::new(&out_dir).join("attack_tables.rs");
    let mut file = File::create(magic_path).unwrap();

    let has_bmi2 = env::var("CARGO_CFG_TARGET_FEATURE").map_or(false, |x| x.contains("bmi2"));
    if has_bmi2 {
        writeln!(file, "//Tables for BMI2").unwrap();
    } else {
        writeln!(file, "//Tables for Magics").unwrap();
    }
    let attacks = init_attacks(has_bmi2);
    write!(
        file,
        "#[rustfmt::skip]\n pub static ATTACKS: [u64; 107648] = {};\n",
        print_arr1d(&attacks, false)
    )
    .unwrap();

    let (line_bb, between_bb) = init_line_and_between_bb();
    write!(
        file,
        "#[rustfmt::skip]\n pub const LINE_BB: [[Bitboard; SQUARE_NB]; SQUARE_NB] = {};\n",
        print_arr2d(&line_bb, true)
    )
    .unwrap();

    write!(
        file,
        "#[rustfmt::skip]\n pub const BETWEEN_BB: [[Bitboard; SQUARE_NB]; SQUARE_NB] = {};\n",
        print_arr2d(&between_bb, true)
    )
    .unwrap();

    //println!("cargo:rerun-if-changed=build.rs");

}

pub fn print_arr2d(arr: &[Vec<Bitboard>], bb: bool) -> String {
    let mut res_str = String::new();
    res_str.push('[');
    for arr2 in arr.iter() {
        res_str.push_str(&format!("{},", print_arr1d(arr2, bb)));
    }
    res_str.push(']');
    res_str
}

pub fn print_arr1d(arr: &[Bitboard], bb: bool) -> String {
    let mut res_str = String::new();
    res_str.push('[');
    for &attack in arr.iter() {
        if bb {
            res_str.push_str(&format!("Bitboard({}),", attack.0))
        } else {
            res_str.push_str(&format!("{},", attack.0));
        };
    }
    res_str.push(']');
    res_str
}


impl Magic {
    pub fn build_index(self, occ: Bitboard, has_bmi2: bool) -> usize {
        if has_bmi2 {
            use std::arch::x86_64::_pext_u64;
            self.offset + unsafe { _pext_u64(occ.0, self.mask.0) } as usize
        } else {
            self.offset + (((occ & self.mask).0).wrapping_mul(self.magic) >> self.shift) as usize
        }
    }
}

pub fn init_attacks(has_bmi2: bool) -> Vec<Bitboard> {
    let mut res = vec![EMPTY_BB; 107648];
    for (magics, pt) in [(BISHOP_MAGICS, BISHOP), (ROOK_MAGICS, ROOK)].iter() {
        for (sq, magic) in magics.iter().enumerate() {
            let mut occ = EMPTY_BB;
            loop {
                let attacks = sliding_attacks(*pt, Square(sq as u32), occ);
                res[magic.build_index(occ, has_bmi2)] = attacks;
                occ = Bitboard((occ.0.wrapping_sub(magic.mask.0)) & magic.mask.0);
                if occ == EMPTY_BB {
                    break;
                }
            }
        }
    }
    res
}

pub fn init_line_and_between_bb() -> (Vec<Vec<Bitboard>>, Vec<Vec<Bitboard>>) {
    let mut line_bb = vec![vec![EMPTY_BB; SQUARE_NB]; SQUARE_NB];
    let mut between_bb = vec![vec![EMPTY_BB; SQUARE_NB]; SQUARE_NB];
    for s1 in VALID_SQUARES {
        for &pt in [BISHOP, ROOK].iter() {
            for s2 in VALID_SQUARES {

                if sliding_attacks(pt, s1, EMPTY_BB) & square_bb(s2) != 0 {
                    line_bb[s1.0 as usize][s2.0 as usize] =
                    sliding_attacks(pt, s1, EMPTY_BB)
                    & sliding_attacks(pt, s2, EMPTY_BB) | square_bb(s1) | square_bb(s2);

                    between_bb[s1.0 as usize][s2.0 as usize] =
                    sliding_attacks(pt, s1, square_bb(s2))
                    & sliding_attacks(pt, s2, square_bb(s1));
                }

                between_bb[s1.0 as usize][s2.0 as usize] |= square_bb(s2);

            }
        }
    }
    (line_bb, between_bb)
}

// pub fn init_line_and_between_bb() -> (Vec<Vec<Bitboard>>, Vec<Vec<Bitboard>>) {
//     let mut line_bb = vec![vec![EMPTY_BB; SQUARE_NB]; SQUARE_NB];
//     let mut between_bb = vec![vec![EMPTY_BB; SQUARE_NB]; SQUARE_NB];
//     for s1 in ALL_SQUARES {
//         for s2 in ALL_SQUARES {
//             for pt in [BISHOP, ROOK] {
//                 if (sliding_attacks(pt, s1, EMPTY_BB) & _square_bb(s2)) != 0 {
//                     line_bb[s1][s2]    = (sliding_attacks(pt, s1, EMPTY_BB) & sliding_attacks(pt, s2, EMPTY_BB)) | _square_bb(s1) | _square_bb(s2);
//                     between_bb[s1][s2] = sliding_attacks(pt, s1, _square_bb(s2)) & sliding_attacks(pt, s2, _square_bb(s1));
//                 }
//                 between_bb[s1][s2] |= _square_bb(s2);
//             }
//         }
//     }
//     (line_bb, between_bb)
// }



