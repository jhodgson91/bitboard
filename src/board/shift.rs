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
            _ => self.blocks.iter_mut().for_each(|block| *block = R::zero()),
        }
    }

    fn shift_internal(&mut self, mut rhs: usize, direction: Shift, mask: Option<EdgeMask>) {
        while rhs > 0 {
            let mut prev_lost = R::zero();

            match direction {
                Shift::Left => self.blocks.iter_mut().for_each(|block| {
                    prev_lost = Self::shift_block(direction, rhs, prev_lost, block)
                }),
                Shift::Right => self.blocks.iter_mut().rev().for_each(|block| {
                    prev_lost = Self::shift_block(direction, rhs, prev_lost, block)
                }),
            };

            rhs -= std::cmp::min(Self::BLOCK_SIZE_BITS, rhs);
        }

        if let Some(m) = mask {
            self.blocks
                .iter_mut()
                .enumerate()
                .for_each(|(i, block)| *block &= Self::edge_mask_internal(m, i));
        }

        if Self::HAS_BLOCK_MASK {
            if let Some(block) = self.blocks.iter_mut().last() {
                *block &= Self::last_block_mask();
            }
        }
    }

    // Performs a shift on a single block, returning the bits that would be lost
    fn shift_block(dir: Shift, by: usize, prev_lost: R, block: &mut R) -> R {
        if by >= Self::BLOCK_SIZE_BITS {
            let lost = *block;
            *block = prev_lost;
            lost
        } else {
            let lost = dir.back_shift(*block, Self::BLOCK_SIZE_BITS - by);
            *block = dir.shift(*block, by);
            *block |= prev_lost;
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
}
