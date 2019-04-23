use super::*;
use std::mem;

const fn required_bits(board_size: usize, block_size_bits: usize) -> usize {
    let remainder = board_size % block_size_bits;
    let mul = (remainder != 0) as usize;
    board_size + (block_size_bits - remainder) * mul
}

const fn required_blocks(total_bits: usize, block_size_bits: usize) -> usize {
    let t = (total_bits % block_size_bits != 0) as usize;
    (total_bits / block_size_bits) + t
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub const BOARD_SIZE: usize = N::USIZE * N::USIZE;

    pub const BLOCK_SIZE_BITS: usize = mem::size_of::<R>() * 8;

    pub const REQUIRED_BITS: usize = required_bits(Self::BOARD_SIZE, Self::BLOCK_SIZE_BITS);
    pub const REQUIRED_BLOCKS: usize = required_blocks(Self::REQUIRED_BITS, Self::BLOCK_SIZE_BITS);

    pub const HAS_BLOCK_MASK: bool = Self::BOARD_SIZE % Self::BLOCK_SIZE_BITS != 0;

    #[inline(always)]
    pub(super) fn last_block_mask() -> R {
        let remainder = Self::BOARD_SIZE % Self::BLOCK_SIZE_BITS;
        match remainder {
            0 => R::zero(),
            _ => (R::one() << remainder) - R::one(),
        }
    }
}
