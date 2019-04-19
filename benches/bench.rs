#[macro_use]
extern crate criterion;
extern crate bitboard;
extern crate typenum;

use criterion::Criterion;

use bitboard::*;
use typenum::*;

type RealLife = BitBoard<U16, u64>;

fn criterion_benchmark(c: &mut Criterion) {
    let mut q1 = RealLife::new(vec![(4,4)]);
    let mut q2 = RealLife::new(vec![(4,4)]);

    c.bench_function("allocate", |b| b.iter(|| RealLife::default()));
    c.bench_function("unmasked_shift", move |b| b.iter( || { q1 <<= Move::Up(1) } ));
    c.bench_function("masked_shift", move |b| b.iter( || { q2 <<= Move::UpLeft(1,1) }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
