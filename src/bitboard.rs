use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

#[derive(Debug, Clone)]
pub struct BitboardIter(u64);

impl Bitboard {
    #[must_use]
    pub const fn from_index(sq: u8) -> Self {
        Self(1u64 << sq)
    }

    #[must_use]
    pub const fn full() -> Self {
        Self(0x1ffffffffffff)
    }

    #[must_use]
    pub const fn new(bb: u64) -> Self {
        Self(bb)
    }
}

impl Iterator for BitboardIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Some(index)
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = u8;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    #[must_use]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        Self(!self.0 & 0x1ffffffffffff)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    #[must_use]
    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    #[must_use]
    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl Bitboard {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.0 == 0x1ffffffffffff
    }

    #[must_use]
    pub const fn north(&self) -> Self {
        Self((self.0 << 7) & 0x1ffffffffffff)
    }

    #[must_use]
    pub const fn south(&self) -> Self {
        Self(self.0 >> 7)
    }

    #[must_use]
    pub const fn east(&self) -> Self {
        Self((self.0 << 1) & 0x1fbf7efdfbf7e)
    }

    #[must_use]
    pub const fn west(&self) -> Self {
        Self((self.0 >> 1) & 0xfdfbf7efdfbf)
    }

    #[must_use]
    pub const fn singles(&self) -> Self {
        Self(
            (self.0 << 7 | self.0 >> 7) & 0x1ffffffffffff
                | (self.0 >> 1 | self.0 >> 8 | self.0 << 6) & 0xfdfbf7efdfbf
                | (self.0 << 1 | self.0 >> 6 | self.0 << 8) & 0x1fbf7efdfbf7e,
        )
    }

    #[must_use]
    pub const fn doubles(&self) -> Self {
        Self(
            // right 2
            ((self.0 << 16 | self.0 << 9 | self.0 << 2 | self.0 >> 5 | self.0 >> 12) & 0x1f3e7cf9f3e7c) |
            // right 1
            ((self.0 << 15 | self.0 >> 13) & 0x1fbf7efdfbf7e) |
            // centre
            ((self.0 << 14 | self.0 >> 14) & 0x1ffffffffffff) |
            // left 1
            ((self.0 << 13 | self.0 >> 15) & 0xfdfbf7efdfbf) |
            // left 2
            ((self.0 << 12 | self.0 << 5 | self.0 >> 2 | self.0 >> 9 | self.0 >> 16) & 0x7cf9f3e7cf9f),
        )
    }

    #[must_use]
    pub fn reach(&self) -> Self {
        self.singles() | self.doubles()
    }

    #[must_use]
    pub const fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_tests() {
        assert!(crate::Bitboard(0).count() == 0);
        assert!(crate::Bitboard(1).count() == 1);
        assert!(crate::Bitboard(2).count() == 1);
        assert!(crate::Bitboard(3).count() == 2);
    }

    #[test]
    fn empty() {
        assert!(crate::Bitboard(0).is_empty());
        assert!(!crate::Bitboard(1).is_empty());
    }

    #[test]
    fn bitxor() {
        assert!(crate::Bitboard(0) ^ crate::Bitboard(0) == crate::Bitboard(0));
        assert!(crate::Bitboard(1) ^ crate::Bitboard(2) == crate::Bitboard(3));
    }

    #[test]
    fn bitor() {
        assert!(crate::Bitboard(1) | crate::Bitboard(2) == crate::Bitboard(3));
    }

    #[test]
    fn bitand() {
        assert!(crate::Bitboard(1) & crate::Bitboard(2) == crate::Bitboard(0));
    }

    #[test]
    fn bitnot() {
        assert_eq!(!crate::Bitboard(0), crate::Bitboard(0x1ffffffffffff));
        assert_eq!(!crate::Bitboard(0x1ffffffffffff), crate::Bitboard(0));
    }

    #[test]
    fn north() {
        assert_eq!(crate::Bitboard(0x0).north(), crate::Bitboard(0x0));
        assert_eq!(crate::Bitboard(0x1).north(), crate::Bitboard(0x80));
        assert_eq!(crate::Bitboard(0x7f).north(), crate::Bitboard(0x3f80));
        assert_eq!(
            crate::Bitboard(0x3f800000000).north(),
            crate::Bitboard(0x1fc0000000000)
        );
        assert_eq!(
            crate::Bitboard(0x1fc0000000000).north(),
            crate::Bitboard(0x0)
        );
    }

    #[test]
    fn south() {
        assert_eq!(crate::Bitboard(0x0).south(), crate::Bitboard(0x0));
        assert_eq!(crate::Bitboard(0x80).south(), crate::Bitboard(0x1));
        assert_eq!(crate::Bitboard(0x1fc000).south(), crate::Bitboard(0x3f80));
        assert_eq!(crate::Bitboard(0x3f80).south(), crate::Bitboard(0x7f));
        assert_eq!(crate::Bitboard(0x7f).south(), crate::Bitboard(0x0));
    }

    #[test]
    fn east() {
        assert_eq!(crate::Bitboard(0x0).east(), crate::Bitboard(0x0));
        assert_eq!(crate::Bitboard(0x1).east(), crate::Bitboard(0x2));
    }

    #[test]
    fn west() {
        assert_eq!(crate::Bitboard(0x0).west(), crate::Bitboard(0x0));
        assert_eq!(crate::Bitboard(0x1).west(), crate::Bitboard(0x0));
    }

    #[test]
    fn singles() {
        assert_eq!(crate::Bitboard(0x0).singles(), crate::Bitboard(0x0));
        assert_eq!(crate::Bitboard(0x1).singles(), crate::Bitboard(0x182));
        assert_eq!(crate::Bitboard(0x100).singles(), crate::Bitboard(0x1c287));
    }

    #[test]
    fn doubles() {
        assert_eq!(crate::Bitboard(0x0).doubles(), crate::Bitboard(0x0));
        assert_eq!(crate::Bitboard(0x1).doubles(), crate::Bitboard(0x1c204));
        assert_eq!(crate::Bitboard(0x100).doubles(), crate::Bitboard(0x1e20408));
    }
}
