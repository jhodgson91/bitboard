extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U4, u8>::new(vec![(0, 0)]);
    dbg!(&bb);
    println!("{}", bb);
    bb <<= 8;
    println!("{}", bb);
}
