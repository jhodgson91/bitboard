use super::*;

pub struct BitBoardIter<'a, N: Unsigned, R: PrimUInt> {
    board: &'a BitBoard<N, R>,
    set: bool,
    current: usize,
}

impl<'a, N: Unsigned, R: PrimUInt> Iterator for BitBoardIter<'a, N, R> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.current < BitBoard::<N, R>::BOARD_SIZE {
            let coord = (self.current % N::USIZE, self.current / N::USIZE);
            let valid = self.board.is_set(coord.0, coord.1) == self.set;
            self.current += 1;
            if valid {
                return Some(coord);
            }
        }
        None
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
    pub fn positions(&self, set: bool) -> BitBoardIter<N, R> {
        BitBoardIter {
            board: self,
            set,
            current: 0,
        }
    }

    pub(super) unsafe fn block_iter(&self) -> BlockIter<R> {
        let start = self.ptr as *const R;
        let end = self.ptr.add(Self::REQUIRED_BLOCKS) as *const R;
        BlockIter { start, end }
    }

    pub(super) unsafe fn block_iter_mut(&mut self) -> BlockIterMut<R> {
        let start = self.ptr;
        let end = self.ptr.add(Self::REQUIRED_BLOCKS);
        BlockIterMut { start, end }
    }
}
