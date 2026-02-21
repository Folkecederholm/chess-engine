mod find_moves {
    pub mod all;
    pub mod castling;
    pub mod check;
    pub mod pawn;
    pub mod sliding;
}

mod choose_move {
    pub mod master;
}

mod parsing {
    pub mod fen;
    pub mod input;
    pub mod trace;
}
mod types {
    pub mod defs;
    pub mod impls {
        mod board {
            pub mod fmt;
            pub mod helpers;
            pub mod make_move;
        }
        pub mod chess_move;
        pub mod coord;
        pub mod piece;
        pub mod slider;
        pub mod tile;
    }
}

mod extras {
    pub mod macros;
    pub mod patterns;
}

use crate::types::defs::Board;
fn main() {
    let mut input = String::new();
    let mut board = Board::empty();
    loop {
        parsing::input::take_user_input(&mut input);
        parsing::input::parse_user_input(input.as_str(), &mut board);
    }
}
