use super::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign};

impl<N: Unsigned, R: PrimUInt> Shl<Move> for &mut BitBoard<N, R> {
    type Output = Self;
    fn shl(self, rhs: Move) -> Self::Output {
        self.shift(rhs);
        self
    }
}

impl<N: Unsigned, R: PrimUInt> Shl<Move> for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn shl(self, rhs: Move) -> Self::Output {
        let mut result = self.clone();
        result.shift(rhs);
        result
    }
}

impl<N: Unsigned, R: PrimUInt> Shl<Move> for BitBoard<N, R> {
    type Output = Self;
    fn shl(mut self, rhs: Move) -> Self::Output {
        self.shift(rhs);
        self
    }
}

impl<N: Unsigned, R: PrimUInt> ShlAssign<Move> for &mut BitBoard<N, R> {
    fn shl_assign(&mut self, rhs: Move) {
        self.shift(rhs);
    }
}

impl<N: Unsigned, R: PrimUInt> ShlAssign<Move> for BitBoard<N, R> {
    fn shl_assign(&mut self, rhs: Move) {
        self.shift(rhs);
    }
}

impl<N: Unsigned, R: PrimUInt> BitAnd for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result
                .block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock &= rblock);
            result
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitAnd for BitBoard<N, R> {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock &= rblock);
            self
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitAndAssign for BitBoard<N, R> {
    fn bitand_assign(&mut self, rhs: Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock &= rblock);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitAndAssign<&Self> for BitBoard<N, R> {
    fn bitand_assign(&mut self, rhs: &Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock &= rblock);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOr for &mut BitBoard<N, R> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
            self
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOr for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result
                .block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
            result
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOr for BitBoard<N, R> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
            self
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOrAssign<&Self> for BitBoard<N, R> {
    fn bitor_assign(&mut self, rhs: &Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitOrAssign for BitBoard<N, R> {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock |= rblock);
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitXor for &BitBoard<N, R> {
    type Output = BitBoard<N, R>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            result
                .block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock ^= rblock);
            result
        }
    }
}

impl<N: Unsigned, R: PrimUInt> BitXorAssign<&Self> for BitBoard<N, R> {
    fn bitxor_assign(&mut self, rhs: &Self) {
        unsafe {
            self.block_iter_mut()
                .zip(rhs.block_iter())
                .for_each(|(lblock, rblock)| *lblock ^= rblock);
        }
    }
}
