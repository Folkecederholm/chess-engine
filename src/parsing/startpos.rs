use crate::types::defs::{Board, Coord};

impl Board {
    pub fn goto_startpos(&mut self, moves: Vec<&str>) {
        for single_move in moves {
            let Some(start) = Coord::new(single_move) else {
                println!("Can't parse single move!");
                std::process::exit(1);
            };
            let Some(single_move_later_part) = single_move.get(2..) else {
                eprintln!("Can't do that move!");
                std::process::exit(1);
            };
            let Some(end) = Coord::new(single_move_later_part) else {
                println!("Can't parse single move!");
                std::process::exit(1);
            };
            self.make_move(start, end);
        }
    }
}
