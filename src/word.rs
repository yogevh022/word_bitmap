use std::fmt::{Binary, Debug, Display};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

pub trait Word:
    Default
    + Copy
    + Debug
    + Display
    + Binary
    + BitScan
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
{
}

impl<T> Word for T where
    T: Default
        + Copy
        + Debug
        + Display
        + Binary
        + BitScan
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
{
}

pub trait BitScan {
    fn trailing_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_ones(self) -> u32;
    fn leading_ones(self) -> u32;
}

impl BitScan for u8 {
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

impl BitScan for u16 {
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

impl BitScan for u32 {
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

impl BitScan for u64 {
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

impl BitScan for u128 {
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

impl BitScan for usize {
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
