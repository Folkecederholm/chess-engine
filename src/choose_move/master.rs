use std::f64;

use crate::types::defs::*;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::needless_pass_by_value)]
pub fn choose_move(board: &Board, moves: Vec<ChessMove>) -> ChessMove {
    let mut scores = vec![];
    let mut lowest: f64 = f64::INFINITY;
    let mut lowest_index: usize = 7; // Default: choose first move in list
    for (i, chess_move) in moves.iter().enumerate() {
        let score = eval_move(board, *chess_move, 0);
        scores.push(score);
        println!("score: {score}");
        if score < lowest {
            lowest = score;
            lowest_index = i;
        }
    }
    let Some(to_ret) = moves.get(lowest_index) else {
        unreachable!()
    };
    *to_ret
}

// Lower score = better
/*
fn eval_move(_board: &Board, _chess_move: ChessMove) -> f64 {
    // use std::time;
    // const MAX: f64 = 8_908_439_407_189f64;
    // const MOD: f64 = 17f64;
    // let moment = time::Instant::now().elapsed().as_nanos();
    // moment as f64 % MAX / MOD
    let state = {
        let mut s = _board.clone();
        s.make_move(_chess_move).unwrap();
        s
    };
    // How many moves your opponent can do after this move
    state.find_all_moves().len() as f64 / 100f64
}
*/

fn eval_move(board: &Board, chess_move: ChessMove, depth: u8) -> f64 {
    if depth == 0 {
        let state = {
            let mut s = board.clone();
            s.make_move(chess_move).unwrap();
            s
        };
        state.find_all_moves().len() as f64 / state.find_unchecked_moves().len() as f64
    } else {
        let state = {
            let mut s = board.clone();
            s.make_move(chess_move).unwrap();
            s
        };
        let list = state.find_all_moves();
        let goodness: Vec<f64> = list
            .iter()
            .map(|&x| eval_move(&state, x, depth - 1))
            .collect();
        {
            // let len = goodness.len() as f64;
            // let mut acc = 0.0;
            // for move_potential in goodness {
            //     acc -= move_potential;
            // }
            // acc / len
            let mut highest = f64::NEG_INFINITY;
            for moves_potential in goodness {
                if moves_potential > highest {
                    highest = moves_potential;
                };
            }
            highest
        }
    }
}
