use crate::{find_moves::check::is_move_allowed, types::defs::*};

impl Board {
    pub fn find_all_moves(&self) -> Vec<ChessMove> {
        let unchecked_moves = self.find_unchecked_moves();

        unchecked_moves
            .into_iter()
            .filter(|x| is_move_allowed(*x, self))
            .collect()
    }
    pub fn find_unchecked_moves(&self) -> Vec<ChessMove> {
        let mut moves = vec![];
        use MeetsPieceAction::*;
        fn tile_is_capturable(board: &Board, tile: Tile) -> MeetsPieceAction {
            // if tile.get_piece().is_none() {
            //     true
            // } else {
            //     tile.get_piece().unwrap().colour != board.get_colour_turn()
            // }
            let Some(piece) = tile.get_piece() else {
                return CanContinue;
            };
            if piece.colour == board.get_colour_turn() {
                CannotTake
            } else {
                // Other colour
                CanTake
            }
        }
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
                        moves.append(&mut self.find_pawn_moves(coord));
                    }
                    PieceType::Knight => {
                        let slider = Slider::new(1, 2, false, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                    }
                    PieceType::Rook => {
                        let slider = Slider::new(1, 0, true, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                    }
                    PieceType::Bishop => {
                        let slider = Slider::new(1, 1, true, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                    }
                    PieceType::Queen => {
                        let slider = Slider::new(1, 0, true, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                        let slider = Slider::new(1, 1, true, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                    }
                    PieceType::King => {
                        let slider = Slider::new(1, 0, false, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                        let slider = Slider::new(1, 1, false, tile_is_capturable);
                        moves.append(&mut self.find_sliding_moves(&coord, slider));
                    }
                }
            }
        }
        // Add castling moves to found moves
        let mut castling_moves = self
            .find_castling_moves()
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<ChessMove>>();
        moves.append(&mut castling_moves);
        moves
    }
}
