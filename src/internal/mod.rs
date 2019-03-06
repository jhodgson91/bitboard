use num::PrimInt;
use std::fmt::{Binary, Display};
use std::ops::{BitAndAssign, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};
use typenum::*;

pub type BitBoard3x3 = BitBoard<U3, u16>;
pub type BitBoard4x4 = BitBoard<U4, u16>;
pub type BitBoard5x5 = BitBoard<U5, u32>;
pub type BitBoard6x6 = BitBoard<U6, u64>;
pub type BitBoard7x7 = BitBoard<U7, u64>;
pub type BitBoard8x8 = BitBoard<U8, u64>;

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
impl PrimUInt for u32 {}
impl PrimUInt for u64 {}
impl PrimUInt for u128 {}

mod board;
mod iter;
mod ops;

pub use board::BitBoard;
pub use iter::BitBoardIter;
