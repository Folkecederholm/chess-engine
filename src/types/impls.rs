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
    pub fn make_move(&mut self, chess_move: ChessMove) {
        make_physical_move(self, chess_move);
        update_pasant_square(self, chess_move);
        switch_colours(self);
        update_whole_moves(self); // Run after make_physical_move()
        // INNER FNS
        fn make_physical_move(board: &mut Board, chess_move: ChessMove) {
            // Check for moving empty piece
            let start_tile = board.get_tile(chess_move.start());
            let Some(moved_piece) = start_tile.piece else {
                eprintln!("Tried to move an empty piece: {chess_move:?}");
                std::process::exit(1);
            };
            // Check for promotion
            let moving_piece = if let Some(promote_piece) = chess_move.promote_to {
                promote_piece.with_colour(board.turn_to_play)
            } else {
                moved_piece
            };
            check_for_passant(board, chess_move);
            board.set_piece(chess_move.end(), moving_piece);
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
        }
        fn update_pasant_square(board: &mut Board, chess_move: ChessMove) {
            // Check for pawn double move
            let y_diff = chess_move.start().y.abs_diff(chess_move.end().y);
            // And if there is a double move, set the en passant tile to the skipped tile
            if y_diff == 2 {
                let skipped_tile = Coord {
                    x: chess_move.start().x,
                    y: usize::midpoint(chess_move.start().y, chess_move.end().y),
                };
                board.set_passant(Some(skipped_tile));
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
    }
    pub fn set_piece(&mut self, coord: Coord, piece: Piece) {
        // self.grid[coord.x][coord.y] = Tile { piece: Some(piece) };
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0] = Tile { piece: Some(piece) };
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
}

impl Coord {
    pub fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn ay(a: char, y: usize) -> Option<Self> {
        let x = a as usize - 96; // a goes to 1, b goes to 2 ...
        if
        /*0 > x ||*/
        x > 8 {
            // Wrong coord
            // print!("Wrong coord!x:{};y:{},a:{}", x, y, a);
            // std::process::exit(1);
            return None;
        }
        Some(Self { x, y })
    }
    pub fn new(move_str: &str) -> Option<Self> {
        let Some(first_char) = move_str.chars().nth(0) else {
            eprintln!("Too short move sent: {move_str}");
            std::process::exit(1);
        };
        let Some(second_char) = move_str.chars().nth(1) else {
            eprintln!("Too short move sent: {move_str}");
            std::process::exit(1);
        };
        // let Some(to_return) = Self::ay(first_char, second_char as usize - 48) else {
        //     return None;
        // };
        let to_return = Self::ay(first_char, second_char as usize - 48)?;
        Some(to_return)
        // Self::ay(first_char, second_char as usize - 48)
    }
    pub fn zero_indexed(&self) -> (usize, usize) {
        (self.x - 1, self.y - 1)
    }
}

#[allow(unused)]
impl Tile {
    fn empty() -> Self {
        Self { piece: None }
    }
    pub fn with_piece(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
}

impl Piece {
    pub fn get_piece_from_fen(piece: char) -> Self {
        use super::defs::{Colour, PieceType};
        let colour = if piece.is_ascii_lowercase() {
            Colour::White
        } else {
            Colour::Black
        };
        let piece_type = match piece.to_ascii_lowercase() {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'p' => PieceType::Pawn,
            _ => {
                eprintln!("No such piece: {piece}");
                std::process::exit(0);
            }
        };
        Self { colour, piece_type }
    }
}

impl PieceType {
    fn with_colour(self, colour: Colour) -> Piece {
        Piece {
            piece_type: self,
            colour,
        }
    }
}

impl Colour {
    fn switch(&mut self) {
        *self = match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        };
    }
}

impl ChessMove {
    pub fn new(start: Coord, end: Coord, promote_to: Option<PieceType>) -> Self {
        Self {
            start,
            end,
            promote_to,
        }
    }
    pub fn start(&self) -> Coord {
        self.start
    }
    pub fn end(&self) -> Coord {
        self.end
    }
}
