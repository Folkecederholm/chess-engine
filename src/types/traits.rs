use std::fmt;

use crate::types::defs::*;

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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter().rev() {
            buf.push('\n');
            for tile in row.iter().rev() {
                buf.push_str(format!("{tile} ").as_str());
            }
        }
        {
            let chess_variant = match self.variant {
                ChessVariant::Chess => "Chess",
                ChessVariant::Fisher => "Fisher",
            };
            let colour_to_play = match self.turn_to_play {
                Colour::White => "White",
                Colour::Black => "Black",
            };
            let castling_rights = format!("{:?}", self.castling_rights);
            let en_passant_square = format!("{:?}", self.passant_square);
            let fifty_move_rule = self.fifty_move_rule;
            let moves = self.moves;
            buf.push_str(format!("\nChess variant: {chess_variant}\nColour to play: {colour_to_play}\nCastling rights: {castling_rights}\nEn passant square: {en_passant_square}\nFifty move rule moves: {fifty_move_rule}\nWhole moves: {moves}\n").as_str());
        }
        writeln!(f, "{buf}")
    }
}
