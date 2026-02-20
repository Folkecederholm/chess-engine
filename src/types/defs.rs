#[derive(Clone)]
pub struct Board {
    pub grid: [[Tile; 8]; 8],
    pub(super) turn_to_play: Colour,
    pub variant: ChessVariant,
    pub(super) passant_square: Option<Coord>,
    pub(super) fifty_move_rule: u32,
    pub(super) moves: u32,
    pub(super) castling_rights: CastlingRights,
}

#[allow(unused)]
// Fisher chess in unimplemented
/*
 * To implement Fisher chess:
 *  - Add the UCI compatible option
 *  - Change the parsing of FEN
 * The rest should be Fisher chess compatible!
 */
#[derive(Clone)]
pub enum ChessVariant {
    Chess,
    Fisher,
}

// These represent what castling rights are on a board.
// For example, if you've moved your rook and moved it back, you can't castle on that side.
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

// This struct is to define pieces.
// rook: x = 1, y = 0, slide = true
// knight: x = 1, y = 2, slide = false
#[derive(Debug, Clone, Copy)]
pub struct Slider {
    // x and y denote the jump sizes
    pub x: isize,
    pub y: isize,
    pub slide: bool,
    pub move_fn: fn(&Board, Tile) -> MeetsPieceAction,
}

// This enum is to define what a piece should do after having met a piece.
// Should it not be able to take the piece?
// Should it be able to take the piece?
// Or should it continue?
pub enum MeetsPieceAction {
    CanTake,
    CanContinue,
    CannotTake,
}
