use crate::types::defs::*;

impl Board {
    pub fn go_to_fen(&mut self, fen: &str) {
        use crate::exit;
        use crate::types::defs::Piece;
        self.drain();
        let mut split = fen.split(' ');
        // let pieces_fen = match split.next() {
        //     Some(n) => n,
        //     None => {
        //         exit!("Invalid FEN!")
        //     }
        // };
        let Some(pieces_fen) = split.next() else {
            exit!("Invalid FEN!");
        };
        fen_board_state(self, pieces_fen);
        let Some(colour_fen) = split.next() else {
            exit!("Invalid FEN!");
        };
        fen_colour(self, colour_fen);
        let Some(castling_fen) = split.next() else {
            exit!("Invalid FEN!");
        };
        fen_castling(self, castling_fen);
        let Some(passant_fen) = split.next() else {
            exit!("Invalid FEN!");
        };
        fen_passant(self, passant_fen);
        let Some(fifty_move_rule_fen) = split.next() else {
            exit!("Invalid FEN!");
        };
        fen_fifty_move_rule(self, fifty_move_rule_fen);
        let Some(whole_move_fen) = split.next() else {
            exit!("Invalid FEN!");
        };
        fen_whole_moves(self, whole_move_fen);
        // INNER FNS
        fn fen_board_state(board: &mut Board, pieces_fen: &str) {
            let rows = pieces_fen.split('/');
            for (y, row) in rows.enumerate() {
                let mut row_iter = row.chars().enumerate();
                while let Some(tuple) = row_iter.next() {
                    let (x, piece) = tuple;
                    if piece.is_ascii_digit() {
                        row_iter.nth(piece.to_digit(10).unwrap() as usize);
                    } else {
                        let piece_to_add = Piece::get_piece_from_fen(piece);
                        board.set_piece(Coord::xy(x + 1, y + 1), piece_to_add);
                    }
                }
            }
        }
        fn fen_colour(board: &mut Board, colour_fen: &str) {
            board.set_to_move(match colour_fen {
                "w" => Colour::White,
                "b" => Colour::Black,
                _ => {
                    exit!("Invalid FEN!");
                }
            });
        }
        fn fen_castling(board: &mut Board, castling_fen: &str) {
            if castling_fen.len() > 4 {
                exit!("Invalid FEN!")
            }
            let mut castling_rights: [Option<Coord>; 4] = [None, None, None, None];
            let fen_castling_fn: fn(char) -> Option<Coord> = match board.variant {
                ChessVariant::Chess => fen_castling_normal_chess,
                ChessVariant::Fisher => fen_castling_fisher_chess,
            };
            for (i, piece) in castling_fen.chars().enumerate() {
                castling_rights[i] = fen_castling_fn(piece);
            }
            board.set_castling(castling_rights);

            fn fen_castling_normal_chess(piece: char) -> Option<Coord> {
                match piece {
                    'K' => Coord::ay('a', 1),
                    'Q' => Coord::ay('h', 1),
                    'k' => Coord::ay('a', 8),
                    'q' => Coord::ay('h', 8),
                    _ => None,
                }
            }
            fn fen_castling_fisher_chess(piece: char) -> Option<Coord> {
                let y = if piece.is_ascii_lowercase() { 8 } else { 1 };
                let a = piece.to_ascii_lowercase();
                Coord::ay(a, y)
            }
        }
        fn fen_passant(board: &mut Board, passant_fen: &str) {
            if passant_fen == "-" {
                board.set_passant(None);
            } else {
                let passant_tile = Coord::new(passant_fen);
                board.set_passant(passant_tile);
            }
        }
        fn fen_fifty_move_rule(board: &mut Board, fifty_move_rule_fen: &str) {
            let moves: Result<u32, _> = fifty_move_rule_fen.parse();
            let moves = match moves {
                Ok(n) => n,
                Err(e) => exit!("{e}"),
            };
            board.set_fifty_moves(moves);
        }
        fn fen_whole_moves(board: &mut Board, whole_move_fen: &str) {
            let moves: Result<u32, _> = whole_move_fen.parse();
            let moves = match moves {
                Ok(n) => n,
                Err(e) => exit!("{e}"),
            };
            board.set_whole_moves(moves);
        }
    }
}
