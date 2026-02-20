use crate::types::defs::*;
use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter().rev() {
            buf.push('\n');
            for tile in row {
                buf.push_str(format!("{tile} ").as_str());
            }
        }
        {
            push_board_info(self, &mut buf);
        }
        return writeln!(f, "{buf}");
        fn push_board_info(board: &Board, buf: &mut String) {
            let chess_variant = match board.variant {
                ChessVariant::Chess => "Chess",
                ChessVariant::Fisher => "Fisher",
            };
            let colour_to_play = match board.turn_to_play {
                Colour::White => "White",
                Colour::Black => "Black",
            };
            let castling_rights = format!("{:?}", board.castling_rights);
            let en_passant_square = format!("{:?}", board.passant_square);
            let fifty_move_rule = board.fifty_move_rule;
            let moves = board.moves;
            buf.push_str(format!("\nChess variant: {chess_variant}\nColour to play: {colour_to_play}\nCastling rights: {castling_rights}\nEn passant square: {en_passant_square}\nFifty move rule moves: {fifty_move_rule}\nWhole moves: {moves}\n").as_str());
        }
    }
}
