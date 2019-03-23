extern crate bitboard;

use bitboard::*;
use typenum::*;

const SIZE: usize = 20;
type TestBoard = BitBoard<U20, u8>;

fn main() {
    let init = (0..SIZE).into_iter().map(|i| (19, i)).collect();
    let mut bb = TestBoard::new(init);
    println!("{}", bb);
    bb = &bb << Move::Left(20);
    println!("{}", bb);
}
