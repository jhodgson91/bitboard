extern crate bitboard;

use bitboard::BitBoard;
use typenum::*;

const SIZE: usize = 8;

fn main() {
    let mut bb = BitBoard::<U5, u64>::new(vec![(3, 3)]);
    dbg!(&bb);
    println!("{}", bb);
    bb <<= 1;
    println!("{}", bb);
    let before = bb.count_ones();
    bb >>= 1;
    println!("{}", bb);
}
