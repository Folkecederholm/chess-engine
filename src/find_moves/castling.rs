use crate::types::defs::*;

impl Board {
    /*
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
            let mut king_to_rook_x_iter = make_range_iterator(king_x, rook_x);
            king_to_rook_x_iter.next();
            king_to_rook_x_iter.next_back();
            let mut king_to_rook_tiles =
                king_to_rook_x_iter.map(|x| self.get_tile(Coord::xy(x, rank)));
            if king_to_rook_tiles.any(|x| x.get_piece().is_some()) {
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
    */
    pub fn find_castling_moves(&self) -> Vec<(ChessMove, ChessMove)> {
        use std::collections::HashSet;
        let mut castling_moves = vec![];
        let castling_rights = self.get_castling_rights();
        let rank = match self.get_colour_turn() {
            Colour::White => 1,
            Colour::Black => 8,
        };
        'candidates: for castling_candidate_wrapped in castling_rights {
            // Sort out alternatives that are None
            let Some(castling_candidate) = castling_candidate_wrapped else {
                continue;
            };
            // Sort out alternatives that aren't on the right rank; these are for the other player
            if castling_candidate.y != rank {
                continue;
            }
            // Time to check if there are any pieces in between them!
            let rook_x = castling_candidate.x;
            let Some(king_x) = find_king_x(self, rank) else {
                panic!("Error: can't find castling moves: castling rights but no king!"); // continue; // There is no king. There shouldn't have been any castling alternatives left.
            };
            // let range1: HashSet<usize> = HashSet::from_iter(rook_x..=king_x);
            // let range2: HashSet<usize> = HashSet::from_iter(king_x..=rook_x);
            let mut king_dest_x = 0;
            let mut rook_dest_x = 0;
            let range1: HashSet<usize> = {
                let mut s = rook_x..=king_x;
                s.next();
                s.next_back();
                {
                    let mut z = s.clone();
                    if let Some(dest) = z.next_back() {
                        rook_dest_x = dest;
                    }
                    if let Some(dest) = z.next_back() {
                        king_dest_x = dest;
                    }
                }
                HashSet::from_iter(s)
            };
            let range2 = {
                let mut s = king_x..=rook_x;
                s.next();
                s.next_back();
                {
                    let mut z = s.clone();
                    if let Some(dest) = z.next() {
                        rook_dest_x = dest;
                    }
                    if let Some(dest) = z.next() {
                        king_dest_x = dest;
                    }
                }
                HashSet::from_iter(s)
            };
            if king_dest_x * rook_dest_x == 0 {
                // Some of the variables are zero. That means that neither range1 nor range2 are more than
                // an empty iterator.
                // So the king is squashed between the rooks???
                return vec![];
            }
            let range = range1.union(&range2);
            for &x_coord in range {
                // If no tiles contain pieces, this can be a castling.
                // If any tiles contain pieces, this can not be a castling.
                let piece = self.get_tile(Coord::xy(x_coord, rank)).get_piece();
                // println!("piece: {piece:?}");
                if piece.is_some() {
                    continue 'candidates;
                }
            }
            // Since it passed all checks, add the move to the list of allowed castlig moves
            castling_moves.push((
                ChessMove::new(Coord::xy(king_x, rank), Coord::xy(king_dest_x, rank), None),
                ChessMove::new(Coord::xy(rook_x, rank), Coord::xy(rook_dest_x, rank), None),
            ));
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
    }
}
