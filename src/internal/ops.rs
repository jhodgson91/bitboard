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

        // NB: Left shift == sends bits right and up the board ( right now )

        // Up/Down is easy, it will be an exact multiple of N and there's no ambiguity

        // Left/Right depends on the distance away from the nearest N
        // If a user requested a left shift of 7 on an 8x8 board, we can assume
        // they were going up 1, left 1. This breaks down at the boundary though.
        // If a user left shifts 4, they might be trying to go right 4,
        // or up 1 left 4.

        // There's an argument to be made that right 4 is a, in this case,
        // more sensible move, but it would be nice if we could be generic.
        // I also don't think that's the only case. Some are more grey areas

        // What I'd like is a nice front-facing API to that is as easy to use
        // as shifting, but doesn't have these edge-cases, maybe something
        // functional programmingy...

        // This was my attempt at getting edge masking working for the general case,
        // It's not finished, and doesn't try to predict left/right as I realised the problem
        // while coding it...

        let mut remainder = rhs % N::USIZE;
        rhs -= remainder;

        // Move up/down first
        while rhs > remainder {
            let mut prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits() - 1, rhs);
            self.enumerate_blocks(direction, |_idx, block| {
                do_shift(to_shift, &mut prev_lost, block);
            });

            rhs = std::cmp::max(remainder, rhs - to_shift);
        }

        // Anything left here should count as a left/right move,
        // so we mask out all the dirty cheat moves
        while remainder > 0 {
            let mut prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits() - 1, remainder);
            self.enumerate_blocks(direction, |idx, block| {
                do_shift(to_shift, &mut prev_lost, block);
                println!(
                    "{:064b}",
                    Self::edge_mask(direction.other(), remainder, idx)
                );
                *block &= Self::edge_mask(direction.other(), remainder, idx);
            });

            remainder -= to_shift;
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
        width = std::cmp::min(N::USIZE - 1, width);

        println!("Edge width {}", width);
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
