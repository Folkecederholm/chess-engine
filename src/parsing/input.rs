use crate::{choose_move::master::choose_move, types::defs::Board};

pub fn take_user_input(input: &mut String) {
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
    println!("{string}");
    let _ = stdout().flush();
}

pub fn parse_user_input(input: &str, board: &mut Board) {
    if input.contains(' ') {
        // Mutable so that we can remove the already known stuff when parsing later
        let mut tokens: Vec<_> = input.split(' ').collect();
        if let Some(first) = tokens.first() {
            match *first {
                "position" => {
                    let _ = tokens.remove(0);
                    parse_position(&mut tokens, board);
                }
                "go" => {
                    let found_moves = board.find_all_moves();
                    let chosen_move = choose_move(board, &found_moves);
                    println!("bestmove {}", chosen_move.long_algebraic());
                }
                _ => {
                    println!("Hello");
                }
            }
        }
    } else {
        parse_single_word(input, board);
    }

    fn parse_single_word(input: &str, board: &Board) {
        match input {
            "uci" => {
                print_flush(include_str!("../extras/uci.txt"));
            }
            "isready" => {
                print_flush("readyok");
            }
            "quit" | "q" => {
                std::process::exit(0);
            }
            "board" => {
                // board.print();
                print!("{board}");
            }
            "castling" => {
                println!("{:?}", board.find_castling_moves());
            }
            "moves" => {
                let found_moves = board.find_all_moves();
                println!("Found {} moves", found_moves.len());
                println!("moves: \n{found_moves:?}");
            }
            "unchecked" => {
                let found_moves = board.find_unchecked_moves();
                println!("Found {} moves:", found_moves.len());
                println!("{found_moves:?}");
            }
            "go" => {
                let found_moves = board.find_all_moves();
                let chosen_move = choose_move(board, &found_moves);
                println!("bestmove {}", chosen_move.long_algebraic());
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
                    board.go_to_fen(include_str!("../extras/startpos.fen"));
                    // parse_more_moves(board, &mut input_tokens.split_off(1));
                    if let Some(moves) = input_tokens.get(1)
                        && *moves == "moves"
                    {
                        // parse_more_moves(board, &mut input_tokens.split_off(1));
                        board.trace_moves(input_tokens.split_off(2));
                    }
                }
                "fen" => {
                    // vec!["fen", "...", "...", "...", "moves", "e2e4", "e7e5q"]
                    if let Some(fen_end) = input_tokens.iter().position(|x| *x == "moves") {
                        let mut trace_moves = input_tokens.split_off(fen_end);
                        board.go_to_fen(input_tokens.split_off(1).join(" ").as_str());
                        board.trace_moves(trace_moves.split_off(1));
                    } else {
                        board.go_to_fen(input_tokens.split_off(1).join(" ").as_str());
                    }
                }
                "default" => {
                    board.go_to_fen(include_str!("../extras/startpos.fen"));
                }
                _ => {
                    wrongly_called();
                }
            }
        }
    }
}

fn wrongly_called() {
    // Do nothing, according to UCI protocol
}
