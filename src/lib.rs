use std::fmt::{Binary, Debug, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Not, Shl, Shr};
use std::mem::size_of;
use crate::word::Word;
mod word;

#[derive(Clone, Copy, Default)]
pub struct BitMap<W: Word> {
    word: W,
}

impl<W: Word> BitMap<W> {
    const WORD_BITS: usize = size_of::<W>() * 8;
    pub fn new() -> Self {
        Self { word: W::default() }
    }

    #[inline(always)]
    pub fn set_bit_one(&mut self, bit_idx: usize) {
        debug_assert!(bit_idx < Self::WORD_BITS);
        self.word |= W::from(1) << bit_idx;
    }

    #[inline(always)]
    pub fn set_bit_zero(&mut self, bit_idx: usize) {
        debug_assert!(bit_idx < Self::WORD_BITS);
        self.word &= !(W::from(1) << bit_idx);
    }

    #[inline(always)]
    pub fn first_zero_lsb(&self) -> u32 {
        self.word.trailing_ones()
    }

    #[inline(always)]
    pub fn first_one_lsb(&self) -> u32 {
        self.word.trailing_zeros()
    }

    #[inline(always)]
    pub fn first_zero_msb(&self) -> u32 {
        self.word.leading_ones()
    }

    #[inline(always)]
    pub fn first_one_msb(&self) -> u32 {
        self.word.leading_zeros()
    }
}

impl<W: Word> BitAnd for BitMap<W> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { word: self.word & rhs.word }
    }
}

impl<W: Word> BitOr for BitMap<W> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { word: self.word | rhs.word }
    }
}

impl<W: Word> BitXor for BitMap<W> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { word: self.word ^ rhs.word }
    }
}

impl<W: Word> BitAndAssign for BitMap<W> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.word &= rhs.word;
    }
}

impl<W: Word> BitOrAssign for BitMap<W> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.word |= rhs.word;
    }
}

impl<W: Word> BitXorAssign for BitMap<W> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.word ^= rhs.word;
    }
}

impl<W: Word> Not for BitMap<W> {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self { word: !self.word }
    }
}

impl<W: Word> Shl<usize> for BitMap<W> {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self { word: self.word << rhs }
    }
}

impl<W: Word> Shr<usize> for BitMap<W> {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        Self { word: self.word >> rhs }
    }
}

impl<W: Word> Deref for BitMap<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.word
    }
}

impl<W: Word> DerefMut for BitMap<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.word
    }
}

impl<W: Word> From<W> for BitMap<W> {
    fn from(word: W) -> Self {
        Self { word }
    }
}

impl<W: Word> Debug for BitMap<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.word)
    }
}

impl<W: Word> Binary for BitMap<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.word)
    }
}