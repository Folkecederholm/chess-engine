#![allow(clippy::unused_self)]
use crate::types::defs::*;

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
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[allow(unused)]
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
    pub fn moved_piece(&self, board: &Board) -> Piece {
        let Tile { piece: Some(moved) } = board.get_tile(self.start()) else {
            eprintln!("Move tries to move nothing!");
            std::process::exit(1);
        };
        moved
    }
    pub fn player_colour(&self, board: &Board) -> Colour {
        let mut colour = board.turn_to_play;
        colour.switch();
        colour
    }
    pub fn taken_piece(&self, board: &Board) -> Option<Piece> {
        // This function doesn't work with en passant
        // I don't know where this could turn into a problem
        board.get_tile(self.end()).piece
    }
    pub fn draw(&self, board: &Board) -> bool {
        self.fifty_move_rule_draw(board)
    }
    // This belongs to the draw() method -------v
    fn fifty_move_rule_draw(&self, board: &Board) -> bool {
        let one_move_left = board.fifty_move_rule == 49;
        let piece_taken = self.taken_piece(board).is_some();
        let pawn_moved = self.moved_piece(board).piece_type == PieceType::Pawn;
        one_move_left && !(piece_taken || pawn_moved)
    }
    // This is free territory----------^
    pub fn is_castling(&self, board: &Board) -> bool {
        let castling_moves = board.find_castling_moves();
        for castling_move in castling_moves {
            if *self == castling_move.0 {
                // .0 is the king move
                return true;
            }
        }
        false
    }
    fn manhattan(&self) -> usize {
        let x1 = self.start().x;
        let y1 = self.start().y;
        let x2 = self.end().x;
        let y2 = self.end().y;
        usize::abs_diff(x1, x2) + usize::abs_diff(y1, y2)
    }
    pub fn reverse(&self) -> Self {
        // Promotion moves should NOT be reversed
        // Hmmm... maybe I should set promote_to to PieceType::Pawn?
        Self {
            start: self.end,
            end: self.start,
            promote_to: None,
        }
    }
}
