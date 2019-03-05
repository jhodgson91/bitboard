extern crate criterion;
extern crate num;
extern crate typenum;

use num::PrimInt;
use std::alloc::{self, Layout};
use std::fmt::{Binary, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ops::{BitAndAssign, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};
use typenum::*;

#[cfg(test)]
mod tests;

pub trait PrimUInt:
    PrimInt
    + num::Unsigned
    + num::Zero
    + BitAndAssign
    + BitOrAssign
    + Display
    + Binary
    + Shl
    + Shr
    + ShlAssign
    + ShrAssign
{
}

impl PrimUInt for u8 {}
impl PrimUInt for u16 {}
impl PrimUInt for u64 {}
impl PrimUInt for u32 {}

enum ShiftDirection {
    Left,
    Right,
}

pub struct BitBoard<N: Unsigned, R: PrimUInt = u64> {
    ptr: *mut R,
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
        self.into_iter()
            .map(|b| if b { 1 } else { 0 })
            .fold(0, |a, b| a + b)
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

    unsafe fn for_each_block(&self, op: impl Fn(R)) {
        for i in 0..Self::required_blocks() {
            op(self.block_at(i as isize));
        }
    }

    // Should implement a DoubleEndedIterator for block iteration
    unsafe fn for_each_block_mut(&mut self, mut op: impl FnMut(*mut R), reverse: bool) {
        for i in 0..Self::required_blocks() {
            let at = if reverse {
                Self::required_blocks() - i - 1
            } else {
                i
            } as isize;
            op(self.block_at_mut(at));
        }
    }

    #[inline(always)]
    unsafe fn block_at(&self, i: isize) -> R {
        *self.ptr.offset(i)
    }

    #[inline(always)]
    unsafe fn block_at_mut(&mut self, i: isize) -> *mut R {
        self.ptr.offset(i)
    }

    /// Total number of bits on the board
    #[inline(always)]
    fn board_size() -> usize {
        N::USIZE * N::USIZE
    }

    /// Total number of bits necessary to represent this bitboard
    /// Properly aligned the alignment of R
    #[inline(always)]
    fn required_bits() -> usize {
        let remainder = Self::board_size() % Self::alignment_bits();
        match remainder {
            0 => Self::board_size(),
            _ => Self::board_size() + Self::alignment_bits() - remainder,
        }
    }

    /// Total number of bytes necessary to represent this BitBoard
    #[inline(always)]
    fn required_bytes() -> usize {
        (Self::required_bits() as f32 / 8.0).ceil() as usize
    }

    /// Total number of blocks ( R sized memory chunks ) necessary to reperesent this BitBoard
    #[inline(always)]
    fn required_blocks() -> usize {
        (Self::required_bytes() as f32 / Self::alignment() as f32).ceil() as usize
    }

    #[inline(always)]
    fn alignment() -> usize {
        mem::align_of::<R>()
    }

    #[inline(always)]
    fn alignment_bits() -> usize {
        Self::alignment() * 8
    }

    #[inline(always)]
    fn block_size() -> usize {
        mem::size_of::<R>()
    }

    // Number of bits in a single block
    #[inline(always)]
    fn block_size_bits() -> usize {
        Self::block_size() * 8
    }

    unsafe fn shift_internal(&mut self, mut rhs: usize, direction: ShiftDirection) {
        let mut lost: R = R::zero();
        let mut prev_lost: R;

        let shift = match direction {
            ShiftDirection::Left => R::shl,
            ShiftDirection::Right => R::shr,
        };

        let back_shift = match direction {
            ShiftDirection::Left => R::shr,
            ShiftDirection::Right => R::shl,
        };

        let should_reverse = match direction {
            ShiftDirection::Left => false,
            ShiftDirection::Right => true,
        };

        while rhs > 0 {
            prev_lost = R::zero();
            let to_shift = std::cmp::min(Self::block_size_bits() - 1, rhs);
            self.for_each_block_mut(
                |block| {
                    // lost bits are either everything in
                    // this block if shift is larger than bit
                    // size or the reverse of the shift
                    lost = if to_shift < Self::block_size_bits() {
                        back_shift(*block, Self::block_size_bits() - to_shift)
                    } else {
                        *block
                    };

                    *block = shift(*block, to_shift);

                    // Set any bits that were lost from the previous block
                    *block |= prev_lost;

                    prev_lost = lost;
                },
                should_reverse,
            );

            rhs -= to_shift;
        }
    }

    #[inline(always)]
    fn layout() -> Layout {
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

impl<'a, N: Unsigned, R: PrimUInt> IntoIterator for &'a BitBoard<N, R> {
    type Item = bool;
    type IntoIter = BitBoardIter<'a, N, R>;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter {
            cell: (0, 0),
            board: self,
        }
    }
}

pub struct BitBoardIter<'a, N: Unsigned, R: PrimUInt = u64> {
    cell: (usize, usize),
    board: &'a BitBoard<N, R>,
}

impl<'a, N: Unsigned, R: PrimUInt> Iterator for BitBoardIter<'a, N, R> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cell.1 >= N::USIZE {
            None
        } else {
            let result = self.board.is_set(self.cell.0, self.cell.1);
            if self.cell.0 < N::USIZE {
                self.cell.0 = (self.cell.0 + 1) % N::USIZE;
                if self.cell.0 == 0 {
                    self.cell.1 += 1;
                }
            }

            Some(result)
        }
    }
}

unsafe impl<N: Unsigned, R: PrimUInt> Send for BitBoard<N, R> {}
unsafe impl<N: Unsigned, R: PrimUInt> Sync for BitBoard<N, R> {}

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
            for i in 0..Self::required_blocks() {
                let block = self.block_at(i as isize);

                for i in 0..Self::block_size_bits() {
                    let shift: R = R::one() << (Self::block_size_bits() - 1 - i);

                    if block & shift != R::zero() {
                        write!(f, "1")?;
                    } else {
                        write!(f, "0")?;
                    }
                }

                write!(f, " ")?;
            }
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

pub type BitBoard3x3 = BitBoard<U3, u16>;
pub type BitBoard4x4 = BitBoard<U4, u16>;
pub type BitBoard5x5 = BitBoard<U5, u32>;
pub type BitBoard6x6 = BitBoard<U6, u64>;
pub type BitBoard7x7 = BitBoard<U7, u64>;
pub type BitBoard8x8 = BitBoard<U8, u64>;
