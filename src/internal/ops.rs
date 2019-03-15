use super::{BitBoard, Move, PrimUInt};
use std::ops::{BitAnd, BitOr, Shl, ShlAssign, Shr, ShrAssign};
use typenum::*;

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

impl<N: Unsigned, R: PrimUInt> ShlAssign<usize> for BitBoard<N, R> {
    fn shl_assign(&mut self, rhs: usize) {
        unsafe {
            self.shift_internal(rhs, Shift::Left);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> ShrAssign<usize> for BitBoard<N, R> {
    fn shr_assign(&mut self, rhs: usize) {
        unsafe {
            self.shift_internal(rhs, Shift::Right);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Shl<usize> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result.shift_internal(rhs, Shift::Left);
        }
        result
    }
}

impl<N: Unsigned, R: PrimUInt> Shr<usize> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result.shift_internal(rhs, Shift::Right);
        }
        result
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

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    unsafe fn shift_internal(&mut self, mut rhs: usize, dir: Shift) {
        let do_shift = |to_shift: usize, prev_lost: &mut R, block: *mut R| {
            let lost = dir.back_shift(*block, Self::block_size_bits() - to_shift);

            *block = dir.shift(*block, to_shift);
            *block |= *prev_lost;

            *prev_lost = lost;
        };

        while rhs > 0 {
            let mut prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits() - 1, rhs);
            self.enumerate_blocks(dir, |_idx, block| {
                do_shift(to_shift, &mut prev_lost, block);
            });

            rhs -= to_shift;
        }

        if Self::last_block_mask() != R::zero() {
            if let Some(block) = self.block_iter_mut().last() {
                *block &= Self::last_block_mask();
            }
        }
    }

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

    pub(super) fn edge_mask(dir: Shift, mut width: usize, block_idx: usize) -> R {
        width = std::cmp::min(N::USIZE - 1, width);

        println!("Edge width {}", width);
        !(0..Self::block_size_bits())
            .into_iter()
            .filter(|i| match dir {
                Shift::Right => (((Self::block_size_bits()) * block_idx) + i) % N::USIZE < width,
                Shift::Left => {
                    N::USIZE - ((((Self::block_size_bits()) * block_idx) + i) % N::USIZE) - 1
                        < width
                }
            })
            .fold(R::zero(), |a, b| a | R::one() << b)
    }
}
