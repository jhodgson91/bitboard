use super::{BitBoard, PrimUInt};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use typenum::*;

enum ShiftDirection {
    Left,
    Right,
}

impl<N: Unsigned, R: PrimUInt> ShlAssign<usize> for BitBoard<N, R> {
    fn shl_assign(&mut self, rhs: usize) {
        unsafe {
            self.shift_internal(rhs, ShiftDirection::Left);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> ShrAssign<usize> for BitBoard<N, R> {
    fn shr_assign(&mut self, rhs: usize) {
        unsafe {
            self.shift_internal(rhs, ShiftDirection::Right);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Shl<usize> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result.shift_internal(rhs, ShiftDirection::Left);
        }
        result
    }
}

impl<N: Unsigned, R: PrimUInt> Shr<usize> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result.shift_internal(rhs, ShiftDirection::Right);
        }
        result
    }
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    unsafe fn shift_internal(&mut self, mut rhs: usize, direction: ShiftDirection) {
        let shift = match direction {
            ShiftDirection::Left => R::shl,
            ShiftDirection::Right => R::shr,
        };

        let back_shift = match direction {
            ShiftDirection::Left => R::shr,
            ShiftDirection::Right => R::shl,
        };

        let op = |to_shift: usize, prev_lost: &mut R, block: *mut R| {
            let lost = back_shift(*block, Self::block_size_bits() - to_shift);

            *block = shift(*block, to_shift);
            *block |= *prev_lost;

            *prev_lost = lost;
        };

        while rhs > 0 {
            let mut prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits() - 1, rhs);
            match direction {
                ShiftDirection::Left => self
                    .block_iter_mut()
                    .for_each(|block| op(to_shift, &mut prev_lost, block)),
                ShiftDirection::Right => self
                    .block_iter_mut()
                    .rev()
                    .for_each(|block| op(to_shift, &mut prev_lost, block)),
            };

            rhs -= to_shift;
        }

        if Self::last_block_mask() != R::zero() {
            if let Some(block) = self.block_iter_mut().last() {
                *block &= Self::last_block_mask();
            }
        }
    }
}
