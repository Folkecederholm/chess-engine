use crate::types::defs::*;

impl Board {
    pub fn find_castling_moves(&self) -> Vec<(ChessMove, ChessMove)> {
        let mut castling_moves = vec![]; // We will push to this later
        let castling_rights = self.get_castling_rights();
        let rank = match self.get_colour_turn() {
            Colour::White => 1,
            Colour::Black => 8,
        };
        let Some(king_x) = find_king_x(self, rank) else {
            return vec![];
        };
        for castling_candidate in castling_rights {
            let Some(castling_move) = castling_candidate else {
                continue;
            };
            if castling_move.as_tuple().1 != rank {
                continue;
            }
            let rook_x = castling_move.as_tuple().0;
            let mut king_to_rook_x = make_range_iterator(king_x, rook_x);
            king_to_rook_x.next();
            king_to_rook_x.next_back();
            let mut king_to_rook_tiles = king_to_rook_x.map(|x| self.get_tile(Coord::xy(x, rank)));
            if !king_to_rook_tiles.all(|x| x.get_piece().is_none()) {
                continue;
            }
            // Now we know all tiles between the rook and king are empty
            // So we know it's going to be a castling opportunity
            // king_x _ _ _ rook_x
            let mut king_to_rook_x = make_range_iterator(king_x, rook_x);
            king_to_rook_x.next(); // (king_x) _ _ _ rook_x
            let Some(rook_dest_x) = king_to_rook_x.next() else {
                continue;
            }; // (king_x) (_) _ _ rook_x
            let Some(king_dest_x) = king_to_rook_x.next() else {
                continue;
            }; // (king_x) (_) (_) _ rook_x
            let king_move =
                ChessMove::new(Coord::xy(king_x, rank), Coord::xy(king_dest_x, rank), None);
            let rook_move =
                ChessMove::new(Coord::xy(rook_x, rank), Coord::xy(rook_dest_x, rank), None);
            castling_moves.push((king_move, rook_move));
        }
        return castling_moves;
        // INNER FNS
        fn find_king_x(board: &Board, rank: usize) -> Option<usize> {
            for x in 1..8 {
                let tile = board.get_tile(Coord::xy(x, rank));
                let Some(piece) = tile.get_piece() else {
                    continue;
                };
                if piece.piece_type == PieceType::King {
                    return Some(x);
                }
            }
            None
        }
        fn make_range_iterator(
            king_x: usize,
            rook_x: usize,
        ) -> Box<dyn DoubleEndedIterator<Item = usize>> {
            if king_x < rook_x {
                Box::new(king_x..=rook_x)
            } else {
                Box::new((rook_x..=king_x).rev())
            }
        }
    }
}
