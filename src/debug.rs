use bitboard::*;
use typenum::*;

type ChessBoard = BitBoard<U8>;

fn main() {
    use Rotation::*;
    let moves = ChessBoard::make_moves_from(4, 4)
        .up(1)
        .upleft(1, 1)
        .rotate(Clockwise)
        .repeat(8)
        .mirror()
        .collect();

    println!("{}", moves);
}
