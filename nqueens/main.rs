extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
        use Rotation::*;
        use Move::*;

        let mut bb = BitBoard::<U8>::new(vec![(4, 4)]);
        let n = std::time::Instant::now();
        let knight = bb.moves()
                .translate(UpRight(2,1))
                .translate(UpRight(1,2))
                .rotate(Clockwise)
                .mirror()
                .collect();
        let queen = bb.moves()
                .translate(Up(1)).translate(UpRight(1,1))
                .rotate(Clockwise)
                .mirror()
                .repeat(8)
                .collect() | bb;
        let done = n.elapsed().as_micros();

        for coord in queen.positions(true) {
                println!("{:?}", coord);
        }
        println!("{}", queen);
        println!("Took: {}us", done);
}