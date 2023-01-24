#[cfg(test)]
mod tests {
    #[test]
    fn fen() {
        let tests: [&str; 8] = [
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/7/7/7/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 20 40",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 20 40",
        ];

        for fen in tests {
            let pos = ataxx::Board::from_fen(fen);
            assert_eq!(pos.get_fen(), fen);
        }
    }
}
