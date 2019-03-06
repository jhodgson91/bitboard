use super::PrimUInt;
use std::alloc::{self, Layout};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem;
use typenum::Unsigned;

pub struct BitBoard<N: Unsigned, R: PrimUInt = u64> {
    pub(super) ptr: *mut R,
    _typenum: PhantomData<N>,
}

impl<N: Unsigned, R: PrimUInt> BitBoard<N, R> {
    pub fn new(initial: Vec<(usize, usize)>) -> Self {
        let mut result = Self::default();
        initial.iter().for_each(|(x, y)| {
            result.set(*x, *y);
        });

        result
    }

    pub fn set(&mut self, x: usize, y: usize) {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            unsafe { *self.block_at_mut(offset) |= bit_pos };
        }
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            unsafe { *self.block_at_mut(offset) &= !bit_pos };
        }
    }

    pub fn is_set(&self, x: usize, y: usize) -> bool {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            unsafe { self.block_at(offset) & bit_pos != R::zero() }
        } else {
            false
        }
    }

    pub fn count_ones(&self) -> usize {
        self.into_iter().map(|b| if b { 1 } else { 0 }).sum()
    }

    fn in_bounds(x: usize, y: usize) -> bool {
        x < N::to_usize() && y < N::to_usize()
    }

    fn map_coords(x: usize, y: usize) -> (isize, R) {
        let pos = x + y * N::to_usize();
        let byte_offset = pos / Self::alignment_bits();
        let bit_pos: usize = 1 << (pos % Self::alignment_bits());

        (byte_offset as isize, R::from(bit_pos).unwrap())
    }

    #[inline(always)]
    pub(super) unsafe fn block_at(&self, i: isize) -> R {
        *self.ptr.offset(i)
    }

    #[inline(always)]
    pub(super) unsafe fn block_at_mut(&mut self, i: isize) -> *mut R {
        self.ptr.offset(i)
    }

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
    pub(super) fn layout() -> Layout {
        Layout::from_size_align(Self::required_bytes(), Self::alignment()).unwrap()
    }
}

impl<N: Unsigned, R: PrimUInt> Default for BitBoard<N, R> {
    fn default() -> Self {
        let layout = Self::layout();
        let ptr;

        unsafe {
            ptr = alloc::alloc_zeroed(layout) as *mut R;
        };

        BitBoard {
            ptr,
            _typenum: PhantomData,
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Drop for BitBoard<N, R> {
    fn drop(&mut self) {
        let layout = Self::layout();
        unsafe { alloc::dealloc(self.ptr as *mut u8, layout) }
    }
}

impl<N: Unsigned, R: PrimUInt> Clone for BitBoard<N, R> {
    fn clone(&self) -> Self {
        let result = BitBoard::<N, R>::default();
        unsafe {
            std::ptr::copy(
                self.ptr as *const u8,
                result.ptr as *mut u8,
                Self::required_bytes(),
            );
        }
        result
    }
}

unsafe impl<N: Unsigned, R: PrimUInt> Send for BitBoard<N, R> {}
unsafe impl<N: Unsigned, R: PrimUInt> Sync for BitBoard<N, R> {}

impl<N: Unsigned, R: PrimUInt> Debug for BitBoard<N, R> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "{s}x{s} BitBoard: ", s = N::to_usize())?;
        writeln!(f, "Size            : {} bits", Self::board_size())?;
        writeln!(f, "Block size      : {}-bit", Self::block_size_bits())?;
        writeln!(f, "Required blocks : {}", Self::required_blocks())?;
        writeln!(f, "Allocated bytes : {}", Self::required_bytes())?;
        writeln!(f, "Allocated bits  : {}", Self::required_bits())?;
        writeln!(f, "Alignment       : {}", Self::alignment())?;
        writeln!(f, "Data            : {:?}", self.ptr)?;
        unsafe {
            self.block_iter().rev().for_each(|block| {
                for i in 0..Self::block_size_bits() {
                    let shift: R = R::one() << (Self::block_size_bits() - i - 1);

                    if block & shift != R::zero() {
                        if write!(f, "1").is_err() {
                            return;
                        }
                    } else if write!(f, "0").is_err() {
                        return;
                    }
                }

                if write!(f, " ").is_err() {
                    return;
                }
            });
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<N: Unsigned, R: PrimUInt> Display for BitBoard<N, R> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = N::to_usize();

        for c in 0..s {
            for r in 0..s {
                if self.is_set(r, s - c - 1) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}
