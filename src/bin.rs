extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U8, u16>::new(vec![(7, 7)]);
    bb = &bb >> 17;
    println!("{}", bb);
    
}
