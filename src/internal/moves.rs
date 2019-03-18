use super::*;

#[derive(Clone, Eq, PartialEq)]
pub enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    Mix(Vec<Move>),
}

#[derive(Clone, Copy, Eq, PartialEq)]
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
                Mix(v) => Mix(v.iter().map(|m| m.clone().rotate(dir)).collect()),
            },
            Direction::Anticlockwise => match self {
                Left(i) => Down(i),
                Down(i) => Right(i),
                Right(i) => Up(i),
                Up(i) => Left(i),
                Mix(v) => Mix(v.iter().map(|m| m.clone().rotate(dir)).collect()),
            },
        }
    }

    fn mirror(self) -> Self {
        match self {
            Up(i) => Down(i),
            Down(i) => Up(i),
            Left(i) => Right(i),
            Right(i) => Left(i),
            Mix(v) => Mix(v.iter().map(|m| m.clone().mirror()).collect()),
        }
    }
}

pub trait Movable
where
    Self: Sized + Clone,
{
    type Output;

    fn run_moves(self, moves: Vec<Move>) -> Self::Output;
    fn combine_moves(self, moves: Vec<Move>) -> Self::Output;

    fn moves(self) -> MoveGenerator<Self> {
        MoveGenerator {
            to_move: self,
            moves: Vec::new(),
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Movable for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn combine_moves(self, moves: Vec<Move>) -> Self::Output {
        let mut result = self.clone();
        for m in moves {
            result.shift(m);
            result |= self;
        }
        result
    }

    fn run_moves(self, moves: Vec<Move>) -> Self::Output {
        let mut result = self.clone();
        for m in moves {
            result.shift(m);
        }
        result
    }
}

pub struct MoveGenerator<T> {
    to_move: T,
    moves: Vec<Move>,
}

use Move::*;
impl<T: Movable> MoveGenerator<T> {
    pub fn run(self) -> T::Output {
        self.to_move.run_moves(self.moves)
    }

    pub fn collect(self) -> T::Output {
        self.to_move.combine_moves(self.moves)
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

    pub fn combine(mut self) -> Self {
        self.moves = vec![Mix(self.moves.clone())];
        self
    }

    pub fn repeat(mut self, n: usize) -> Self {
        if let Some(m) = self.moves.pop() {
            for _i in 0..n {
                self.moves.push(m.clone());
            }
        }
        self
    }

    pub fn rotate(mut self, dir: Direction) -> Self {
        self.moves
            .append(&mut self.moves.iter().map(|m| m.clone().rotate(dir)).collect());
        self
    }

    pub fn mirror(mut self) -> Self {
        self.moves
            .append(&mut self.moves.iter().map(|m| m.clone().mirror()).collect());
        self
    }
}
