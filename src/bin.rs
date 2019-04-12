extern crate bitboard;

use bitboard::*;
use typenum::*;

fn main() {
    use Rotation::*;
    use Move::*;
    
    let bb = BitBoard::<U8>::new(vec![(4, 4)]);
    let n = std::time::Instant::now();
    let queen = bb.moves()  
        .translate(Up(1)).translate(UpRight(1,1))
        .rotate(Clockwise)
        .mirror()
        .repeat(8)
        .collect();
    let done = n.elapsed().as_micros();

    println!("Queen: \n{}", queen);
    println!("Took: {}us", done);
}
