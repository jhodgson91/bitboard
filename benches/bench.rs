#[macro_use]
extern crate criterion;
extern crate bitboard;
extern crate typenum;

use criterion::Criterion;

use bitboard::*;
use typenum::*;

type RealLife = BitBoard<U8, u64>;

fn real_life() {
    use Move::*;
    use Rotation::*;

    let mut bb = RealLife::new(vec![(4, 4)]);
    let queen = bb
        .moves()
        .translate(Up(1))
        .translate(UpLeft(1, 1))
        .rotate(Clockwise)
        .mirror()
        .repeat(8)
        .collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Moves", |b| b.iter(|| real_life()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
