extern crate bitboard;

use bitboard::BitBoard;
use typenum::*;

fn main() {
    let bb = BitBoard::<U4, u64>::new(vec![(3, 3)]);
    println!("{}", bb);
    dbg!(bb);
}
