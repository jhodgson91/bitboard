extern crate bitboard;

use bitboard::*;
use typenum::*;


fn main() {
    let bb = BitBoard::<U8>::default();
    println!("{}", bb);
}
