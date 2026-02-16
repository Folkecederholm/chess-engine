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
    pub fn move_events(&self, board: &Board) -> MoveEvents {
        MoveEvents {
            moved_piece: (self.moved_piece(board)),
            player_colour: (self.player_colour(board)),
            taken_piece: (self.taken_piece(board)),
            draw: (self.draw(board)),
        }
    }
    fn moved_piece(&self, board: &Board) -> Piece {
        let Tile { piece: Some(moved) } = board.get_tile(self.start()) else {
            eprintln!("Move tries to move nothing!");
            std::process::exit(1);
        };
        moved
    }
    fn player_colour(&self, board: &Board) -> Colour {
        let mut colour = board.turn_to_play;
        colour.switch();
        colour
    }
    fn taken_piece(&self, board: &Board) -> Option<Piece> {
        board.get_tile(self.end()).piece
    }
    fn draw(&self, board: &Board) -> bool {
        self.fifty_move_rule_draw(board)
    }
    fn fifty_move_rule_draw(&self, board: &Board) -> bool {
        let one_move_left = board.fifty_move_rule == 49;
        let piece_taken = self.move_events(board).taken_piece.is_some();
        let pawn_moved = self.move_events(board).moved_piece.piece_type == PieceType::Pawn;
        one_move_left && !(piece_taken || pawn_moved)
    }
}
