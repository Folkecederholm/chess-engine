use crate::types::defs::{Board, Coord};

impl Board {
    pub fn goto_startpos(&mut self, moves: Vec<&str>) {
        self.go_to_fen(include_str!("../extras/startpos.fen"));
        for single_move in moves {
            println!("{single_move}");
            let Some(start) = Coord::new(single_move) else {
                println!("Can't parse single move: {single_move}");
                std::process::exit(1);
            };
            let Some(single_move_later_part) = single_move.get(2..) else {
                eprintln!("Can't do that move: {single_move}");
                std::process::exit(1);
            };
            let Some(end) = Coord::new(single_move_later_part) else {
                println!("Can't parse single move: {single_move}");
                std::process::exit(1);
            };
            println!("start: {start:?}, end: {end:?}");
            self.make_move(start, end);
        }
    }
}
