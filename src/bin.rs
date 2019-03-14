extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U8, u64>::new(vec![(0, 0)]);

    bb <<= 55;
    println!("{}", bb);
}
