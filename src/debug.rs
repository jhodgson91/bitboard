use bitboard::*;
use typenum::*;

type ChessBoard = BitBoard<U8>;

use std::time::{Instant, Duration};
fn main() {
    use Rotation::*;
    let t = Instant::now();

    let moves = ChessBoard::make_moves_from(4, 4)
        .up(1)
        .upleft(1, 1)
        .rotate(Clockwise)
        .repeat(8)
        .mirror()
        .collect();
    let elapsed = t.elapsed().as_micros();

    println!("Took {}us", elapsed);
}
