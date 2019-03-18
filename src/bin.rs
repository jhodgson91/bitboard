extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U8, u8>::new(vec![(3, 3)]);
    println!("{}", bb);
    let test = bb.moves().up(1).left(2).combine().mirror().collect();
    println!("{}", test);
}
