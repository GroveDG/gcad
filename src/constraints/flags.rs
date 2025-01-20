use std::ops::{BitAnd, BitOr, BitXor};

/// Flags for characterizing constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConFlags {
    bits: u8,
}
impl ConFlags {
    pub const NONE: Self = Self { bits: 0b0000_0000 };
    /// This constrataint counts towards the 2
    /// constraints needed for discretizing.
    pub const DISCRETIZING: Self = Self { bits: 0b0000_0001 };

    pub fn contains(self, other: Self) -> bool {
        self == self & other
    }
}
impl Into<u8> for ConFlags {
    fn into(self) -> u8 {
        self.bits
    }
}
impl Default for ConFlags {
    fn default() -> Self {
        ConFlags::DISCRETIZING
    }
}
impl BitAnd for ConFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}
impl BitOr for ConFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}
impl BitXor for ConFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}
