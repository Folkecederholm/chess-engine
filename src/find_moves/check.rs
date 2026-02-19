use crate::types::defs::*;

impl Board {
    // pub fn find_problematic_moves(&self) -> Vec<ChessMove> {
    //     let possible_moves = self.find_unchecked_moves();
    //     let mut problematic_moves = vec![];
    //     let mut suspicious_moves = vec![];
    //     println!("self.get_king(): {:?}", self.get_king());
    //     for chess_move in possible_moves {
    //         println!("move: {chess_move:?}");
    //         if chess_move.end() == self.get_king() {
    //             suspicious_moves.push(chess_move);
    //         }
    //     }
    //     //
    //     let moves_that_see_king = {
    //         fn tile_is_capturable(board: &Board, tile: Tile) -> bool {
    //             if tile.get_piece().is_none() {
    //                 true
    //             } else {
    //                 tile.get_piece().unwrap().colour != board.get_colour_turn()
    //             }
    //         }
    //         let mut moves_king_ortho = self.find_sliding_moves(
    //             &self.get_king(),
    //             Slider {
    //                 x: 1,
    //                 y: 0,
    //                 slide: true,
    //                 move_fn: tile_is_capturable,
    //             },
    //         );
    //         let mut moves_king_diag = self.find_sliding_moves(
    //             &self.get_king(),
    //             Slider {
    //                 x: 1,
    //                 y: 1,
    //                 slide: true,
    //                 move_fn: tile_is_capturable,
    //             },
    //         );
    //         moves_king_ortho.append(&mut moves_king_diag);
    //         moves_king_ortho
    //     };
    //     let moves_that_king_sees = {
    //         let mut result = vec![];
    //         for chess_move in moves_that_see_king {
    //             result.push(chess_move.reverse());
    //         }
    //         result
    //     };
    //     let suspicious_moves = suspicious_moves
    //         .into_iter()
    //         .filter(|x| moves_that_king_sees.contains(x))
    //         .collect::<Vec<ChessMove>>();
    //     for chess_move in suspicious_moves {
    //         // if !sees_king(chess_move.start(), self) {
    //         //     let mut state = self.clone();
    //         //     state.make_move(chess_move);
    //         //     if state.get_problematic_moves().len() == 0 {
    //         //         problematic_moves.push(chess_move);
    //         //     }
    //         // }
    //         let mut state = self.clone();
    //         match state.make_move(chess_move) {
    //             Ok(()) => {}
    //             Err(_) => break, // I honestly don't know what to do here. unreachable!() ?
    //         }
    //         if state.find_problematic_moves().is_empty() {
    //             problematic_moves.push(chess_move);
    //         }
    //     }
    //     problematic_moves
    // }
    pub fn find_problematic_moves(&self) -> Vec<ChessMove> {
        vec![]
    }
}
