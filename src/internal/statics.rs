use super::*;
use std::alloc::Layout;
use std::mem;

const fn required_bits(board_size: usize, alignment_bits: usize) -> usize {
    let remainder = board_size % alignment_bits;
    let mul = (remainder != 0) as usize;
    board_size + (alignment_bits - remainder) * mul
}

const fn required_bytes(required_bits: usize) -> usize {
    let t = (required_bits % 8) != 0;
    required_bits / 8 + (t as usize)
}

const fn required_blocks(bytes: usize, alignment: usize) -> usize {
    let t = (bytes % alignment != 0) as usize;
    (bytes / alignment) + t
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub const BOARD_SIZE: usize = N::USIZE * N::USIZE;

    pub const BLOCK_SIZE: usize = mem::size_of::<R>();
    pub const BLOCK_SIZE_BITS: usize = Self::BLOCK_SIZE * 8;

    pub const ALIGNMENT: usize = mem::align_of::<R>();
    pub const ALIGNMENT_BITS: usize = Self::ALIGNMENT * 8;

    pub const REQUIRED_BITS: usize = required_bits(Self::BOARD_SIZE, Self::ALIGNMENT_BITS);
    pub const REQUIRED_BYTES: usize = required_bytes(Self::REQUIRED_BITS);
    pub const REQUIRED_BLOCKS: usize = required_blocks(Self::REQUIRED_BYTES, Self::ALIGNMENT);

    #[inline(always)]
    pub(super) fn last_block_mask() -> R {
        let remainder = Self::BOARD_SIZE % Self::BLOCK_SIZE_BITS;
        match remainder {
            0 => R::zero(),
            _ => (R::one() + R::one()).pow(remainder as u32) - R::one(),
        }
    }

    #[inline(always)]
    pub(super) fn layout() -> Layout {
        Layout::from_size_align(Self::REQUIRED_BYTES, Self::ALIGNMENT).unwrap()
    }
}
