#[macro_use]
extern crate criterion;
extern crate bitboard;
extern crate typenum;

use criterion::Criterion;

use bitboard::*;
use typenum::*;

type RealLife = BitBoard<U8, u64>;
type WorstCase = BitBoard<U100, u8>;

fn worst_case() {
    let mut bb = WorstCase::default();

    bb = &bb << 100;
    bb = &bb >> 100;

    bb <<= 100;
    bb >>= 100;
}

fn real_life() {
    let mut bb = RealLife::default();

    bb = &bb << 8;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Real life", |b| b.iter(|| real_life()));
    //c.bench_function("Worst case", |b| b.iter(|| worst_case()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
