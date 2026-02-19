#![allow(unused)]
pub struct Board {
    pub grid: [[Tile; 8]; 8],
    pub(super) turn_to_play: Colour,
    pub variant: ChessVariant,
    pub(super) passant_square: Option<Coord>,
    pub(super) fifty_move_rule: u32,
    pub(super) moves: u32,
    pub(super) castling_rights: CastlingRights,
}

pub enum ChessVariant {
    Chess,
    Fisher,
}

#[derive(Debug, Copy, Clone)]
pub struct CastlingRights {
    pub(super) castling_rights: [Option<Coord>; 4],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord {
    // These are 1-indexed
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub(super) piece: Option<Piece>,
}

#[derive(Clone, Copy, Debug)]
//This needs to be public to construct an Option<Piece>
pub struct Piece {
    pub colour: Colour,
    pub piece_type: PieceType,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Colour {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessMove {
    // These are 1-indexed
    pub(super) start: Coord,
    pub(super) end: Coord,
    pub(super) promote_to: Option<PieceType>,
}

#[derive(Debug)]
pub struct Slider {
    // x and y denote the jump sizes
    pub x: isize,
    pub y: isize,
    pub slide: bool,
    pub move_fn: fn(&Board, Tile) -> bool,
}
