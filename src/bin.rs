extern crate bitboard;

use bitboard::*;
use typenum::*;

type RealLife = BitBoard<U8, u64>;

fn queen_moves(from: (usize, usize)) -> RealLife {
    let mut bb = RealLife::new(vec![from]);
    let bb_ref = &bb;

    let mut moves = RealLife::default();
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Left(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Right(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Up(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::Down(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::UpLeft(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::DownRight(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::UpRight(i));
    (0..8)
        .into_iter()
        .for_each(|i| moves |= bb_ref << Move::DownLeft(i));
    moves ^= bb_ref;
    moves
}

fn main() {
    let t = std::time::Instant::now();
    let knight = RealLife::new(vec![(3,5)]);
    let moves = queen_moves((4,4));
    println!("{}us", t.elapsed().as_micros());

    println!("{}", moves);
    println!("{}", knight);

}
