use crate::types::defs::*;
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
            // En Croissant sends e1a1 instead of e1c1
            /*[*/
            if *self == ChessMove::new(castling_move.0.start(), castling_move.1.start(), None) {
                return true;
            }
            /*]*/
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
    pub fn long_algebraic(&self) -> String {
        let s = match self.promote_to {
            None => "",
            Some(PieceType::Queen) => "q",
            Some(PieceType::Knight) => "n",
            Some(PieceType::Rook) => "r",
            Some(PieceType::Bishop) => "b",
            _ => unreachable!(),
        };
        format!("{}{}{}", self.start(), self.end(), s)
    }
}
