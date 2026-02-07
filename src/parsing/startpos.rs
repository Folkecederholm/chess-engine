use crate::types::defs::{Board, Coord};

impl Board {
    pub fn from_startpos(&mut self, moves: Vec<&str>) {
        for single_move in moves {
            let start = Coord::new(single_move);
            let single_move_later_part = match single_move.get(2..) {
                Some(n) => n,
                None => {
                    eprintln!("Can't do that move!");
                    std::process::exit(1);
                }
            };
            let end = Coord::new(single_move_later_part);
            self.make_move(start, end);
        }
    }
}
