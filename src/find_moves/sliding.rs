use crate::types::defs::*;

// I realise that knight.rs is unnecessary.
// This file should contain a function that gets all sliding moves, from an x- and a y-coordinate
// for the jump lengths. It should also contain a slide function to continue.
// A rook = 1,0,true
// A knight = 1,2,false
// A king = 1,0,false + 1,1,false
// und so weiter

impl Board {
    pub fn find_sliding_moves(&self, coord: &Coord, slider: Slider) -> Vec<ChessMove> {
        let mut moves = vec![];
        let siblings = slider.siblings();
        for single_slider in siblings {
            moves.append(&mut straight_sliding(self, *coord, single_slider));
        }
        return moves;
        #[allow(clippy::cast_possible_wrap)]
        #[allow(clippy::cast_sign_loss)]
        fn straight_sliding(board: &Board, start_coord: Coord, slider: Slider) -> Vec<ChessMove> {
            let mut moves = vec![];
            let mut coord = start_coord;
            loop {
                coord = match coord_add(coord, slider.x, slider.y) {
                    Ok(n) => n,
                    Err(()) => {
                        // This should silently fail since it happens every time a piece moves off the board
                        return moves;
                    }
                };
                // if (slider.move_fn)(board, board.get_tile(coord)) {
                //     moves.push(ChessMove::new(start_coord, coord, None));
                // } else {
                //     return moves;
                // }
                use MeetsPieceAction::*;
                match (slider.move_fn)(board, board.get_tile(coord)) {
                    CanContinue => {
                        moves.push(ChessMove::new(start_coord, coord, None));
                    }
                    CanTake => {
                        moves.push(ChessMove::new(start_coord, coord, None));
                        return moves;
                    }
                    CannotTake => {
                        return moves;
                    }
                }
                if !slider.slide {
                    return moves;
                }
            }
            fn coord_add(coord: Coord, other_x: isize, y_other: isize) -> Result<Coord, ()> {
                let new_x_isize = coord.x as isize + other_x;
                if !new_x_isize.is_positive() || new_x_isize > 8 {
                    return Err(());
                }
                let new_isize_y = coord.y as isize + y_other;
                if !new_isize_y.is_positive() || new_isize_y > 8 {
                    return Err(());
                }
                Ok(Coord::xy(new_x_isize as usize, new_isize_y as usize))
            }
        }
    }
}
