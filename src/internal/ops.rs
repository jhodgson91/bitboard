use super::{BitBoard, Move, PrimUInt};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign};
use typenum::*;

impl<N: Unsigned, R: PrimUInt> Shl<Move> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shl(self, rhs: Move) -> Self::Output {
        let mut result = self.clone();
        result.shift(rhs);
        result
    }
}

impl<N: Unsigned, R: PrimUInt> ShlAssign<Move> for BitBoard<N, R> {
    fn shl_assign(&mut self, rhs: Move) {
        self.shift(rhs);
    }
}

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
    pub(self) fn shift<R: PrimUInt>(&self, other: R, n: usize) -> R {
        match self {
            Shift::Left => other << n,
            Shift::Right => other >> n,
        }
    }

    pub(self) fn back_shift<R: PrimUInt>(&self, other: R, n: usize) -> R {
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
                Move::Mix(v) => v.iter().for_each(|m| self.shift(m.clone())),
            }
        }
    }

    unsafe fn shift_internal(&mut self, mut rhs: usize, direction: Shift, edge_mask_width: usize) {
        let edge_masks: Vec<R> = (0..Self::REQUIRED_BLOCKS)
            .into_iter()
            .map(|i| Self::edge_mask(direction, edge_mask_width, i))
            .collect();

        while rhs > 0 {
            let mut prev_lost = R::zero();

            self.enumerate_blocks(direction, |idx, block| {
                prev_lost = Self::shift_block(direction, rhs, prev_lost, block, edge_masks[idx]);
            });

            rhs -= std::cmp::min(Self::BLOCK_SIZE_BITS, rhs);
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
                    op(Self::REQUIRED_BLOCKS - i - 1, block);
                }
            }
        }
    }

    // Performs a shift on a single block, returning the bits that would be lost
    unsafe fn shift_block(dir: Shift, by: usize, prev_lost: R, block: *mut R, mask: R) -> R {
        if by >= Self::BLOCK_SIZE_BITS {
            let lost = *block;
            *block = prev_lost;
            if by == Self::BLOCK_SIZE_BITS {
                *block &= mask;
            }
            lost
        } else {
            let lost = dir.back_shift(*block, Self::BLOCK_SIZE_BITS - by);
            *block = dir.shift(*block, by);
            *block |= prev_lost;
            *block &= mask;
            lost
        }
    }

    // Returns the edge mask for a single block based on
    // shift direction. Only used in left/right shifts
    // Remember moving left means shifting right, ie Shift::Left
    // will give you the right edge mask
    fn edge_mask(dir: Shift, width: usize, block_idx: usize) -> R {
        if width >= N::USIZE {
            R::zero()
        } else {
            !(0..Self::BLOCK_SIZE_BITS)
                .into_iter()
                .filter(|i| match dir {
                    Shift::Left => (((Self::BLOCK_SIZE_BITS) * block_idx) + i) % N::USIZE < width,
                    Shift::Right => {
                        N::USIZE - ((((Self::BLOCK_SIZE_BITS) * block_idx) + i) % N::USIZE) - 1
                            < width
                    }
                })
                .fold(R::zero(), |a, b| a | R::one() << b)
        }
    }
}
