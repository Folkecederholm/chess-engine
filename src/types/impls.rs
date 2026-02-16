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
    pub fn make_move(&mut self, start: Coord, end: Coord, promote_to: Option<Promotion>) {
        println!("start: {start:?}, end: {end:?}");
        make_physical_move(self, start, end, promote_to);
        update_pasant_square(self, start, end);
        switch_colours(self);
        update_whole_moves(self); // Run after make_physical_move()
        // INNER FNS
        fn make_physical_move(
            board: &mut Board,
            start: Coord,
            end: Coord,
            promote_to: Option<Promotion>,
        ) {
            // Check for moving empty piece
            let moved_option = board.grid[start.y][start.x].piece;
            let Some(moved_piece) = moved_option else {
                eprintln!("Tried to move an empty piece!");
                std::process::exit(1);
            };
            // Check for promotion
            let moving_piece = Some(
                // If there is a promotion
                if let Some(promote_piece) = promote_to {
                    promote_piece.piece().with_colour(board.turn_to_play)
                } else {
                    moved_piece
                },
            );
            check_for_passant(board, start, end);
            board.grid[end.y][end.x].piece = moving_piece;
            board.grid[start.y][start.x].piece = None;
            // INNER FNS
            fn check_for_passant(board: &mut Board, _start: Coord, end: Coord) {
                if end == board.passant_square.unwrap_or(Coord { x: 9, y: 9 }) {
                    println!("Doing en passant!");
                    // Remove the captured piece
                    let offset = match board.turn_to_play {
                        Colour::White => -1,
                        Colour::Black => 1,
                    };
                    // Casting usize to isize is OK since 0 <= start.y <= 7
                    // Castling isize to usize is OK since en passant never happens on the first rank
                    // If en passant happens on the first rank, this is UB
                    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                    {
                        assert!(end.y <= 7);
                        assert!(end.y as isize + offset > 0);
                        println!(
                            "end.y: {}\n(end.y as isize + offset) as usize: {}\nend.x: {}",
                            end.y,
                            (end.y as isize + offset) as usize,
                            end.x
                        );
                        board.grid[(end.y as isize + offset) as usize][end.x].piece = None;
                    }
                }
            }
        }
        fn update_pasant_square(board: &mut Board, start: Coord, end: Coord) {
            // Check for pawn double move
            let y_diff = start.y.abs_diff(end.y);
            // And if there is a double move, set the en passant tile to the skipped tile
            if y_diff == 2 {
                let skipped_tile = Coord {
                    x: start.x,
                    y: usize::midpoint(start.y, end.y),
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
        self.grid[coord.x][coord.y] = Tile { piece: Some(piece) };
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
}

impl Coord {
    pub fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn ay(a: char, y: usize) -> Option<Self> {
        let x = a as usize - 97; // a goes to 0, b goes to 1 ...
        if
        /*0 > x ||*/
        x > 7 {
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
        let to_return = Self::ay(first_char, second_char as usize - 49)?; // -49 because it's zero-indexed
        Some(to_return)
        // Self::ay(first_char, second_char as usize - 48)
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

impl Promotion {
    fn piece(&self) -> PieceType {
        match self {
            Self::Queen => PieceType::Queen,
            Self::Rook => PieceType::Rook,
            Self::Bishop => PieceType::Bishop,
            Self::Knight => PieceType::Knight,
        }
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
