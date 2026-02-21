use std::f64;

use crate::types::defs::*;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::needless_pass_by_value)]
pub fn choose_move(board: &Board, moves: Vec<ChessMove>) -> ChessMove {
    let mut scores = vec![];
    let mut lowest: f64 = f64::INFINITY;
    let mut lowest_index: usize = 0; // Default: choose first move in list
    for (i, chess_move) in moves.iter().enumerate() {
        let score = eval_move(board, *chess_move);
        scores.push(score);
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
#[allow(clippy::cast_precision_loss)]
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
    let mut lowest = f64::INFINITY;
    for x in 1..8 {
        for y in 1..8 {
            let score = eval_tile(
                state.get_tile(Coord::xy(x, y)),
                Coord::xy(x, y),
                _board.get_colour_turn(),
            );
            println!("info string move {_chess_move:?} score {score}");
            if score < lowest {
                lowest = score;
            }
        }
    }
    lowest
}

fn eval_tile(tile: Tile, coord: Coord, colour: Colour) -> f64 {
    use crate::extras::patterns::*;
    let Some(piece) = tile.get_piece() else {
        return 0.0;
    };
    let pawn_pattern = match colour {
        Colour::White => PAWN_PATTERN_WHITE,
        Colour::Black => PAWN_PATTERN_BLACK,
    };
    -match piece.piece_type {
        PieceType::Pawn => pawn_pattern[coord.one_d_coord()],
        PieceType::Queen => 9.0,
        PieceType::Rook => 5.0,
        PieceType::Bishop => 3.2,
        PieceType::Knight => 2.9,
        PieceType::King => 100.0, // It's pretty good to have a king
    }
}
