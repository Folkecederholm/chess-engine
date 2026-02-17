use crate::types::defs::Board;

mod parsing {
    pub mod fen;
    pub mod input;
    pub mod trace;
}
mod types {
    pub mod defs;
    pub mod impls {
        pub mod board;
        pub mod coord;
        pub mod piece;
        pub mod tile;
    }
}

mod extras {
    pub mod macros;
}

fn main() {
    let mut input = String::new();
    let mut board = Board::empty();
    loop {
        parsing::input::take_user_input(&mut input);
        parsing::input::parse_user_input(input.as_str(), &mut board);
    }
}
