#![feature(test)]

extern crate test;

use ataxx;

fn run_singles() {
    for sq in 0..49 {
        std::hint::black_box(ataxx::Bitboard::from_index(sq).singles());
    }
}

fn run_doubles() {
    for sq in 0..49 {
        std::hint::black_box(ataxx::Bitboard::from_index(sq).doubles());
    }
}

#[cfg(test)]
mod shifts {
    use super::*;
    use test::Bencher;

    #[bench]
    fn singles(b: &mut Bencher) {
        b.iter(|| run_singles());
    }

    #[bench]
    fn doubles(b: &mut Bencher) {
        b.iter(|| run_doubles());
    }
}
