use crate::types::defs::*;

impl Board {
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        update_fifty_move_rule(self, chess_move); // Run before make_physical_move()
        if chess_move.is_castling(self) {
            make_castling_move(self, chess_move)?;
            self.remove_castling_rights_colour(self.get_colour_turn());
        } else {
            make_physical_move(self, chess_move)?;
        }
        update_pasant_square(self, chess_move);
        switch_colours(self);
        update_whole_moves(self); // Run after make_physical_move()
        return Ok(());
        // INNER FNS
        fn make_castling_move(
            board: &mut Board,
            chess_move: ChessMove,
        ) -> Result<(), &'static str> {
            if !chess_move.is_castling(board) {
                return Err("Can't do castling move: not a castling");
            }
            let possible_castlings = board.find_castling_moves();
            // This closure is a bit weird. It's because En Croissant sends e1a1 instead of e1c1 /*[*/.../*]*/
            let Some(this_castling_move) = possible_castlings.iter().find(|&x| {
                x.0 == chess_move /*[*/||(chess_move == ChessMove::new(x.0.start(), x.1.start(), None))
            } /*]*/) else {
                return Err("Can't do castling move: internal error");
            };
            // Firstly, move king
            make_physical_move(board, this_castling_move.0)?;
            // Secondly, move rook
            make_physical_move(board, this_castling_move.1)?;
            Ok(())
        }
        fn make_physical_move(
            board: &mut Board,
            chess_move: ChessMove,
        ) -> Result<(), &'static str> {
            // Check for moving empty piece
            let start_tile = board.get_tile(chess_move.start());
            let Some(moved_piece) = start_tile.piece else {
                return Err("Can't do move: tried to move an empty piece");
            };
            // Check for promotion
            let moving_piece = if let Some(promote_piece) = chess_move.promote_to {
                promote_piece.with_colour(board.turn_to_play)
            } else {
                moved_piece
            };
            check_for_passant(board, chess_move);
            remove_castling_rights(board, chess_move);
            board.set_piece(chess_move.end(), moving_piece)?;
            board.remove_piece(chess_move.start());
            // INNER FNS
            fn check_for_passant(board: &mut Board, chess_move: ChessMove) {
                if chess_move.end == board.passant_square.unwrap_or(Coord { x: 9, y: 9 }) {
                    // Remove the captured piece
                    let offset_func = match board.turn_to_play {
                        Colour::White => usize::checked_sub,
                        Colour::Black => usize::checked_add,
                    };
                    let Some(captured_y) = offset_func(chess_move.end().y, 1) else {
                        unreachable!()
                    };
                    let captured_tile = Coord {
                        x: (chess_move.end().x),
                        y: (captured_y),
                    };
                    board.remove_piece(captured_tile);
                }
            }
            Ok(())
        }
        fn update_pasant_square(board: &mut Board, chess_move: ChessMove) {
            if let Tile { piece: Some(piece) } = board.get_tile(chess_move.start()) {
                if piece.piece_type != PieceType::Pawn {
                    return;
                }
            } else {
                return;
            }
            // Check for pawn double move
            let y_diff = chess_move.start().y.abs_diff(chess_move.end().y);
            // And if there is a double move, set the en passant tile to the skipped tile
            if y_diff == 2 {
                let skipped_tile = Coord {
                    x: chess_move.start().x,
                    y: usize::midpoint(chess_move.start().y, chess_move.end().y),
                };
                board.set_passant(Some(skipped_tile));
            } else {
                board.set_passant(None);
            }
        }
        fn switch_colours(board: &mut Board) {
            board.turn_to_play.switch();
        }
        fn update_whole_moves(board: &mut Board) {
            // This checks if it's white's turn since it's run after make_physical_move()
            if board.turn_to_play == Colour::White {
                board.increment_whole_moves();
            }
        }
        fn update_fifty_move_rule(board: &mut Board, chess_move: ChessMove) {
            let taken_piece = chess_move.taken_piece(board);
            let moved_piece = chess_move.moved_piece(board);
            if taken_piece.is_some() || moved_piece.piece_type == PieceType::Pawn {
                board.fifty_move_rule = 0;
            } else {
                board.fifty_move_rule += 1;
            }
        }
        fn remove_castling_rights(board: &mut Board, chess_move: ChessMove) {
            match chess_move.moved_piece(board).piece_type {
                PieceType::King => board.remove_castling_rights_colour(board.get_colour_turn()),
                PieceType::Rook => board.remove_castling_rights_coord(chess_move.start()),
                _ => {}
            }
        }
    }
}
