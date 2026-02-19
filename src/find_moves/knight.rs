use crate::types::defs::*;

const KNIGHT_DESTS: [(isize, isize); 8] = [
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
];

impl Board {
    pub(super) fn find_knight_moves(&self, coord: Coord) -> Vec<ChessMove> {
        let mut moves = vec![];
        for knight_dest in KNIGHT_DESTS {
            let Some(dest_x) = coord.x.checked_add_signed(knight_dest.0) else {
                continue;
            };
            let Some(dest_y) = coord.y.checked_add_signed(knight_dest.1) else {
                continue;
            };
            if dest_x < 1 || dest_x > 8 || dest_y < 1 || dest_y > 8 {
                continue;
            }
            if self.tile_is_movable(dest_x, dest_y) {
                moves.push(ChessMove::new(coord, Coord::xy(dest_x, dest_y), None))
            }
        }
        moves
    }
}
