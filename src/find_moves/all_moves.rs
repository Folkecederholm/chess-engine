use crate::types::defs::*;

impl Board {
    pub fn find_all_moves(&self) -> Vec<ChessMove> {
        let mut moves = vec![];
        for x in 1..=8 {
            for y in 1..=8 {
                let coord = Coord::xy(x, y);
                let Some(piece) = self.get_tile(coord).get_piece() else {
                    continue;
                };
                if piece.colour != self.get_colour_turn() {
                    continue;
                }
                match piece.piece_type {
                    PieceType::Pawn => {
                        moves.append(self.find_pawn_moves(coord).as_mut());
                    }
                    _ => {}
                };
            }
        }
        return moves;
    }
}
