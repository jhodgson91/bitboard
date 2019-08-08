use super::*;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use typenum::Unsigned;

pub struct BitBoard<N: Unsigned, R: PrimUInt = u64> {
    pub(super) blocks: Vec<R>,
    _typenum: PhantomData<N>,
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub fn new(initial: Vec<(usize, usize)>) -> Self {
        let mut result = Self::default();
        initial.iter().for_each(|(x, y)| {
            result.set(*x, *y);
        });

        result
    }

    pub fn set(&mut self, x: usize, y: usize) {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            self.blocks[offset] |= bit_pos;
        }
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            self.blocks[offset] &= !bit_pos;
        }
    }

    pub fn is_set(&self, (x, y): (usize, usize)) -> bool {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            self.blocks[offset] & bit_pos != R::zero()
        } else {
            false
        }
    }

    pub fn count_ones(&self) -> usize {
        self.blocks.iter().map(|b| b.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        !self.blocks.iter().any(|r| *r != R::zero())
    }

    pub fn edge_mask(mask: EdgeMask) -> Self {
        let mut result = Self::default();
        for (i, block) in result.blocks.iter_mut().enumerate() {
            *block |= Self::edge_mask_internal(mask, i);
        }
        result
    }

    pub fn cells(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> {
        (0..N::USIZE).flat_map(|x| (0..N::USIZE).map(move |y| (x, y)))
    }

    fn in_bounds(x: usize, y: usize) -> bool {
        x < N::USIZE && y < N::USIZE
    }

    fn map_coords(x: usize, y: usize) -> (usize, R) {
        let pos = x + y * N::USIZE;
        let offset = pos / Self::BLOCK_SIZE_BITS;
        let bit_pos: R = R::one() << (pos % Self::BLOCK_SIZE_BITS);

        (offset, bit_pos)
    }
}

impl<N: Unsigned, R: PrimUInt> Default for BitBoard<N, R> {
    fn default() -> Self {
        BitBoard {
            blocks: vec![R::zero(); Self::REQUIRED_BLOCKS],
            _typenum: PhantomData,
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Clone for BitBoard<N, R> {
    fn clone(&self) -> Self {
        BitBoard {
            blocks: self.blocks.clone(),
            _typenum: PhantomData::<N>,
        }
    }
}

unsafe impl<N: Unsigned, R: PrimUInt> Send for BitBoard<N, R> {}
unsafe impl<N: Unsigned, R: PrimUInt> Sync for BitBoard<N, R> {}

impl<N: Unsigned, R: PrimUInt> Debug for BitBoard<N, R> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "{s}x{s} BitBoard: ", s = N::USIZE)?;
        writeln!(f, "Size            : {} bits", Self::BOARD_SIZE)?;
        writeln!(f, "Block size      : {}-bit", Self::BLOCK_SIZE_BITS)?;
        writeln!(f, "Required blocks : {}", Self::REQUIRED_BLOCKS)?;
        writeln!(f, "Last Block Mask : {:b}", Self::last_block_mask())?;

        Ok(())
    }
}

impl<N: Unsigned, R: PrimUInt> Display for BitBoard<N, R> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = N::USIZE;

        for c in 0..s {
            for r in 0..s {
                if self.is_set((r, s - c - 1)) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}
