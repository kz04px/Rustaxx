use crate::bitboard::*;
use crate::colour::*;
use crate::moves::Move;
use crate::LUT_DOUBLES;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board {
    pub pieces: [Bitboard; 3],
    pub turn: Colour,
    pub halfmoves: u32,
    pub fullmoves: u32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "board")
    }
}

impl Board {
    #[must_use]
    pub fn after_move(&self, mv: &Move) -> Self {
        let mut npos = *self;
        npos.makemove(mv);
        npos
    }

    #[must_use]
    pub fn after_pass(&self) -> Self {
        let mut npos = *self;
        npos.makepass();
        npos
    }

    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        if fen == "startpos" {
            return Board::from_fen("x5o/7/7/7/7/7/o5x x 0 1");
        }

        let mut pos = Board {
            pieces: [Bitboard(0), Bitboard(0), Bitboard(0)],
            turn: Colour::Black,
            halfmoves: 0,
            fullmoves: 1,
        };
        let parts: Vec<&str> = fen.split(' ').collect();

        if parts.len() != 4 {
            panic!("FEN wrong format");
        }

        let mut x: i32 = 0;
        let mut y: i32 = 6;
        for c in parts[0].chars() {
            match c {
                'x' | 'X' => {
                    pos.pieces[0] ^= crate::Bitboard::from_index((7 * y + x) as u8);
                    x += 1;
                }
                'o' | 'O' => {
                    pos.pieces[1] ^= crate::Bitboard::from_index((7 * y + x) as u8);
                    x += 1;
                }
                '-' => {
                    pos.pieces[2] ^= crate::Bitboard::from_index((7 * y + x) as u8);
                    x += 1;
                }
                '1'..='7' => x += (c as u8 - b'0') as i32,
                '/' => {
                    x = 0;
                    y -= 1;
                }
                _ => panic!("Unrecognised FEN token"),
            }
        }

        match parts[1] {
            "x" | "X" => pos.turn = crate::Colour::Black,
            "o" | "O" => pos.turn = crate::Colour::White,
            _ => panic!("Unrecognised FEN token"),
        }

        match parts[2].parse::<u32>() {
            Ok(n) => pos.halfmoves = n,
            Err(_e) => panic!("Unrecognised FEN token"),
        }

        match parts[3].parse::<u32>() {
            Ok(n) => pos.fullmoves = n,
            Err(_e) => panic!("Unrecognised FEN token"),
        }

        pos
    }

    #[must_use]
    pub fn get_fen(&self) -> String {
        let mut fen = String::from("");

        for y in (0..=6).rev() {
            let mut spaces = 0;

            for x in 0..=6 {
                let sq: u8 = 7 * y + x;
                let bb: Bitboard = Bitboard::from_index(sq);

                if !(self.pieces[0] & bb).is_empty() {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += "x";
                } else if !(self.pieces[1] & bb).is_empty() {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += "o";
                } else if !(self.pieces[2] & bb).is_empty() {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += "-";
                } else {
                    spaces += 1;
                }
            }

            if spaces > 0 {
                fen += &spaces.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        if self.turn == Colour::Black {
            fen += " x";
        } else {
            fen += " o";
        }

        fen += " ";
        fen += &self.halfmoves.to_string();

        fen += " ";
        fen += &self.fullmoves.to_string();

        fen
    }

    #[must_use]
    pub fn is_gameover(&self) -> bool {
        self.black().is_empty()
            || self.white().is_empty()
            || self.halfmoves >= 100
            || (self.both().reach() & self.empty()).is_empty()
    }

    #[must_use]
    pub const fn black(&self) -> Bitboard {
        self.pieces[Colour::Black as usize]
    }

    #[must_use]
    pub const fn white(&self) -> Bitboard {
        self.pieces[Colour::White as usize]
    }

    #[must_use]
    pub const fn blockers(&self) -> Bitboard {
        self.pieces[2]
    }

    #[must_use]
    pub fn both(&self) -> Bitboard {
        self.pieces[0] | self.pieces[1]
    }

    #[must_use]
    pub fn empty(&self) -> Bitboard {
        !(self.black() | self.white() | self.blockers())
    }

    #[must_use]
    pub const fn us(&self) -> Bitboard {
        self.pieces[self.turn as usize]
    }

    #[must_use]
    pub fn them(&self) -> Bitboard {
        self.pieces[!self.turn as usize]
    }

    pub fn makemove(&mut self, moved: &crate::Move) {
        let to_bb = crate::Bitboard::from_index(moved.to);
        let from_bb = crate::Bitboard::from_index(moved.from);
        let captured: Bitboard = to_bb.singles() & self.them();

        // Remove and replace our stone
        self.pieces[self.turn as usize] ^= to_bb | from_bb;

        // Flip any captured stones
        self.pieces[!self.turn as usize] ^= captured;
        self.pieces[self.turn as usize] ^= captured;

        self.halfmoves += 1;
        self.fullmoves += (self.turn == Colour::White) as u32;

        if moved.is_single() {
            self.halfmoves = 0;
        }

        self.turn = !self.turn;
    }

    #[must_use]
    pub fn legal_moves(&self) -> Vec<Move> {
        if self.is_gameover() {
            return Vec::new();
        }

        self.pseudolegal_moves()
    }

    #[must_use]
    pub fn pseudolegal_moves(&self) -> Vec<Move> {
        let mut vec = Vec::with_capacity(200);

        // Single moves
        for sq in self.us().singles() & self.empty() {
            vec.push(Move { from: sq, to: sq });
        }

        // Double moves
        for from in self.us() {
            for to in LUT_DOUBLES[from as usize] & self.empty() {
                vec.push(Move { from, to });
            }
        }

        vec
    }

    #[must_use]
    pub fn count_pseudomoves(&self) -> u64 {
        let mut nodes = 0;

        // Single moves
        nodes += (self.us().singles() & self.empty()).count();

        // Double moves
        for from in self.us() {
            nodes += (LUT_DOUBLES[from as usize] & self.empty()).count();
        }

        // Pass
        if nodes == 0 {
            nodes = 1;
        }

        nodes as u64
    }

    #[must_use]
    pub fn count_moves(&self) -> u64 {
        if self.is_gameover() {
            return 0;
        }

        self.count_pseudomoves()
    }

    #[must_use]
    pub fn can_pass(&self) -> bool {
        if self.is_gameover() {
            return false;
        }

        self.us().reach() & self.empty() == Bitboard(0)
    }

    pub fn makepass(&mut self) {
        self.halfmoves += 1;
        self.fullmoves += (self.turn == Colour::White) as u32;
        self.turn = !self.turn;
    }
}
