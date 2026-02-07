use std::fmt;

use crate::types::defs::*;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(piece) = self.piece {
            let piece_char = get_codepoint(piece);
            return write!(f, "{}", piece_char);
        } else {
            return write!(f, "Ø");
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        for row in self.board.iter().rev() {
            buf.push_str("\n");
            for tile in row.iter().rev() {
                buf.push_str(format!("{} ", tile).as_str());
            }
        }
        writeln!(f, "{}", buf)
    }
}
