extern crate bitboard;

use bitboard::BitBoard;
use typenum::*;

fn main() {
    let bb = BitBoard::<U8, u8>::new(vec![(0, 0)]);
    dbg!(bb);
}
