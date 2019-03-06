extern crate bitboard;

use bitboard::BitBoard;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U2, u128>::new(vec![(0, 0)]);
    bb <<= 10;
    println!("{}", bb);
    dbg!(bb);
}
