use std::char;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Pass,
    Drop(u8),
    Jump(u8, u8),
}

impl Move {
    #[must_use]
    pub const fn is_pass(&self) -> bool {
        match self {
            Move::Pass => true,
            _ => false,
        }
    }

    #[must_use]
    pub const fn is_single(&self) -> bool {
        match self {
            Move::Drop(_) => true,
            _ => false,
        }
    }

    #[must_use]
    pub const fn is_double(&self) -> bool {
        match self {
            Move::Jump(_, _) => true,
            _ => false,
        }
    }

    #[must_use]
    pub const fn from(&self) -> Option<u8> {
        match self {
            Move::Pass => None,
            Move::Drop(_) => None,
            Move::Jump(fr, _) => Some(*fr),
        }
    }

    #[must_use]
    pub const fn to(&self) -> Option<u8> {
        match self {
            Move::Pass => None,
            Move::Drop(sq) => Some(*sq),
            Move::Jump(_, to) => Some(*to),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "0000"),
            Move::Drop(sq) => write!(f, "{}{}", (97 + (sq % 7)) as char, (49 + (sq / 7)) as char),
            Move::Jump(fr, to) => write!(
                f,
                "{}{}{}{}",
                (97 + (fr % 7)) as char,
                (49 + (fr / 7)) as char,
                (97 + (to % 7)) as char,
                (49 + (to / 7)) as char
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Move, Square};

    #[test]
    fn from() {
        assert_eq!(Move::Pass.from(), None);
        assert_eq!(Move::Drop(Square::A1 as u8).from(), None);
        assert_eq!(
            Move::Jump(Square::A1 as u8, Square::A3 as u8).from(),
            Some(Square::A1 as u8)
        );
    }

    #[test]
    fn to() {
        assert_eq!(Move::Pass.to(), None);
        assert_eq!(Move::Drop(Square::A1 as u8).to(), Some(Square::A1 as u8));
        assert_eq!(
            Move::Jump(Square::A1 as u8, Square::A3 as u8).to(),
            Some(Square::A3 as u8)
        );
    }

    #[test]
    fn is_single() {
        assert!(Move::Pass.is_pass());
        assert!(!Move::Pass.is_single());
        assert!(!Move::Pass.is_double());

        assert!(!Move::Drop(Square::A1 as u8).is_pass());
        assert!(Move::Drop(Square::A1 as u8).is_single());
        assert!(!Move::Drop(Square::A1 as u8).is_double());

        assert!(!Move::Jump(Square::A1 as u8, Square::A3 as u8).is_pass());
        assert!(!Move::Jump(Square::A1 as u8, Square::A3 as u8).is_single());
        assert!(Move::Jump(Square::A1 as u8, Square::A3 as u8).is_double());
    }
}
