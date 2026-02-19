use crate::types::defs::*;

#[allow(clippy::match_wild_err_arm)]
pub fn is_move_allowed(chess_move: &ChessMove, board: &Board) -> bool {
    let state = {
        let mut x = board.clone();
        match x.make_move(*chess_move) {
            Ok(()) => {}
            Err(_) => panic!("This should be unreachable. find_moves/check.rs:8"),
        }
        x
    };
    println!("move: {chess_move:?}");
    if !king_sees(*chess_move, board) {
        // If the king doesn't see the piece, it can't block an enemy.
        // That is, it can move away without making an enemy piece threaten the king.
        println!("\x1b[31mThis isn't visible by the king! {chess_move:?}\x1b[0m");
        return true;
    }
    println!("Calling find_all_moves();");
    let answers = state.find_all_moves();
    let king = state.get_king(); // The king of the person who's yet to make a move.
    for answer in answers {
        if answer.end() == king {
            let two_moves_forward = {
                let mut x = state.clone();
                match x.make_move(answer) {
                    Ok(()) => {}
                    Err(_) => unreachable!(),
                }
                x
            };
            println!("->");
            return !is_move_allowed(&answer, &two_moves_forward);
        }
    }
    println!("<-");
    return true;
    // This function is defect. It's defnct.
    fn king_sees(chess_move: ChessMove, board: &Board) -> bool {
        fn king_can_see(board: &Board, tile: Tile) -> bool {
            // Maybe it's this part that doesn't work correctly
            if let Some(piece) = tile.get_piece() {
                piece.colour == board.get_colour_turn()
                // false
            } else {
                true
            }
        }
        println!("board: {board}");
        let a = board.find_sliding_moves(
            // &chess_move.end(),
            &board.get_king(),
            Slider {
                x: 1,
                y: 0,
                slide: true,
                move_fn: king_can_see,
            },
        );
        let b = board.find_sliding_moves(
            // &chess_move.end(),
            &board.get_king(),
            Slider {
                x: 1,
                y: 1,
                slide: true,
                move_fn: king_can_see,
            },
        );
        (!a.is_empty()) || (!b.is_empty()) // There is at least one move that takes the king to a piece of the own colour
        // I think
    }
}
