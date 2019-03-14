extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U20, u8>::new(vec![(0, 0)]);
    bb <<= 16;
}
