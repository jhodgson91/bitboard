use super::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    Identity,

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
            (Identity, _) => other,
            (_, Identity) => self,
            _ => Identity,
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
            Identity => Identity,
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
            _ => Identity,
        }
    }

    /// Consumes the move and returns component
    /// moves. If performed on a single direction
    /// move, the second part of this tuple will
    /// be identity
    pub fn split(self) -> (Self, Self) {
        match self {
            UpLeft(i, j) => (Up(i), Left(j)),
            UpRight(i, j) => (Up(i), Right(j)),
            DownLeft(i, j) => (Down(i), Left(j)),
            DownRight(i, j) => (Down(i), Right(j)),
            Up(i) => (Up(i), Identity),
            Down(i) => (Down(i), Identity),
            Left(i) => (Left(i), Identity),
            Right(i) => (Right(i), Identity),
            Identity => (Identity, Identity),
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
            Identity => Identity,
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub fn moves(&self) -> Moves<N, R> {
        Moves {
            from: self.clone(),
            moves: Vec::new(),
            combine: false,
        }
    }

    pub fn make_moves_from(x: usize, y: usize) -> Moves<N, R> {
        let mut bb = Self::default();
        bb.set(x, y);
        Moves {
            from: bb,
            moves: Vec::new(),
            combine: true,
        }
    }
}

pub struct Moves<N: Unsigned, R: PrimUInt> {
    from: BitBoard<N, R>,
    moves: Vec<Move>,
    combine: bool,
}

impl<N: Unsigned, R: PrimUInt> Moves<N, R> {
    pub fn new(from: BitBoard<N, R>, combine: bool) -> Self {
        Moves {
            from,
            moves: Vec::new(),
            combine,
        }
    }

    pub fn up(mut self, i: usize) -> Self {
        self.moves.push(Up(i));
        self
    }
    pub fn down(mut self, i: usize) -> Self {
        self.moves.push(Down(i));
        self
    }
    pub fn left(mut self, i: usize) -> Self {
        self.moves.push(Left(i));
        self
    }
    pub fn right(mut self, i: usize) -> Self {
        self.moves.push(Right(i));
        self
    }
    pub fn upleft(mut self, i: usize, j: usize) -> Self {
        self.moves.push(UpLeft(i, j));
        self
    }
    pub fn upright(mut self, i: usize, j: usize) -> Self {
        self.moves.push(UpRight(i, j));
        self
    }
    pub fn downleft(mut self, i: usize, j: usize) -> Self {
        self.moves.push(DownLeft(i, j));
        self
    }
    pub fn downright(mut self, i: usize, j: usize) -> Self {
        self.moves.push(DownRight(i, j));
        self
    }
    pub fn identity(mut self) -> Self {
        self.moves.push(Identity);
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

    pub fn collect(mut self) -> BitBoard<N, R> {
        let mut result = BitBoard::default();
        self.moves.dedup();
        for m in self.moves {
            if self.combine {
                result |= &self.from << m;
            } else {
                result <<= m;
            }
        }
        result
    }
}
