use super::*;
use std::alloc::Layout;
use std::mem;

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    /// Total number of bits on the board
    #[inline(always)]
    pub(super) fn board_size() -> usize {
        N::USIZE * N::USIZE
    }

    /// Total number of bits necessary to represent this bitboard
    /// Properly aligned the alignment of R
    #[inline(always)]
    pub(super) fn required_bits() -> usize {
        let remainder = Self::board_size() % Self::alignment_bits();
        match remainder {
            0 => Self::board_size(),
            _ => Self::board_size() + Self::alignment_bits() - remainder,
        }
    }

    /// Total number of bytes necessary to represent this BitBoard
    #[inline(always)]
    pub(super) fn required_bytes() -> usize {
        (Self::required_bits() as f32 / 8.0).ceil() as usize
    }

    /// Total number of blocks ( R sized memory chunks ) necessary to reperesent this BitBoard
    #[inline(always)]
    pub(super) fn required_blocks() -> usize {
        (Self::required_bytes() as f32 / Self::alignment() as f32).ceil() as usize
    }

    #[inline(always)]
    pub(super) fn alignment() -> usize {
        mem::align_of::<R>()
    }

    #[inline(always)]
    pub(super) fn alignment_bits() -> usize {
        Self::alignment() * 8
    }

    #[inline(always)]
    pub(super) fn block_size() -> usize {
        mem::size_of::<R>()
    }

    // Number of bits in a single block
    #[inline(always)]
    pub(super) fn block_size_bits() -> usize {
        Self::block_size() * 8
    }

    #[inline(always)]
    pub(super) fn last_block_mask() -> R {
        let remainder = Self::board_size() % Self::block_size_bits();
        match remainder {
            0 => R::zero(),
            _ => (R::one() + R::one()).pow(remainder as u32) - R::one(),
        }
    }

    #[inline(always)]
    pub(super) fn layout() -> Layout {
        Layout::from_size_align(Self::required_bytes(), Self::alignment()).unwrap()
    }
}
