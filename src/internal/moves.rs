#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    NullMove,
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),

    UpLeft(usize, usize),
    UpRight(usize, usize),
    DownLeft(usize, usize),
    DownRight(usize, usize),
}

use Move::*;
impl Move {
    pub fn combine(self, other: Move) -> Self {
        match (self, other) {
            (Up(u), Left(l)) | (Left(l), Up(u)) => UpLeft(u, l),
            (Up(u), Right(r)) | (Right(r), Up(u)) => UpRight(u, r),
            (Down(d), Left(l)) | (Left(l), Down(d)) => DownLeft(d, l),
            (Down(d), Right(r)) | (Right(r), Down(d)) => DownRight(d, r),
            (Up(i1), Up(i2)) => Up(i1 + i2),
            (Down(i1), Down(i2)) => Down(i1 + i2),
            (Left(i1), Left(i2)) => Left(i1 + i2),
            (Right(i1), Right(i2)) => Right(i1 + i2),
            _ => NullMove,
        }
    }

    pub fn mirror(self) -> Self {
        match self {
            Up(i) => Down(i),
            Down(i) => Up(i),
            Left(i) => Right(i),
            Right(i) => Left(i),
            UpLeft(u, l) => DownRight(u, l),
            UpRight(u, r) => DownLeft(u, r),
            DownRight(d, r) => UpLeft(d, r),
            DownLeft(d, l) => UpRight(d, l),
            NullMove => NullMove,
        }
    }
}
