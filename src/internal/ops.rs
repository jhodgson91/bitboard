use super::{BitBoard, PrimUInt};
use std::ops::{BitAnd, BitOr, Shl, ShlAssign, Shr, ShrAssign};
use typenum::*;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn other(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl<N: Unsigned, R: PrimUInt> ShlAssign<usize> for BitBoard<N, R> {
    fn shl_assign(&mut self, rhs: usize) {
        unsafe {
            self.shift_internal(rhs, Direction::Left);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> ShrAssign<usize> for BitBoard<N, R> {
    fn shr_assign(&mut self, rhs: usize) {
        unsafe {
            self.shift_internal(rhs, Direction::Right);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Shl<usize> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result.shift_internal(rhs, Direction::Left);
        }
        result
    }
}

impl<N: Unsigned, R: PrimUInt> Shr<usize> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result.shift_internal(rhs, Direction::Right);
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
    unsafe fn shift_internal(&mut self, mut rhs: usize, direction: Direction) {
        let shift = match direction {
            Direction::Left => R::shl,
            Direction::Right => R::shr,
        };

        let back_shift = match direction {
            Direction::Left => R::shr,
            Direction::Right => R::shl,
        };

        let do_shift = |to_shift: usize, prev_lost: &mut R, block: *mut R| {
            let lost = back_shift(*block, Self::block_size_bits() - to_shift);

            *block = shift(*block, to_shift);
            *block |= *prev_lost;

            *prev_lost = lost;
        };

        let remainder = rhs % N::USIZE;
        if rhs > remainder {
            rhs -= remainder;
        }
        // Move up/down first
        while rhs > 0 {
            let mut prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits() - 1, rhs);
            self.enumerate_blocks(direction, |_idx, block| {
                do_shift(to_shift, &mut prev_lost, block);
            });

            rhs -= to_shift;
        }

        // Anything left here should count as a left/right move,
        // so we mask out all the dirty cheat moves
        if remainder > 0 {
            let mut prev_lost = R::zero();

            self.enumerate_blocks(direction, |idx, block| {
                do_shift(remainder, &mut prev_lost, block);
                *block &= Self::edge_mask(direction.other(), remainder, idx);
            });
        }

        if Self::last_block_mask() != R::zero() {
            if let Some(block) = self.block_iter_mut().last() {
                *block &= Self::last_block_mask();
            }
        }
    }

    unsafe fn enumerate_blocks(&mut self, dir: Direction, mut op: impl FnMut(usize, *mut R)) {
        match dir {
            Direction::Left => {
                for (i, block) in self.block_iter_mut().enumerate() {
                    op(i, block);
                }
            }
            Direction::Right => {
                for (i, block) in self.block_iter_mut().rev().enumerate() {
                    op(Self::required_blocks() - i - 1, block);
                }
            }
        }
    }

    pub(super) fn edge_mask(dir: Direction, mut width: usize, block_idx: usize) -> R {
        // Clamp the dir_mask below N::USIZE - 1
        width = std::cmp::min(N::USIZE - 1, width);

        !(0..Self::block_size_bits())
            .into_iter()
            .filter(|i| match dir {
                Direction::Right => {
                    (((Self::block_size_bits()) * block_idx) + i) % N::USIZE < width
                }
                Direction::Left => {
                    N::USIZE - ((((Self::block_size_bits()) * block_idx) + i) % N::USIZE) - 1
                        < width
                }
            })
            .fold(R::zero(), |a, b| a | R::one() << b)
    }

    pub fn dir_mask(dir: Direction, width: usize) -> Self {
        // TODO - allocation here is unnecessary and expensive,
        // this chunk of logic should be done
        // block-by-block during shift_internal
        let mut result = Self::default();
        unsafe {
            for (idx, block) in result.block_iter_mut().enumerate() {
                *block |= Self::edge_mask(dir, width, idx)
            }
        }
        result
    }
}
