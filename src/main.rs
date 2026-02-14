use crate::types::defs::Board;
use crate::types::defs::{Piece, Tile};

mod parsing {
    pub mod fen;
    pub mod startpos;
}
mod types {
    pub mod defs;
    pub mod impls;
    pub mod traits;
}

mod extras {
    pub mod macros;
}

fn main() {
    let piece = Piece::get_piece_from_fen('Q');
    let tile = Tile::with_piece(piece);
    println!("{}", tile);
    let mut input = String::new();
    let mut board = Board::empty();
    loop {
        take_user_input(&mut input);
        parse_user_input(input.as_str(), &mut board);
    }
}

fn take_user_input(input: &mut String) {
    use std::io::{Write, stdin, stdout};
    input.clear();
    let _ = stdout().flush();
    stdin().read_line(input).expect("Failed to get input");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}

fn print_flush(string: &str) {
    use std::io::{Write, stdout};
    print!("{}\n", string);
    let _ = stdout().flush();
}

fn parse_user_input(input: &str, board: &mut Board) {
    if !input.contains(" ") {
        parse_single_word(input, board);
    } else {
        // Mutable so that we can remove the already known stuff when parsing later
        let mut tokens: Vec<_> = input.split(' ').collect();
        if let Some(first) = tokens.first() {
            match *first {
                "position" => {
                    let _ = tokens.remove(0);
                    parse_position(&mut tokens, board);
                }
                _ => {
                    println!("Hello");
                }
            }
        }
    }

    fn parse_single_word(input: &str, board: &Board) {
        match input {
            "uci" => {
                print_flush(include_str!("extras/uci.txt"));
            }
            "isready" => {
                print_flush("readyok");
            }
            "quit" | "q" => {
                std::process::exit(0);
            }
            "board" => {
                // board.print();
                print!("{}", board);
            }
            _ => {
                wrongly_called();
            }
        }
    }

    fn parse_position(input_tokens: &mut Vec<&str>, board: &mut Board) {
        if let Some(first) = input_tokens.first() {
            match *first {
                "startpos" => {
                    println!("Startpos it is!");
                    board.from_startpos(input_tokens.split_off(1));
                }
                "fen" => {
                    println!("fen is the way to go!");
                    board.go_to_fen(input_tokens.split_off(1).first().unwrap());
                }
                "default" => {
                    board.go_to_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                }
                _ => {
                    wrongly_called();
                }
            };
        }
    }
}

fn wrongly_called() {
    // Do nothing, according to UCI protocol
}
