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
pub enum EdgeMask {
    Left(usize),
    Right(usize),
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub(super) fn shift(&mut self, m: Move) {
        match m {
            Move::Up(i) if i < N::USIZE => self.shift_internal(i * N::USIZE, Shift::Left, None),
            Move::Down(i) if i < N::USIZE => self.shift_internal(i * N::USIZE, Shift::Right, None),
            Move::Left(i) if i < N::USIZE => {
                self.shift_internal(i, Shift::Right, Some(EdgeMask::Right(i)))
            }
            Move::Right(i) if i < N::USIZE => {
                self.shift_internal(i, Shift::Left, Some(EdgeMask::Left(i)))
            }
            Move::UpLeft(u, l) if u < N::USIZE && l < N::USIZE => {
                self.shift_internal(u * N::USIZE - l, Shift::Left, Some(EdgeMask::Right(l)))
            }
            Move::UpRight(u, r) if u < N::USIZE && r < N::USIZE => {
                self.shift_internal(u * N::USIZE + r, Shift::Left, Some(EdgeMask::Left(r)))
            }
            Move::DownLeft(d, l) if d < N::USIZE && l < N::USIZE => {
                self.shift_internal(d * N::USIZE + l, Shift::Right, Some(EdgeMask::Right(l)))
            }
            Move::DownRight(d, r) if d < N::USIZE && r < N::USIZE => {
                self.shift_internal(d * N::USIZE - r, Shift::Right, Some(EdgeMask::Left(r)))
            }
            Move::Identity => (),
            _ => self.reset(),
        }
    }

    fn shift_internal(&mut self, mut rhs: usize, direction: Shift, mask: Option<EdgeMask>) {
        let edge_masks: Option<Vec<R>> = if let Some(m) = mask {
            Some(
                (0..Self::REQUIRED_BLOCKS)
                    .into_iter()
                    .map(|i| Self::edge_mask_internal(m, i))
                    .collect(),
            )
        } else {
            None
        };

        while rhs > 0 {
            let mut prev_lost = R::zero();

            self.enumerate_blocks(direction, |idx, block| {
                let mask = if let Some(v) = &edge_masks {
                    v[idx]
                } else {
                    R::max_value()
                };
                prev_lost = Self::shift_block(direction, rhs, prev_lost, block, mask);
            });

            rhs -= std::cmp::min(Self::BLOCK_SIZE_BITS, rhs);
        }

        if Self::HAS_BLOCK_MASK {
            if let Some(block) = self.blocks.iter_mut().last() {
                *block &= Self::last_block_mask();
            }
        }
    }

    // Convenience function for enumerating the blocks correctly during shifts
    fn enumerate_blocks(&mut self, dir: Shift, mut op: impl FnMut(usize, &mut R)) {
        match dir {
            Shift::Left => {
                for (i, block) in self.blocks.iter_mut().enumerate() {
                    op(i, block);
                }
            }
            Shift::Right => {
                for (i, block) in self.blocks.iter_mut().rev().enumerate() {
                    op(Self::REQUIRED_BLOCKS - i - 1, block);
                }
            }
        }
    }

    // Performs a shift on a single block, returning the bits that would be lost
    fn shift_block(dir: Shift, by: usize, prev_lost: R, block: &mut R, mask: R) -> R {
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
    pub(super) fn edge_mask_internal(mask: EdgeMask, block_idx: usize) -> R {
        let start = Self::BLOCK_SIZE_BITS * block_idx;

        !(0..Self::BLOCK_SIZE_BITS)
            .filter(|i| match mask {
                EdgeMask::Left(width) => (start + i) % N::USIZE < width,
                EdgeMask::Right(width) => N::USIZE - ((start + i) % N::USIZE) - 1 < width,
            })
            .fold(R::zero(), |a, b| a | R::one() << b)
    }

    fn reset(&mut self) {
        self.blocks.iter_mut().for_each(|block| *block = R::zero());
    }
}
