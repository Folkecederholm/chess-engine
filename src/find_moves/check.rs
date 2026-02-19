use crate::types::defs::*;

#[allow(clippy::match_wild_err_arm)]
pub fn is_move_allowed(chess_move: ChessMove, board: &Board) -> bool {
    // let state = {
    //     let mut x = board.clone();
    //     match x.make_move(*chess_move) {
    //         Ok(()) => {}
    //         Err(_) => panic!("This should be unreachable. find_moves/check.rs:8"),
    //     }
    //     x
    // };
    // println!("Calling find_all_moves();");
    // let answers = state.find_unchecked_moves(); // You can put your own king in check if you take your opponent's king!!!
    // let king = board.get_king(); // The king of the person who's yet to make a move.
    // for answer in answers {
    //     if answer.end() == king {
    //         return false;
    //     }
    // }
    // true
    /**/
    // use std::time::UNIX_EPOCH;
    // let rand = match std::time::SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_micros()
    //     % 2
    // {
    //     1 => true,
    //     0 => false,
    //     _ => {
    //         unreachable!()
    //     }
    // };
    // rand
    /* */
    println!("move: {chess_move:?}");
    let state = {
        let mut x = board.clone();
        x.make_move(chess_move).unwrap(); // .unwrap() OK since we got the move because it's OK
        x
    };
    let moves = state.find_unchecked_moves();
    let to_ret = moves.iter().all(|x| x.end() != state.get_king());
    println!("to_ret: {to_ret}");
    to_ret
}
