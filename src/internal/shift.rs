use super::*;

#[derive(Copy, Clone)]
enum Shift {
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

#[derive(Copy, Clone)]
enum EdgeMask {
    Left(usize),
    Right(usize),
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub(super) fn shift(&mut self, m: Move) {
        unsafe {
            match m {
                Move::Left(i) => self.shift_internal(i, Shift::Right, Some(EdgeMask::Right(i))),
                Move::Right(i) => self.shift_internal(i, Shift::Left, Some(EdgeMask::Left(i))),
                Move::Up(i) => self.shift_internal(i * N::USIZE, Shift::Left, None),
                Move::Down(i) => self.shift_internal(i * N::USIZE, Shift::Right, None),
                Move::UpLeft(i) => self.shift_internal(
                    i * N::USIZE - std::cmp::min(i, N::USIZE),
                    Shift::Left,
                    Some(EdgeMask::Right(i)),
                ),
                Move::UpRight(i) => self.shift_internal(
                    i * N::USIZE + std::cmp::min(i, N::USIZE),
                    Shift::Left,
                    Some(EdgeMask::Left(i)),
                ),
                Move::DownLeft(i) => self.shift_internal(
                    i * N::USIZE + std::cmp::min(i, N::USIZE),
                    Shift::Right,
                    Some(EdgeMask::Right(i)),
                ),
                Move::DownRight(i) => self.shift_internal(
                    i * N::USIZE - std::cmp::min(i, N::USIZE),
                    Shift::Right,
                    Some(EdgeMask::Left(i)),
                ),
            }
        }
    }

    unsafe fn shift_internal(&mut self, mut rhs: usize, direction: Shift, mask: Option<EdgeMask>) {
        let edge_masks: Vec<R> = (0..Self::REQUIRED_BLOCKS)
            .into_iter()
            .map(|i| {
                if let Some(m) = mask {
                    Self::edge_mask(m, i)
                } else {
                    R::max_value()
                }
            })
            .collect();

        while rhs > 0 {
            let mut prev_lost = R::zero();

            self.enumerate_blocks(direction, |idx, block| {
                prev_lost = Self::shift_block(direction, rhs, prev_lost, block, edge_masks[idx]);
            });

            rhs -= std::cmp::min(Self::BLOCK_SIZE_BITS, rhs);
        }

        if Self::HAS_BLOCK_MASK {
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

    // Calculates the mask for a block at a given index
    // Works by figuring out the bits that are width away from % N
    fn edge_mask(mask: EdgeMask, block_idx: usize) -> R {
        !(0..Self::BLOCK_SIZE_BITS)
            .into_iter()
            .filter(|i| match mask {
                EdgeMask::Left(width) => {
                    (((Self::BLOCK_SIZE_BITS) * block_idx) + i) % N::USIZE < width
                }
                EdgeMask::Right(width) => {
                    N::USIZE - ((((Self::BLOCK_SIZE_BITS) * block_idx) + i) % N::USIZE) - 1 < width
                }
            })
            .fold(R::zero(), |a, b| a | R::one() << b)
    }
}
