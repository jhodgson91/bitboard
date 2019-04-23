use bitboard::*;
use typenum::*;

type ChessBoard = BitBoard<U100, u128>;

use std::time::Instant;
fn main() {
    use Rotation::*;
    let t = Instant::now();

    let moves = ChessBoard::make_moves_from(8, 8)
        .up(1)
        .upleft(1, 1)
        .rotate(Clockwise)
        .repeat(16)
        .mirror()
        .collect();
    let elapsed = t.elapsed().as_micros();
    dbg!(moves);
    println!("Took {}us", elapsed);
}
