use super::{BitBoard, PrimUInt};
use typenum::Unsigned;

impl<'a, N: Unsigned, R: PrimUInt> IntoIterator for &'a BitBoard<N, R> {
    type Item = bool;
    type IntoIter = BitBoardIter<'a, N, R>;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter {
            cell: (0, 0),
            board: self,
        }
    }
}

pub struct BitBoardIter<'a, N: Unsigned, R: PrimUInt = u64> {
    cell: (usize, usize),
    board: &'a BitBoard<N, R>,
}

impl<'a, N: Unsigned, R: PrimUInt> Iterator for BitBoardIter<'a, N, R> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cell.1 >= N::USIZE {
            None
        } else {
            let result = self.board.is_set(self.cell.0, self.cell.1);
            if self.cell.0 < N::USIZE {
                self.cell.0 = (self.cell.0 + 1) % N::USIZE;
                if self.cell.0 == 0 {
                    self.cell.1 += 1;
                }
            }

            Some(result)
        }
    }
}

pub(super) struct BlockIter<R: PrimUInt> {
    start: *const R,
    end: *const R,
}

impl<R: PrimUInt> Iterator for BlockIter<R> {
    type Item = R;

    fn next(&mut self) -> Option<R> {
        unsafe {
            if self.start < self.end {
                self.start = self.start.add(1);
                Some(*self.start.sub(1))
            } else {
                None
            }
        }
    }
}

impl<R: PrimUInt> DoubleEndedIterator for BlockIter<R> {
    fn next_back(&mut self) -> Option<R> {
        unsafe {
            self.end = self.end.sub(1);
            if self.end >= self.start {
                Some(*self.end)
            } else {
                None
            }
        }
    }
}

pub(super) struct BlockIterMut<R: PrimUInt> {
    start: *mut R,
    end: *mut R,
}

impl<R: PrimUInt> Iterator for BlockIterMut<R> {
    type Item = *mut R;

    fn next(&mut self) -> Option<*mut R> {
        unsafe {
            if self.start < self.end {
                self.start = self.start.add(1);
                Some(self.start.sub(1))
            } else {
                None
            }
        }
    }
}

impl<R: PrimUInt> DoubleEndedIterator for BlockIterMut<R> {
    fn next_back(&mut self) -> Option<*mut R> {
        unsafe {
            self.end = self.end.sub(1);
            if self.end >= self.start {
                Some(self.end)
            } else {
                None
            }
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub(super) unsafe fn block_iter(&self) -> BlockIter<R> {
        let start = self.ptr as *const R;
        let end = self.ptr.add(Self::required_blocks()) as *const R;
        BlockIter { start, end }
    }

    pub(super) unsafe fn block_iter_mut(&mut self) -> BlockIterMut<R> {
        let start = self.ptr;
        let end = self.ptr.add(Self::required_blocks());
        BlockIterMut { start, end }
    }
}
