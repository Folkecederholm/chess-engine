use crate::types::defs::{Board, Coord, Promotion};

impl Board {
    pub fn goto_startpos(&mut self, moves: Vec<&str>) {
        self.go_to_fen(include_str!("../extras/startpos.fen"));
        for single_move in moves {
            // println!("{single_move}");
            let (start, end, promote_to) = parse_single_move(single_move);
            // println!("start: {start:?}, end: {end:?}");
            self.make_move(start, end, promote_to);
        }
        fn parse_single_move(single_move: &str) -> (Coord, Coord, Option<Promotion>) {
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
                    Some('q') => Some(Promotion::Queen),
                    Some('r') => Some(Promotion::Rook),
                    Some('b') => Some(Promotion::Bishop),
                    Some('n') => Some(Promotion::Knight),
                    None => None,
                    _ => {
                        eprintln!("Can't parse single move: {single_move}");
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };
            (start, end, promote_to)
        }
    }
}
