use crate::types::defs::*;

impl Board {
    pub(super) fn find_pawn_moves(&self, coord: Coord) -> Vec<ChessMove> {
        if !legal_pawn(coord) {
            return vec![];
        }
        let mut found_moves = vec![];
        let forward_function = match self.get_colour_turn() {
            Colour::White => usize::checked_add,
            Colour::Black => usize::checked_sub,
        };
        // One tile forward
        let one_tile_forward = forward_function(coord.y, 1).unwrap();
        if self.tile_is_empty(coord.x, one_tile_forward) {
            add_to_moves_vector(&mut found_moves, coord, coord.x, one_tile_forward, self);
            // Only check for double move if a single move is possible
            let starting_rank = match self.get_colour_turn() {
                Colour::White => 2,
                Colour::Black => 7,
            };
            if coord.y == starting_rank {
                let two_tiles_forward = forward_function(coord.y, 2).unwrap();
                if self.tile_is_empty(coord.x, two_tiles_forward) && coord.y == starting_rank {
                    add_to_moves_vector(&mut found_moves, coord, coord.x, two_tiles_forward, self);
                }
            }
        }
        // Check for capturing, including en passant
        let tile_between_left = coord.x.checked_sub(1);
        if let Some(tile) = tile_between_left
            && tile > 0
            && (self.tile_can_be_taken(tile, one_tile_forward)
                || Some(Coord::xy(tile, one_tile_forward)) == self.get_passant_square())
        {
            add_to_moves_vector(&mut found_moves, coord, tile, one_tile_forward, self);
        }
        let tile_between_right = coord.x.checked_add(1);
        if let Some(tile) = tile_between_right
            && tile <= 8
            && (self.tile_can_be_taken(tile, one_tile_forward)
                || Some(Coord::xy(tile, one_tile_forward)) == self.get_passant_square())
        {
            add_to_moves_vector(&mut found_moves, coord, tile, one_tile_forward, self);
        }
        return found_moves;
        // INNER FNS
        fn legal_pawn(coord: Coord) -> bool {
            if coord.y < 2 || coord.y > 7 {
                return false;
            }
            true
        }
        fn add_to_moves_vector(
            vector: &mut Vec<ChessMove>,
            coord: Coord,
            x: usize,
            y: usize,
            board: &Board,
        ) {
            let promotion_rank = match board.get_colour_turn() {
                Colour::White => 8,
                Colour::Black => 1,
            };
            if y == promotion_rank {
                vector.push(ChessMove::new(
                    coord,
                    Coord::xy(x, y),
                    Some(PieceType::Queen),
                ));
                vector.push(ChessMove::new(
                    coord,
                    Coord::xy(x, y),
                    Some(PieceType::Knight),
                ));
                vector.push(ChessMove::new(
                    coord,
                    Coord::xy(x, y),
                    Some(PieceType::Bishop),
                ));
                vector.push(ChessMove::new(
                    coord,
                    Coord::xy(x, y),
                    Some(PieceType::Rook),
                ));
            } else {
                vector.push(ChessMove::new(coord, Coord::xy(x, y), None));
            }
        }
    }
}
