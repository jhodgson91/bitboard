extern crate bitboard;

use bitboard::*;
use typenum::*;

use std::time::{Duration, Instant};

fn main() {
    let mut bb = BitBoard::<U8, u32>::new(vec![(3, 3)]);
    bb <<= Move::Right(10);
    println!("{}", bb);
}
