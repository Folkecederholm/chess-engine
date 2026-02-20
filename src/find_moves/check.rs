use crate::types::defs::*;

#[allow(clippy::match_wild_err_arm)]
pub fn is_move_allowed(chess_move: ChessMove, board: &Board) -> bool {
    let state = {
        let mut x = board.clone();
        x.make_move(chess_move).unwrap(); // .unwrap() OK since we got the move because it's OK
        x
    };
    let moves = state.find_unchecked_moves();
    let to_ret = moves.iter().all(|x| x.end() != state.get_king());
    to_ret
}
