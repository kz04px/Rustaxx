use std::ops::Not;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Colour {
    Black,
    White,
}

impl Not for Colour {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self::Output {
        match self {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black,
        }
    }
}
