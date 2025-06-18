use crate::chess::board::Board;
use crate::chess::board::Color::White;
use crate::chess::r#move::Move;

pub fn negamax_move(board: Board, depth: u8) -> Option<(Move, f64)> {
    let moves = board.generate_moves(board.next_player);

    let scores_by_move = moves.into_iter().map(|r#move| {
        let mut updated_board = board.clone();
        updated_board.execute_move(r#move);

        (r#move, -negamax(updated_board, depth - 1))
    });

    scores_by_move.max_by(|l, r| l.1.partial_cmp(&r.1).unwrap())
}

fn negamax(board: Board, depth: u8) -> f64 {
    if depth == 0 {
        return board.evaluate_position_for_current_player();
    }

    let next_player = board.next_player;
    let moves = board.generate_moves(next_player);

    let scores = moves.into_iter().map(|r#move| {
        if r#move.is_capture_king(&board) {
            return 200.0;
        }

        let mut updated_board = board.clone(); // TODO: get mutable board, make move, unmake move
        updated_board.execute_move(r#move);

        -negamax(updated_board, depth - 1)
    });

    scores.max_by(|l, r| l.partial_cmp(&r).unwrap()).unwrap_or(0.0) // draw evaluates to 0, but this would ignore king taken!!! -> abort early / take into account
}

// TODO: extract to generic?