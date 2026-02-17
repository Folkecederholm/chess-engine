use crate::types::defs::*;
use std::fmt;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(piece) = self.piece {
            let piece_char = get_codepoint(piece);
            return write!(f, "{piece_char}");
        } else {
            return write!(f, " ");
        }
        fn get_codepoint(piece: Piece) -> char {
            let mut codepoint: u32 = 0x2654;
            codepoint += match piece.piece_type {
                PieceType::King => 0,
                PieceType::Queen => 1,
                PieceType::Rook => 2,
                PieceType::Bishop => 3,
                PieceType::Knight => 4,
                PieceType::Pawn => 5,
            };
            codepoint += match piece.colour {
                Colour::White => 0,
                Colour::Black => 6,
            };
            match char::from_u32(codepoint) {
                None => unreachable!(),
                Some(n) => n,
            }
        }
    }
}

#[allow(unused)]
impl Tile {
    pub fn empty() -> Self {
        Self { piece: None }
    }
    pub fn with_piece(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
    pub fn get_piece(self) -> Option<Piece> {
        self.piece
    }
}
