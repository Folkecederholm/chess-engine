use crate::types::defs::*;

// I realise that knight.rs is unnecessary.
// This file should contain a function that gets all sliding moves, from an x- and a y-coordinate
// for the jump lengths. It should also contain a slide function to continue.
// A rook = 1,0,true
// A knight = 1,2,false
// A king = 1,0,false + 1,1,false
// und so weiter

impl Board {
    // fn find_sliding_moves(
    //     &self,
    //     coord: Coord,
    //     jumps: (isize, isize),
    //     move_to_fn: fn(Tile) -> bool,
    //     slide: bool,
    // ) -> Vec<ChessMove> {
    //     let (a, b) = jumps;
    //     let mut moves = vec![];
    //     moves.append(&mut straight_sliding(
    //         &self,
    //         coord,
    //         (a, b),
    //         move_to_fn,
    //         slide,
    //     ));
    //     moves.append(&mut straight_sliding(
    //         &self,
    //         coord,
    //         (a, -b),
    //         move_to_fn,
    //         slide,
    //     ));
    //     moves.append(&mut straight_sliding(
    //         &self,
    //         coord,
    //         (-a, b),
    //         move_to_fn,
    //         slide, /* */
    //     ));
    //     moves.append(&mut straight_sliding(
    //         &self,
    //         coord,
    //         (-a, -b),
    //         move_to_fn,
    //         slide,
    //     ));
    //     if !(a == b || a * b == 0) {
    //         moves.append(&mut straight_sliding(
    //             &self,
    //             coord,
    //             (b, a),
    //             move_to_fn,
    //             slide,
    //         ));
    //         moves.append(&mut straight_sliding(
    //             &self,
    //             coord,
    //             (b, -a),
    //             move_to_fn,
    //             slide,
    //         ));
    //         moves.append(&mut straight_sliding(
    //             &self,
    //             coord,
    //             (-b, a),
    //             move_to_fn,
    //             slide,
    //         ));
    //         moves.append(&mut straight_sliding(
    //             &self,
    //             coord,
    //             (-b, -a),
    //             move_to_fn,
    //             slide,
    //         ));
    //     }
    //     return moves;
    //     // INNER FNS
    //     fn straight_sliding(
    //         board: &Board,
    //         coord: Coord,
    //         jump: (isize, isize),
    //         acceptance_func: fn(Tile) -> bool,
    //         slide: bool,
    //     ) -> Vec<ChessMove> {
    //         let mut moves = vec![];
    //         let mut current_coord = coord;
    //         loop {
    //             let (new_x, new_y) = {
    //                 let new_x_isize = current_coord.x as isize + jump.0;
    //                 if !new_x_isize.is_positive() {
    //                     break;
    //                 }
    //                 let new_y_isize = current_coord.y as isize + jump.1;
    //                 if !new_y_isize.is_positive() {
    //                     break;
    //                 }
    //                 (new_x_isize as usize, new_y_isize as usize)
    //             };
    //             current_coord = Coord::xy(new_x, new_y);
    //             if acceptance_func(board.get_tile(current_coord)) {
    //                 moves.push(current_coord);
    //             } else {
    //                 break;
    //             }
    //             if !slide {
    //                 break;
    //             }
    //         }
    //         return vec![];
    //     }
    // }
    pub fn find_sliding_moves(&self, coord: Coord, slider: Slider) -> Vec<ChessMove> {
        let mut moves = vec![];
        let siblings = slider.siblings();
        for single_slider in siblings {
            moves.append(&mut straight_sliding(self, coord, single_slider));
        }
        return moves;
        fn straight_sliding(board: &Board, start_coord: Coord, slider: Slider) -> Vec<ChessMove> {
            let mut moves = vec![];
            let mut coord = start_coord;
            loop {
                coord = {
                    let new_x_isize = coord.x as isize + slider.x;
                    if !new_x_isize.is_positive() || new_x_isize > 8 {
                        return moves;
                    }
                    let new_y_isize = coord.y as isize + slider.y;
                    if !new_y_isize.is_positive() || new_y_isize > 8 {
                        return moves;
                    }
                    Coord::xy(new_x_isize as usize, new_y_isize as usize)
                };
                if (slider.move_fn)(board, board.get_tile(coord)) {
                    moves.push(ChessMove::new(start_coord, coord, None));
                } else {
                    return moves;
                }
                if !slider.slide {
                    return moves;
                }
            }
            // It never comes to this point, since the code always exits in the loop {}.
            //return moves;
        }
    }
}
