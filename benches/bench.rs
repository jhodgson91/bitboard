#[macro_use]
extern crate criterion;
extern crate bitboard;
extern crate typenum;

use criterion::Criterion;

use bitboard::*;
use typenum::*;

type RealLife = BitBoard<U8, u64>;

fn real_life() {
    let mut bb = RealLife::new(vec![(4, 4)]);
    let bb_ref = &bb;

    let mut moves = RealLife::default();
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Left(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Right(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Up(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Down(i));
    moves ^= bb_ref;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Moves", |b| b.iter(|| real_life()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
