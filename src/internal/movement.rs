use super::*;

pub struct MoveGenerator<'a, N: Unsigned, R: PrimUInt> {
    bb: &'a BitBoard<N, R>,
}

pub struct Left<T: Move> {
    before: T,
    next: usize,
}
pub struct Right<T: Move> {
    before: T,
    next: usize,
}
pub struct Up<T: Move> {
    before: T,
    next: usize,
}
pub struct Down<T: Move> {
    before: T,
    next: usize,
}
pub struct Repeat<T: Move> {
    before: T,
    count: usize,
}
pub struct Mirror<T: Move> {
    before: T,
}

pub trait Move
where
    Self: Sized,
{
    fn left(self, next: usize) -> Left<Self> {
        Left { before: self, next }
    }
    fn right(self, next: usize) -> Right<Self> {
        Right { before: self, next }
    }
    fn up(self, next: usize) -> Up<Self> {
        Up { before: self, next }
    }
    fn down(self, next: usize) -> Down<Self> {
        Down { before: self, next }
    }
    fn repeat(self, count: usize) -> Repeat<Self> {
        Repeat {
            before: self,
            count,
        }
    }
    fn mirror(self) -> Mirror<Self>;
}
