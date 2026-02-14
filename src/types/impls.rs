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
    pub fn make_move(&mut self, start: Coord, end: Coord) {
        let moved_option = self.grid[start.x][start.y].piece;
        if let Some(moved_piece) = moved_option {
            self.grid[end.x][end.y].piece = Some(moved_piece);
            self.grid[start.x][start.y].piece = None;
        } else {
            eprintln!("Tried to move an empty piece!");
            std::process::exit(1);
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
}

impl Coord {
    #[allow(unused)]
    pub fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn ay(a: char, y: usize) -> Option<Self> {
        let x = a as usize - 97;
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
            eprintln!("Too short move sent");
            std::process::exit(1);
        };
        let Some(second_char) = move_str.chars().nth(1) else {
            eprintln!("Too short move sent");
            std::process::exit(1);
        };
        // let Some(to_return) = Self::ay(first_char, second_char as usize - 48) else {
        //     return None;
        // };
        let to_return = Self::ay(first_char, second_char as usize - 48)?;
        Some(to_return)
        // Self::ay(first_char, second_char as usize - 48)
    }
}

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
