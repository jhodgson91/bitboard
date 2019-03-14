extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    println!(
        "{}",
        BitBoard::<U10, u32>::dir_mask(ShiftDirection::Left, 3)
    );
}
