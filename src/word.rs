use std::fmt::{Binary, Debug, Display};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

pub trait Word:
    Default
    + Copy
    + Debug
    + Display
    + Binary
    + WordOps
    + Eq
    + PartialEq
    + BitOr<Output = Self>
    + BitOrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<usize>
{
}

impl<T> Word for T where
    T: Default
        + Copy
        + Debug
        + Display
        + Binary
        + WordOps
        + Eq
        + PartialEq
        + BitOr<Output = Self>
        + BitOrAssign
        + BitAnd<Output = Self>
        + BitAndAssign
        + BitXor<Output = Self>
        + BitXorAssign
        + Not<Output = Self>
        + Shl<usize, Output = Self>
        + Shr<usize, Output = Self>
        + From<u8>
        + From<u16>
        + From<u32>
        + From<u64>
        + From<usize>
{
}

pub trait WordOps {
    const MAX: Self;
    const MIN: Self;
    const ONE: Self;
    fn trailing_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_ones(self) -> u32;
    fn leading_ones(self) -> u32;
}

impl WordOps for u8 {
    const MAX: Self = u8::MAX;
    const MIN: Self = u8::MIN;
    const ONE: Self = 1;
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    #[inline(always)]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    #[inline(always)]
    fn trailing_ones(self) -> u32 {
        self.trailing_ones()
    }

    #[inline(always)]
    fn leading_ones(self) -> u32 {
        self.leading_ones()
    }
}

impl WordOps for u16 {
    const MAX: Self = u16::MAX;
    const MIN: Self = u16::MIN;
    const ONE: Self = 1;
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    #[inline(always)]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    #[inline(always)]
    fn trailing_ones(self) -> u32 {
        self.trailing_ones()
    }

    #[inline(always)]
    fn leading_ones(self) -> u32 {
        self.leading_ones()
    }
}

impl WordOps for u32 {
    const MAX: Self = u32::MAX;
    const MIN: Self = u32::MIN;
    const ONE: Self = 1;
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    #[inline(always)]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    #[inline(always)]
    fn trailing_ones(self) -> u32 {
        self.trailing_ones()
    }

    #[inline(always)]
    fn leading_ones(self) -> u32 {
        self.leading_ones()
    }
}

impl WordOps for u64 {
    const MAX: Self = u64::MAX;
    const MIN: Self = u64::MIN;
    const ONE: Self = 1;
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    #[inline(always)]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    #[inline(always)]
    fn trailing_ones(self) -> u32 {
        self.trailing_ones()
    }

    #[inline(always)]
    fn leading_ones(self) -> u32 {
        self.leading_ones()
    }
}

impl WordOps for u128 {
    const MAX: Self = u128::MAX;
    const MIN: Self = u128::MIN;
    const ONE: Self = 1;
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    #[inline(always)]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    #[inline(always)]
    fn trailing_ones(self) -> u32 {
        self.trailing_ones()
    }

    #[inline(always)]
    fn leading_ones(self) -> u32 {
        self.leading_ones()
    }
}

impl WordOps for usize {
    const MAX: Self = usize::MAX;
    const MIN: Self = usize::MIN;
    const ONE: Self = 1;
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }

    #[inline(always)]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    #[inline(always)]
    fn trailing_ones(self) -> u32 {
        self.trailing_ones()
    }

    #[inline(always)]
    fn leading_ones(self) -> u32 {
        self.leading_ones()
    }
}
