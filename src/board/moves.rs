use super::*;

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

#[derive(Copy, Clone)]
pub enum Rotation {
    Clockwise,
    AntiClockwise,
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
            (NullMove, _) => other,
            (_, NullMove) => self,
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

    pub fn rotate(self, r: Rotation) -> Self {
        use Rotation::*;
        match (self, r) {
            (Up(i), Clockwise) | (Down(i), AntiClockwise) => Right(i),
            (Down(i), Clockwise) | (Up(i), AntiClockwise) => Left(i),
            (Left(i), Clockwise) | (Right(i), AntiClockwise) => Up(i),
            (Right(i), Clockwise) | (Left(i), AntiClockwise) => Down(i),
            (UpLeft(i1, i2), Clockwise) | (DownRight(i1, i2), AntiClockwise) => UpRight(i1, i2),
            (UpRight(i1, i2), Clockwise) | (DownLeft(i1, i2), AntiClockwise) => DownRight(i1, i2),
            (DownLeft(i1, i2), Clockwise) | (UpRight(i1, i2), AntiClockwise) => UpLeft(i1, i2),
            (DownRight(i1, i2), Clockwise) | (UpLeft(i1, i2), AntiClockwise) => DownLeft(i1, i2),
            _ => NullMove,
        }
    }
}

impl std::ops::Mul<usize> for &Move {
    type Output = Move;
    fn mul(self, rhs: usize) -> Self::Output {
        *self * rhs
    }
}

impl std::ops::Mul<usize> for Move {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        match self {
            Up(i) => Up(i * rhs),
            Down(i) => Down(i * rhs),
            Left(i) => Left(i * rhs),
            Right(i) => Right(i * rhs),
            UpLeft(u, l) => UpLeft(u * rhs, l * rhs),
            UpRight(u, r) => UpRight(u * rhs, r * rhs),
            DownLeft(d, l) => DownLeft(d * rhs, l * rhs),
            DownRight(d, r) => DownRight(d * rhs, r * rhs),
            NullMove => NullMove,
        }
    }
}

impl<'a, N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub fn moves(&'a self) -> Moves<'a, N, R> {
        Moves {
            from: self,
            moves: Vec::new(),
        }
    }
}

pub struct Moves<'a, N: Unsigned, R: PrimUInt> {
    from: &'a BitBoard<N, R>,

    moves: Vec<Move>,
}

impl<'a, N: Unsigned, R: PrimUInt> Moves<'a, N, R> {
    pub fn new(from: &'a BitBoard<N, R>) -> Self {
        Moves::<'a, N, R> {
            from,
            moves: Vec::new(),
        }
    }

    pub fn translate(mut self, m: Move) -> Self {
        self.moves.push(m);
        self
    }

    pub fn repeat(mut self, i: usize) -> Self {
        let mut new = Vec::with_capacity(self.moves.len() * i + 1);
        for m in &self.moves {
            (1..=i).into_iter().for_each(|mul| new.push(m * mul));
        }
        new.append(&mut self.moves);
        self.moves = new;
        self
    }

    pub fn rotate(mut self, r: Rotation) -> Self {
        self.moves.reserve(self.moves.len());
        self.moves
            .append(&mut self.moves.iter().map(|m| m.rotate(r)).collect());
        self
    }

    pub fn mirror(mut self) -> Self {
        self.moves.reserve(self.moves.len());
        self.moves
            .append(&mut self.moves.iter().map(|m| m.mirror()).collect());
        self
    }

    pub fn identity(mut self) -> Self {
        self.moves.push(NullMove);
        self
    }

    pub fn collect(self) -> BitBoard<N, R> {
        let mut result = BitBoard::default();
        for m in self.moves {
            result |= self.from << m;
        }
        result
    }
}
