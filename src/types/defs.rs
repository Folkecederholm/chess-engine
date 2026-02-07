#![allow(unused)]
pub struct Board {
    pub(super) board: [[Tile; 8]; 8],
}

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
pub(super) enum Colour {
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
