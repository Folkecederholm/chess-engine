use std::f64::{self, INFINITY, NEG_INFINITY};

use crate::types::defs::*;

#[allow(clippy::needless_pass_by_value)]
pub fn choose_move(board: &Board, moves: Vec<ChessMove>) -> ChessMove {
    let scores = moves.iter().map(|&x| (x, eval_move(board, x, 2)));
    let Some(chosen) = (match board.get_colour_turn() {
        // Colour::White => scores.min_by_key(|x| x.1),
        // Colour::Black => scores.max_by_key(|x| x.1),
        Colour::White => scores.max_by(|x, y| x.1.partial_cmp(&y.1).unwrap()),
        Colour::Black => scores.min_by(|x, y| x.1.partial_cmp(&y.1).unwrap()),
    }) else {
        unreachable!() /* We'll always find a move, it will be fine !!!!1!!1!1 */
    };
    chosen.0
}

fn eval_move(board: &Board, chess_move: ChessMove, depth: usize) -> f64 {
    if depth == 0 {
        let piece = board.get_tile(chess_move.start()).get_piece().unwrap();
        let material = (match piece.colour {
            Colour::White => 1.0,
            Colour::Black => -1.0,
        }) * (match piece.piece_type {
            PieceType::Pawn => 1.0,
            PieceType::Bishop => 3.2,
            PieceType::Knight => 2.8,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => -2.0,
        });
        // let position = 2.0 - Coord::ay('a', 4).unwrap().manhattan(chess_move.end()) as f64 / 4.0;
        let manhattan = {
            let e4 = Coord::ay('e', 4).unwrap();
            e4.manhattan(chess_move.end()) as f64
        };
        let position = 2.0 - manhattan / 4.0;
        // println!("move: {chess_move:?}");
        // println!("position: {position}");
        // println!("manhattan: {manhattan}");
        let taken = match board.get_tile(chess_move.end()).get_piece() {
            Some(piece) => match piece.piece_type {
                PieceType::Pawn => 1.0,
                PieceType::Bishop => 3.2,
                PieceType::Knight => 2.8,
                PieceType::Rook => 5.0,
                PieceType::Queen => 9.0,
                PieceType::King => 999.9,
            },
            None => 0.0,
        } * match board.get_colour_turn() {
            Colour::White => 1.0,
            Colour::Black => -1.0,
        };
        // println!("piece: {piece:?}");
        // println!("material: {material}");
        // println!("{chess_move:?}/{}", material * position + taken);
        material * position + taken
    } else {
        // depth > 0
        // All the moves you opponent could answer with
        let state = {
            let mut s = board.clone();
            s.make_move(chess_move).unwrap();
            s
        };
        let answers = state.find_all_moves();
        let mut extreme = match state.get_colour_turn() {
            Colour::White => NEG_INFINITY,
            Colour::Black => INFINITY,
        };
        for answer in answers {
            let score = eval_move(&state, answer, depth - 1);
            if match state.get_colour_turn() {
                Colour::White => score > extreme,
                Colour::Black => score < extreme,
            } {
                // println!("from {extreme} to {score}");
                extreme = score;
                // println!("(new):{extreme}/{chess_move:?}");
            }
        }
        extreme
    }
}
