use super::{BitBoard, Move, PrimUInt};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use typenum::*;

impl<N: Unsigned, R: PrimUInt> BitAnd for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result
                .block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock &= rblock);
            result
        }
    }
}
impl<N: Unsigned, R: PrimUInt> BitAndAssign<&Self> for BitBoard<N, R> {
    fn bitand_assign(&mut self, rhs: &Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock &= rblock);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOr for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result
                .block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
            result
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOrAssign<&Self> for BitBoard<N, R> {
    fn bitor_assign(&mut self, rhs: &Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Shift {
    Left,
    Right,
}

impl Shift {
    pub(super) fn other(&self) -> Self {
        match self {
            Shift::Left => Shift::Right,
            Shift::Right => Shift::Left,
        }
    }

    pub(super) fn shift<R: PrimUInt>(&self, other: R, n: usize) -> R {
        match self {
            Shift::Left => other << n,
            Shift::Right => other >> n,
        }
    }

    pub(super) fn back_shift<R: PrimUInt>(&self, other: R, n: usize) -> R {
        match self {
            Shift::Left => other >> n,
            Shift::Right => other << n,
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub(super) fn shift(&mut self, m: Move) {
        unsafe {
            match m {
                Move::Left(i) => self.shift_internal(i, Shift::Right, i),
                Move::Right(i) => self.shift_internal(i, Shift::Left, i),
                Move::Up(i) => self.shift_internal(i * N::USIZE, Shift::Left, 0),
                Move::Down(i) => self.shift_internal(i * N::USIZE, Shift::Right, 0),
            }
        }
    }

    unsafe fn shift_internal(&mut self, mut rhs: usize, dir: Shift, edge_mask_width: usize) {
        while rhs > 0 {
            let mut prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits(), rhs);
            self.enumerate_blocks(dir, |idx, block| {
                let edge_mask = Self::edge_mask(dir.other(), edge_mask_width, idx);
                prev_lost = Self::shift_block(dir, to_shift, prev_lost, block, edge_mask);
            });

            rhs -= to_shift;
        }

        if Self::last_block_mask() != R::zero() {
            if let Some(block) = self.block_iter_mut().last() {
                *block &= Self::last_block_mask();
            }
        }
    }

    // Convenience function for enumerating the blocks correctly during shifts
    unsafe fn enumerate_blocks(&mut self, dir: Shift, mut op: impl FnMut(usize, *mut R)) {
        match dir {
            Shift::Left => {
                for (i, block) in self.block_iter_mut().enumerate() {
                    op(i, block);
                }
            }
            Shift::Right => {
                for (i, block) in self.block_iter_mut().rev().enumerate() {
                    op(Self::required_blocks() - i - 1, block);
                }
            }
        }
    }

    // Performs a shift on a single block, returning the bits that would be lost
    unsafe fn shift_block(dir: Shift, by: usize, prev_lost: R, block: *mut R, mask: R) -> R {
        if by >= Self::block_size_bits() {
            let lost = *block;
            *block = prev_lost;
            lost
        } else {
            let lost = dir.back_shift(*block, Self::block_size_bits() - by);
            *block = dir.shift(*block, by);
            *block |= prev_lost;
            *block &= mask;
            lost
        }
    }

    // Returns the edge mask for a single block based on
    // shift direction. Only used in left/right shifts
    fn edge_mask(dir: Shift, width: usize, block_idx: usize) -> R {
        if width >= N::USIZE {
            R::zero()
        } else {
            !(0..Self::block_size_bits())
                .into_iter()
                .filter(|i| match dir {
                    Shift::Right => {
                        (((Self::block_size_bits()) * block_idx) + i) % N::USIZE < width
                    }
                    Shift::Left => {
                        N::USIZE - ((((Self::block_size_bits()) * block_idx) + i) % N::USIZE) - 1
                            < width
                    }
                })
                .fold(R::zero(), |a, b| a | R::one() << b)
        }
    }
}
