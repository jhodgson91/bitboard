extern crate criterion;
extern crate num;
extern crate typenum;

use num::PrimInt;
use std::alloc::{self, Layout};
use std::fmt::{Binary, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ops::{BitAndAssign, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};
use std::sync::Mutex;
use typenum::*;

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
    + Sized
    + Debug
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

// Essentially the same layout as the original crate, but
// with the introduction of choice of int representation
// This was more for me learning about traits and generics
// than anything else, but it's pretty cool
pub struct BitBoard<N: Unsigned, R: PrimUInt = u64> {
    // TODO - the nice thing here is that it packs the bitboard
    // as tightly as possible, while still allowing for any N.
    // The alternative is a BitVec, but I believe they store
    // more than they need to, and aren't as smart about
    // allowing for different int sizes
    ptr: Mutex<*mut R>,
    _typenum: PhantomData<N>,
}

// IDEA: BitBoardStack
// Tak requires a 3rd dimension. A stack *should* be enough to represent this
//
// Can push/pop/insert/remove at
// Can intersect/union which collapses to a single bitboard ( multi-threaded? )
// Can shift all bitboards in the stack at once ( multi-threaded? )

// TODO: We should expose move_left/move_up stuff here
// The shift operators shouldn't really be used directly
// since you can't do the left/right side masking

// In fact we might want to do away with the operators entirely

// TODO - need to implement a last_block_mask. Currently if you shift
// a bit off the board, but not out of the last block, it's possible to shift
// it back. last_block_mask should be all the bits in the last block valid to our board
// and should be applied in Shl and Shr on the end block
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
            unsafe {
                self.access_block_mut(offset, |block| {
                    *block |= bit_pos;
                });
            }
        }
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            unsafe {
                self.access_block_mut(offset, |block| {
                    *block &= !bit_pos;
                });
            }
        }
    }

    // TODO: return a result/option?
    pub fn is_set(&self, x: usize, y: usize) -> bool {
        if Self::in_bounds(x, y) {
            let (offset, bit_pos) = Self::map_coords(x, y);
            unsafe { self.access_block(offset, |block| *block & bit_pos != R::zero()) }
        } else {
            false
        }
    }

    // TODO - these should return Result<Return, PoisonError> instead of just expecting that it worked
    unsafe fn access_block<Return>(&self, i: isize, op: impl FnOnce(*const R) -> Return) -> Return {
        let block = self.ptr.lock().unwrap();
        op(block.offset(i) as *const R)
    }

    unsafe fn access_block_mut<Return>(
        &mut self,
        i: isize,
        mut op: impl FnMut(*mut R) -> Return,
    ) -> Return {
        let block = self.ptr.lock().unwrap();
        op(block.offset(i))
    }

    fn in_bounds(x: usize, y: usize) -> bool {
        x < N::to_usize() && y < N::to_usize()
    }

    fn map_coords(x: usize, y: usize) -> (isize, R) {
        let pos = x + y * N::to_usize();
        let byte_offset = pos / Self::alignment_bits();
        let bit_pos: usize = 1 << (pos % Self::alignment_bits());

        // TODO: Unwrap here
        (byte_offset as isize, R::from(bit_pos).unwrap())
    }

    /// Total number of bits on the board
    #[inline(always)]
    fn board_size() -> usize {
        // This could technically be compile-time as well, but the trait bounds were a goddam nightmare...
        N::to_usize().pow(2)
    }

    // TODO - Double check this is all actually correct
    // late night coding = bad arithmetic

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

    // It could be possible to thread this, but not sure it's worth it
    // For each loop in the while, we could do threaded shift calculations
    // then apply them all at the end
    unsafe fn shift_internal(&mut self, mut rhs: usize, direction: ShiftDirection) {
        let mut lost: R = R::zero();
        let mut prev_lost: R;

        // Wanted this to be a tuple but the matching wouldn't work
        let shift = match direction {
            ShiftDirection::Left => R::shl,
            ShiftDirection::Right => R::shr,
        };

        let reverse = match direction {
            ShiftDirection::Left => R::shr,
            ShiftDirection::Right => R::shl,
        };

        while rhs > 0 {
            prev_lost = R::zero();

            let to_shift = std::cmp::min(Self::block_size_bits() - 1, rhs);
            for i in 0..(Self::required_blocks() as isize) {
                self.access_block_mut(i, |block| {
                    lost = if to_shift < Self::block_size_bits() {
                        reverse(*block, Self::block_size_bits() - to_shift)
                    } else {
                        *block
                    };

                    *block = shift(*block, to_shift);

                    // Set any bits that were lost from the previous block
                    *block |= prev_lost;

                    prev_lost = lost;
                })
            }

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
            ptr: Mutex::new(ptr),
            _typenum: PhantomData,
        }
    }
}

impl<N: Unsigned, R: PrimUInt> Drop for BitBoard<N, R> {
    fn drop(&mut self) {
        let layout = Self::layout();
        unsafe { alloc::dealloc(*self.ptr.lock().unwrap() as *mut u8, layout) }
    }
}

impl<N: Unsigned, R: PrimUInt> Clone for BitBoard<N, R> {
    fn clone(&self) -> Self {
        let result: BitBoard<N, R> = BitBoard::default();
        unsafe {
            std::ptr::copy(
                *self.ptr.lock().unwrap() as *const R,
                *result.ptr.lock().unwrap(),
                Self::required_bytes(),
            );
        }
        result
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
            result.shift_internal(rhs, ShiftDirection::Right);
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
        unsafe {
            for i in 0..Self::required_blocks() {
                self.access_block(i as isize, |block| {
                    for i in 0..Self::block_size_bits() {
                        let shift = R::from(1 << i).unwrap();
                        if *block & shift != R::zero() {
                            write!(f, "1");
                        } else {
                            write!(f, "0");
                        }
                    }

                    write!(f, " ");
                });
            }
            writeln!(f);
        }

        // TODO - format the data split into block-sized blocks
        Ok(())
    }
}

// TODO - this currently renders out with 0,0 at bottom left
// That seemed sensible, but then shifting left technically moves right
// which is a bit annoying. We probably want to have a clear standard for
// left/right and how the board representation fits in
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

type BitBoard3x3 = BitBoard<U3, u16>;
type BitBoard4x4 = BitBoard<U4, u16>;
type BitBoard5x5 = BitBoard<U5, u32>;
type BitBoard6x6 = BitBoard<U6, u64>;
type BitBoard7x7 = BitBoard<U7, u64>;
type BitBoard8x8 = BitBoard<U8, u64>;

// TODO - A butt-load of tests, especially around the shifting
#[cfg(test)]
mod tests {
    use crate::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Not really a test, just using this for debugging
    // Easiest way to run this is `cargo test -- --nocapture`
    #[test]
    fn it_works() {
        let mut t = BitBoard::<U10, u8>::new(vec![(0, 0)]);
        let shared = Arc::new(Mutex::new(t));

        // Kind of pointless, but just seeing if passing between threads works
        let mut threads = vec![];
        for i in 0..99 {
            let passed = shared.clone();
            threads.push(thread::spawn(move || {
                let bb = &mut *passed.lock().unwrap();
                *bb <<= 1;
            }))
        }

        for thread in threads {
            thread.join();
        }

        println!("{}", *shared.lock().unwrap());
        assert_eq!(shared.lock().unwrap().is_set(9, 9), true);
    }
}
