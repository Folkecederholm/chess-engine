use crate::types::defs::*;

impl Piece {
    pub fn get_piece_from_fen(piece: char) -> Self {
        use crate::types::defs::{Colour, PieceType};
        let colour = if piece.is_ascii_lowercase() {
            Colour::White
        } else {
            Colour::Black
        };
        let piece_type = match piece.to_ascii_lowercase() {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'p' => PieceType::Pawn,
            _ => {
                eprintln!("No such piece: {piece}");
                std::process::exit(0);
            }
        };
        Self { colour, piece_type }
    }
}

impl PieceType {
    pub fn with_colour(self, colour: Colour) -> Piece {
        Piece {
            piece_type: self,
            colour,
        }
    }
}

impl Colour {
    pub fn switch(&mut self) {
        *self = match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        };
    }
}
