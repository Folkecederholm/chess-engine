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
            found_moves.push(ChessMove::new(
                coord,
                Coord::xy(coord.x, one_tile_forward),
                None,
            ));
            // Only check for double move if a single move is possible
            let two_tiles_forward = forward_function(coord.y, 2).unwrap();
            let starting_rank = match self.get_colour_turn() {
                Colour::White => 2,
                Colour::Black => 7,
            };
            if self.tile_is_empty(coord.x, two_tiles_forward) && coord.y == starting_rank {
                found_moves.push(ChessMove::new(
                    coord,
                    Coord::xy(coord.x, two_tiles_forward),
                    None,
                ));
            }
        }
        // Check for capturing, including en passant
        let tile_between_left = coord.x.checked_sub(1);
        if let Some(tile) = tile_between_left
            && tile > 0
            && (self.tile_can_be_taken(tile, one_tile_forward)
                || Some(Coord::xy(tile, one_tile_forward)) == self.get_passant_square())
        {
            found_moves.push(ChessMove::new(
                coord,
                Coord::xy(tile, one_tile_forward),
                None,
            ));
        }
        let tile_between_right = coord.x.checked_add(1);
        if let Some(tile) = tile_between_right
            && tile <= 8
            && (self.tile_can_be_taken(tile, one_tile_forward)
                || Some(Coord::xy(tile, one_tile_forward)) == self.get_passant_square())
        {
            found_moves.push(ChessMove::new(
                coord,
                Coord::xy(tile, one_tile_forward),
                None,
            ));
        }
        return found_moves;
        // INNER FNS
        fn legal_pawn(coord: Coord) -> bool {
            if coord.y < 2 || coord.y > 7 {
                return false;
            }
            true
        }
    }
}
