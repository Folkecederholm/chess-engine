use crate::types::defs::*;

impl Board {
    pub fn empty() -> Self {
        let board: [[Tile; 8]; 8] = std::array::from_fn(|_| std::array::from_fn(|_| Tile::empty()));
        Self {
            grid: board,
            turn_to_play: Colour::White,
            variant: ChessVariant::Chess,
            passant_square: None,
            fifty_move_rule: 0,
            moves: 0,
            castling_rights: CastlingRights {
                castling_rights: [None, None, None, None],
            },
        }
    }
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        update_fifty_move_rule(self, chess_move); // Run before make_physical_move()
        if chess_move.is_castling(self) {
            make_castling_move(self, chess_move)?;
            self.remove_castling_rights_just_played();
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
            // let mut this_castling_move;
            // for possible_castling in possible_castlings {
            //     if possible_castling.0 == chess_move {
            //         this_castling_move = possible_castling;
            //         break;
            //     }
            // }
            let Some(this_castling_move) = possible_castlings.iter().find(|&x| x.0 == chess_move)
            else {
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
    }
    pub fn set_piece(&mut self, coord: Coord, piece: Piece) -> Result<(), &'static str> {
        // self.grid[coord.x][coord.y] = Tile { piece: Some(piece) };
        if coord.x > 8 || coord.y > 8 {
            return Err("Coord out of bounds!");
        }
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0] = Tile { piece: Some(piece) };
        Ok(())
    }
    // pub fn print(&self) {}
    pub fn drain(&mut self) {
        for row in &mut self.grid {
            for tile in row.iter_mut() {
                *tile = Tile::empty();
            }
        }
    }
    pub fn set_to_move(&mut self, colour: Colour) {
        self.turn_to_play = colour;
    }
    pub fn set_castling(&mut self, tiles: [Option<Coord>; 4]) {
        let mut castling_rights: [Option<Coord>; 4] = [None, None, None, None];
        for (i, tile) in tiles.iter().enumerate() {
            match tile {
                None => {
                    break;
                }
                Some(n) => {
                    castling_rights[i] = Some(*n);
                }
            }
        }
        self.castling_rights = CastlingRights { castling_rights };
    }
    pub fn set_passant(&mut self, tile: Option<Coord>) {
        self.passant_square = tile;
    }
    pub fn set_fifty_moves(&mut self, moves: u32) {
        self.fifty_move_rule = moves;
    }
    pub fn set_whole_moves(&mut self, moves: u32) {
        self.moves = moves;
    }
    pub fn increment_whole_moves(&mut self) {
        self.moves += 1;
    }
    pub fn get_tile(&self, coord: Coord) -> Tile {
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0]
    }
    pub fn remove_piece(&mut self, coord: Coord) {
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0] = Tile { piece: None };
    }
    pub fn get_castling_rights(&self) -> [Option<Coord>; 4] {
        self.castling_rights.castling_rights
    }
    pub fn get_colour_turn(&self) -> Colour {
        self.turn_to_play
    }
    pub fn remove_castling_rights_just_played(&mut self) {
        let rank = match self.get_colour_turn() {
            // Switched because we want the colour that just played
            // Not switched because it works like that
            Colour::White => 1,
            Colour::Black => 8,
        };
        for x in 1..8 {
            self.remove_castling_rights_coord(Coord::xy(x, rank));
        }
    }
    pub fn remove_castling_rights_coord(&mut self, coord: Coord) {
        // Feels tautological
        let castling_rights = self.castling_rights.castling_rights;
        // This will automatically clone
        let mut new_castling_rights = castling_rights;
        let mut colour = self.turn_to_play;
        colour.switch();
        let mut i = 0;
        for right in castling_rights {
            let Some(castling_right) = right else {
                continue;
            };
            // If it's the coord we want to remove
            if castling_right == coord {
                // new_castling_rights[i] = Some(castling_right);
                new_castling_rights[i] = None;
            }
            i += 1;
        }
        self.set_castling(new_castling_rights);
    }
    pub fn get_passant_square(&self) -> Option<Coord> {
        self.passant_square
    }
}

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter().rev() {
            buf.push('\n');
            for tile in row {
                buf.push_str(format!("{tile} ").as_str());
            }
        }
        {
            push_board_info(self, &mut buf);
        }
        return writeln!(f, "{buf}");
        fn push_board_info(board: &Board, buf: &mut String) {
            let chess_variant = match board.variant {
                ChessVariant::Chess => "Chess",
                ChessVariant::Fisher => "Fisher",
            };
            let colour_to_play = match board.turn_to_play {
                Colour::White => "White",
                Colour::Black => "Black",
            };
            let castling_rights = format!("{:?}", board.castling_rights);
            let en_passant_square = format!("{:?}", board.passant_square);
            let fifty_move_rule = board.fifty_move_rule;
            let moves = board.moves;
            buf.push_str(format!("\nChess variant: {chess_variant}\nColour to play: {colour_to_play}\nCastling rights: {castling_rights}\nEn passant square: {en_passant_square}\nFifty move rule moves: {fifty_move_rule}\nWhole moves: {moves}\n").as_str());
        }
    }
}
