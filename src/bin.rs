extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    use Rotation::*;
    use Move::*;
    
    let mut bb = BitBoard::<U8>::new(vec![(4, 0)]);
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
            .collect();

    let skip = bb.moves().translate(Up(2)).repeat(2).collect();
    println!("Took: {}us\n", n.elapsed().as_micros());

    println!("Knight: \n{}", knight);
    println!("Queen: \n{}", queen);
    println!("Skip: \n{}", skip);
}
