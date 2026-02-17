use crate::types::defs::PieceType;
use crate::types::defs::*;

impl Board {
    pub fn trace_moves(&mut self, moves: Vec<&str>) {
        for single_move in moves {
            // println!("{single_move}");
            let chess_move = parse_single_move(single_move);
            // println!("start: {start:?}, end: {end:?}");
            match self.make_move(chess_move) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }
        fn parse_single_move(single_move: &str) -> ChessMove {
            let Some(start) = Coord::new(single_move) else {
                println!("Can't parse single move: {single_move}");
                std::process::exit(1);
            };
            let Some(single_move_later_part) = single_move.get(2..) else {
                eprintln!("Can't parse single move: {single_move}");
                std::process::exit(1);
            };
            let Some(end) = Coord::new(single_move_later_part) else {
                println!("Can't parse single move: {single_move}");
                std::process::exit(1);
            };
            let promote_to = if single_move.len() == 5 {
                match single_move.chars().last() {
                    Some('q') => Some(PieceType::Queen),
                    Some('r') => Some(PieceType::Rook),
                    Some('b') => Some(PieceType::Bishop),
                    Some('n') => Some(PieceType::Knight),
                    None => None,
                    _ => {
                        eprintln!("Can't parse single move: {single_move}");
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };
            ChessMove::new(start, end, promote_to)
        }
    }
}
