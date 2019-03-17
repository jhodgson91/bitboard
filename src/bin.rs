extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let mut bb = BitBoard::<U5, u8>::new(vec![(0, 0)]);
    println!("{}", bb);
    let test = bb.moves().up(2).repeat(2).collect();
    println!("{}", test);
}
