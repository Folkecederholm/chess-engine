use crate::types::defs::*;

impl Board {
    pub fn go_to_fen(&mut self, fen: &str) {
        use crate::types::defs::Piece;
        self.drain();
        let mut split = fen.split(" ");
        let pieces_fen = match split.next() {
            Some(n) => n.chars(),
            None => {
                eprintln!("No first field in FEN string for pieces!");
                std::process::exit(1);
            }
        };
        let rows = pieces_fen.as_str().split("/");
        for (x, row) in rows.enumerate() {
            let mut row_iter = row.chars().enumerate();
            while let Some(tuple) = row_iter.next() {
                let (y, piece) = tuple;
                if piece.is_ascii_digit() {
                    row_iter.nth(piece.to_digit(10).unwrap() as usize);
                } else {
                    let piece_to_add = Piece::get_piece_from_fen(piece);
                    self.set_piece(Coord::xy(x, y), piece_to_add);
                }
            }
        }
    }
}
