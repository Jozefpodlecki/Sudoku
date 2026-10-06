use core::fmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(u8);

impl Digit {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(9);

    pub const fn new(value: u8) -> Option<Self> {
        if value >= 1 && value <= 9 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn index(self) -> usize {
        (self.0 - 1) as usize
    }

    pub const fn mask(self) -> Mask {
        Mask(1u16 << self.0)
    }

    pub const fn from_index(index: usize) -> Self {
        Self((index + 1) as u8)
    }

    pub const fn all() -> [Self; 9] {
        [
            Self(1), Self(2), Self(3),
            Self(4), Self(5), Self(6),
            Self(7), Self(8), Self(9),
        ]
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mask(u16);

impl Mask {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0b0000_0011_1111_1110);
    pub const COUNT: u32 = 9;

    pub const fn from_digit(digit: Digit) -> Self {
        digit.mask()
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn len(self) -> u32 {
        self.0.count_ones()
    }

    pub const fn contains(self, digit: Digit) -> bool {
        self.0 & (1u16 << digit.0) != 0
    }

    pub const fn insert(self, digit: Digit) -> Self {
        Self(self.0 | (1u16 << digit.0))
    }

    pub const fn remove(self, digit: Digit) -> Self {
        Self(self.0 & !(1u16 << digit.0))
    }

    pub const fn toggle(self, digit: Digit) -> Self {
        Self(self.0 ^ (1u16 << digit.0))
    }

    pub const fn lowest(self) -> Option<Digit> {
        if self.0 == 0 {
            None
        } else {
            Some(Digit(self.0.trailing_zeros() as u8))
        }
    }

    pub const fn highest(self) -> Option<Digit> {
        if self.0 == 0 {
            None
        } else {
            Some(Digit((15 - self.0.leading_zeros()) as u8))
        }
    }

    pub fn iter(self) -> MaskIter {
        MaskIter { remaining: self.0 }
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersect(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    pub const fn complement(self) -> Self {
        Self(!self.0 & Self::FULL.0)
    }

    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub const fn to_digits(self) -> [Option<Digit>; 9] {
        let mut result = [None; 9];
        let mut index = 0;
        while index < 9 {
            if self.0 & (1u16 << (index + 1)) != 0 {
                result[index] = Some(Digit((index + 1) as u8));
            }
            index += 1;
        }
        result
    }
}


impl BitOr for Mask {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}

impl BitAnd for Mask {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        self.intersect(other)
    }
}

impl BitXor for Mask {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        self.xor(other)
    }
}

impl Not for Mask {
    type Output = Self;
    fn not(self) -> Self {
        self.complement()
    }
}

impl Sub for Mask {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}

impl BitOrAssign for Mask {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl BitAndAssign for Mask {
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl BitXorAssign for Mask {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl SubAssign for Mask {
    fn sub_assign(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

pub struct MaskIter {
    remaining: u16,
}

impl Iterator for MaskIter {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let bit = self.remaining.trailing_zeros();
        self.remaining &= self.remaining - 1;
        Some(Digit(bit as u8))
    }
}