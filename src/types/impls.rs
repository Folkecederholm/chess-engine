use crate::types::defs::*;

impl Board {
    pub fn empty() -> Self {
        let board: [[Tile; 8]; 8] = std::array::from_fn(|_| std::array::from_fn(|_| Tile::empty()));
        Self {
            board: board,
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
        let moved_option = self.board[start.x][start.y].piece;
        if let Some(moved_piece) = moved_option {
            self.board[end.x][end.y].piece = Some(moved_piece);
            self.board[start.x][start.y].piece = None;
        } else {
            eprintln!("Tried to move an empty piece!");
            std::process::exit(1);
        }
    }
    pub fn set_piece(&mut self, coord: Coord, piece: Piece) {
        self.board[coord.x][coord.y] = Tile { piece: Some(piece) };
    }
    // pub fn print(&self) {}
    pub fn drain(&mut self) {
        for row in self.board.iter_mut() {
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
        Self { x: x, y: y }
    }
    pub fn ay(a: char, y: usize) -> Self {
        let x = a as usize - 97;
        if
        /*0 > x ||*/
        x > 7 {
            // Wrong coord
            print!("Wrong coord!x:{};y:{},a:{}", x, y, a);
            std::process::exit(1);
        }
        Self { x: x, y: y }
    }
    pub fn new(move_str: &str) -> Self {
        let first_char = match move_str.chars().nth(0) {
            Some(n) => n,
            None => {
                eprintln!("Too short move sent");
                std::process::exit(1);
            }
        };
        let second_char = match move_str.chars().nth(1) {
            Some(n) => n,
            None => {
                eprintln!("Too short move sent");
                std::process::exit(1);
            }
        };
        Self::ay(first_char, second_char as usize - 48)
    }
}

impl Tile {
    fn empty() -> Self {
        let tile = Self { piece: None };
        tile
    }
    pub fn with_piece(piece: Piece) -> Self {
        let tile = Self { piece: Some(piece) };
        tile
    }
}

impl Piece {
    pub fn get_piece_from_fen(piece: char) -> Self {
        use super::defs::{Colour, PieceType};
        let colour = match piece.is_ascii_lowercase() {
            true => Colour::White,
            false => Colour::Black,
        };
        let piece_type = match piece.to_ascii_lowercase() {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'p' => PieceType::Pawn,
            _ => {
                eprintln!("No such piece: {}", piece);
                std::process::exit(0);
            }
        };
        Self {
            colour: colour,
            piece_type: piece_type,
        }
    }
}
