extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U8, u8>::new(vec![(3, 3)]);
    println!("{}", bb);
    let test = bb.moves().left(2).up(1).collect();
    println!("{}", test);
}
