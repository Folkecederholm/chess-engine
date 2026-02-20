use crate::types::defs::*;

#[allow(clippy::cast_possible_truncation)]
pub fn choose_move(_board: &Board, moves: Vec<ChessMove>) -> ChessMove {
    use std::time;
    let max = moves.len();
    let moment = time::Instant::now().elapsed().as_nanos();
    let chosen = (moment % max as u128) as usize;
    moves[chosen]
}
