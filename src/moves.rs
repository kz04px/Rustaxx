use std::char;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

impl Move {
    #[must_use]
    pub const fn is_single(&self) -> bool {
        self.from == self.to
    }

    #[must_use]
    pub const fn is_double(&self) -> bool {
        self.from != self.to
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_single() {
            write!(
                f,
                "{}{}",
                (97 + (self.from % 7)) as char,
                (49 + (self.from / 7)) as char
            )
        } else {
            write!(
                f,
                "{}{}{}{}",
                (97 + (self.from % 7)) as char,
                (49 + (self.from / 7)) as char,
                (97 + (self.to % 7)) as char,
                (49 + (self.to / 7)) as char
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Move, Square};

    #[test]
    fn from() {
        assert_eq!(
            Move {
                from: Square::A1 as u8,
                to: Square::A3 as u8,
            }
            .from,
            Square::A1 as u8
        );
    }

    #[test]
    fn to() {
        assert_eq!(
            Move {
                from: Square::A1 as u8,
                to: Square::A3 as u8,
            }
            .to,
            Square::A3 as u8
        );
    }

    #[test]
    fn is_single() {
        assert!(Move {
            from: Square::A1 as u8,
            to: Square::A1 as u8,
        }
        .is_single());
        assert!(!Move {
            from: Square::A1 as u8,
            to: Square::A3 as u8,
        }
        .is_single());
    }
}
