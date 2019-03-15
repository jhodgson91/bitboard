use super::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

pub enum Direction {
    Anticlockwise,
    Clockwise,
}

impl Move {
    fn rotate(self, dir: Direction) -> Self {
        match dir {
            Direction::Clockwise => match self {
                Left(i) => Up(i),
                Up(i) => Right(i),
                Right(i) => Down(i),
                Down(i) => Left(i),
            },
            Direction::Anticlockwise => match self {
                Left(i) => Down(i),
                Down(i) => Right(i),
                Right(i) => Up(i),
                Up(i) => Left(i),
            },
        }
    }

    fn mirror(self) -> Self {
        match self {
            Up(i) => Down(i),
            Down(i) => Up(i),
            Left(i) => Right(i),
            Right(i) => Left(i),
        }
    }
}

pub trait Movable
where
    Self: Sized + Clone,
{
    type Output;

    fn apply_moves(self, moves: Vec<Move>) -> Self::Output;

    fn moves(self) -> MoveGenerator<Self> {
        MoveGenerator {
            to_move: self,
            moves: Vec::new(),
        }
    }
}
impl<N: Unsigned, R: PrimUInt> Movable for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn apply_moves(self, moves: Vec<Move>) -> Self::Output {
        BitBoard::<N, R>::default()
    }
}

pub struct MoveGenerator<T: Movable> {
    to_move: T,
    moves: Vec<Move>,
}

use Move::*;
impl<T: Movable> MoveGenerator<T> {
    pub fn collect(self) -> T::Output {
        self.to_move.apply_moves(self.moves)
    }

    pub fn left(mut self, n: usize) -> Self {
        self.moves.push(Left(n));
        self
    }

    pub fn right(mut self, n: usize) -> Self {
        self.moves.push(Right(n));
        self
    }

    pub fn up(mut self, n: usize) -> Self {
        self.moves.push(Up(n));
        self
    }

    pub fn down(mut self, n: usize) -> Self {
        self.moves.push(Down(n));
        self
    }

    pub fn rotate(mut self, dir: Direction) -> Self {
        match self.moves.last() {
            Some(m) => self.moves.push(m.rotate(dir)),
            None => (),
        }
        self
    }

    pub fn mirror(mut self) -> Self {
        self.moves
            .append(&mut self.moves.iter().map(|m| m.mirror()).collect());
        self
    }
}
