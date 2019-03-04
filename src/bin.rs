extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    let bb = BitBoard::<U20, u16>::new(vec![(0, 0)]);
    println!("{}", bb);
}
