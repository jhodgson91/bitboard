use bitboard::*;
use typenum::*;

type ChessBoard = BitBoard<U8, u64>;

fn backtracking_nqueens(last: &ChessBoard, row: usize) -> Option<ChessBoard> {
    let mut result = last.clone();
        
    if row >= ChessBoard::WIDTH {
        return Some(result);
    }

    let moves = |x: usize, y: usize| ChessBoard::make_moves_from(x, y)
            .up(1)
            .upleft(1, 1)
            .rotate(Rotation::Clockwise)
            .repeat(ChessBoard::WIDTH)
            .mirror()
            .collect();

    for col in 0..ChessBoard::WIDTH {
        if (moves(col, row) & last).is_empty() {
            result.set(col, row);
            if let Some(bb) = backtracking_nqueens(&result, row + 1) {
                return Some(bb);
            }
            result.unset(col, row);
        }
    }

    None
}

fn main() {
    use std::time::Instant;

    let t = Instant::now();

    let result = backtracking_nqueens(&ChessBoard::default(), 0);
    let elapsed = t.elapsed().as_millis();
    if let Some(bb) = result {
        println!("{}", bb);
        println!("Took {}ms", elapsed);        
    }
}
