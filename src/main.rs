fn main() {
    let mut input = String::new();
    loop {
        take_user_input(&mut input);
        parse_user_input(input.as_str());
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

fn parse_user_input(input: &str) {
    if !input.contains(" ") {
        parse_single_word(input);
    } else {
        // Mutable so that we can remove the already known stuff when parsing later
        let mut tokens: Vec<_> = input.split(' ').collect();
        if let Some(first) = tokens.first() {
            match *first {
                "position" => {
                    let _ = tokens.remove(0);
                    parse_position(tokens);
                }
                _ => {
                    println!("Hello");
                }
            }
        }
    }

    fn parse_single_word(input: &str) {
        match input {
            "uci" => {
                print_flush(include_str!("strings/uci.txt"));
            }
            "isready" => {
                print_flush("readyok");
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {
                wrongly_called();
            }
        }
    }

    fn parse_position(tokens: Vec<&str>) {
        if let Some(first) = tokens.first() {
            match *first {
                "startpos" => {
                    println!("Startpos it is!");
                }
                "fen" => {
                    println!("fen is the way to go!");
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
