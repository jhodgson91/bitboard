extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U8>::new(vec![(2,4)]);
    
    println!("{}", bb);
    bb <<= Move::UpLeft(20);
    println!("{}", bb);
}
