use crate::types::defs::*;
use std::ops::Range;

impl Board {
    // Sorry future code reader.
    // The idea is to find all tiles between the king and rook,
    // and check if they're all empty.
    pub fn find_castling_moves(&self) -> Vec<ChessMove> {
        let castling_rights = self.get_castling_rights();
        let mut castling_moves = vec![];
        let rank = match self.get_colour_turn() {
            Colour::White => 1,
            Colour::Black => 8,
        };
        // NOT compatible with Fisher Chess. Will fix later
        let king_x = 5;
        for castling_candidate in castling_rights {
            let Some(castling_move) = castling_candidate else {
                continue;
            };
            if castling_move.as_tuple().1 != rank {
                continue;
            }
            let rook_x = castling_move.as_tuple().0;
            // let mut range_between = king_x..=rook_x;
            let mut range_between = make_range_iterator(king_x, rook_x);
            range_between.next();
            range_between.next_back();
            let mut tiles_between = range_between.map(|x| self.get_tile(Coord::xy(x, rank)));
            // let res: Vec<Tile> = tiles_between.clone().collect();
            // dbg!(res);
            if tiles_between.all(|x| x.get_piece().is_none()) {
                castling_moves.push(ChessMove::new(
                    Coord::xy(king_x, rank),
                    Coord::xy(rook_x, rank),
                    None,
                ));
            }
        }
        return castling_moves;
        fn make_range_iterator(a: usize, b: usize) -> Range<usize> {
            if a > b { b..a } else { a..b }
        }
    }
}
