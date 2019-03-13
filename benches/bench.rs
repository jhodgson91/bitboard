#[macro_use]
extern crate criterion;
extern crate bitboard;
extern crate typenum;

use criterion::Criterion;

use bitboard::*;
use typenum::*;

fn big_shift() {
    let mut bb = BitBoard::<U100, u8>::new(vec![]);

    bb = &bb << 100;
    bb = &bb >> 100;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Bit Shift", |b| b.iter(|| big_shift()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
