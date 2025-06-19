use crate::chess::board::Board;
use crate::chess::board::Color::White;
use crate::chess::r#move::Move;
use crate::chess::transposition::TranspositionTable;
use crate::chess::zobrist::ZobristTable;

#[derive(Default, Clone)]
struct OptimizationContext {
    zobrist_table: ZobristTable,
    transposition_table: TranspositionTable,
} // persists with depth information

pub fn negamax_move(board: Board, depth: u8) -> Option<(Move, f64)> {
    let mut ctx = OptimizationContext::default();

    let moves = board.generate_moves(board.next_player);

    let scores_by_move: Vec<(Move, f64)> = moves.into_iter().map(|r#move| {
        let mut updated_board = board.clone();
        updated_board.execute_move(r#move, &ctx.zobrist_table);

        (r#move, -negamax(updated_board, depth - 1, &mut ctx)) // clone context for parallel or use Arc<Mutex<_>>
    }).collect();

    println!("{}", ctx.transposition_table);

    scores_by_move.into_iter().max_by(|l, r| l.1.partial_cmp(&r.1).unwrap())
}

fn negamax(board: Board, depth: u8, ctx: &mut OptimizationContext) -> f64 {
    // Lookup transposition table
    if let Some(score) = ctx.transposition_table.lookup(board.zobrist_hash, depth) {
        return *score;
    }

    if depth == 0 {
        let score = board.evaluate_position_for_current_player();

        ctx.transposition_table.insert(board.zobrist_hash, depth, score);

        return score;
    }

    let next_player = board.next_player;
    let moves = board.generate_moves(next_player);

    let scores = moves.into_iter().map(|r#move| {
        if r#move.is_capture_king(&board) {
            return 200.0;
        }

        let mut updated_board = board.clone(); // TODO: get mutable board, make move, unmake move
        updated_board.execute_move(r#move, &ctx.zobrist_table);

        -negamax(updated_board, depth - 1, ctx)
    });

    let score = scores.max_by(|l, r| l.partial_cmp(&r).unwrap()).unwrap_or(0.0); // draw evaluates to 0, but this would ignore king taken!!! -> abort early / take into account

    ctx.transposition_table.insert(board.zobrist_hash, depth, score);

    score
}




// TODO: extract to generic?