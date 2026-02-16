#![allow(unused)]
pub struct Board {
    pub(super) grid: [[Tile; 8]; 8],
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

#[derive(Debug)]
pub struct CastlingRights {
    pub(super) castling_rights: [Option<Coord>; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub(super) x: usize,
    pub(super) y: usize,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub(super) piece: Option<Piece>,
}

#[derive(Clone, Copy)]
//This needs to be public to construct an Option<Piece>
pub struct Piece {
    pub(super) colour: Colour,
    pub(super) piece_type: PieceType,
}

#[derive(Clone, Copy)]
pub enum Colour {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub(super) enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
