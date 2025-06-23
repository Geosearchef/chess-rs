use egui::emath::OrderedFloat;
use crate::chess::board::Board;
use crate::chess::r#move::Move;

impl Move {
    // Move ordering - lower is better
    pub fn order_score(&self, board: &Board) -> OrderedFloat<f64> {
        // last moved piece, least valuable attacker

        if let Some(Move { dst, .. }) = board.last_move {
            if dst == self.dst {
                return OrderedFloat(board.piece_at(self.src).unwrap().piece_value()); // unwrap - has to be valid moves
            }

            OrderedFloat(f64::INFINITY)
        } else {
            OrderedFloat(f64::INFINITY)
        }

        // TODO: alternatively: consider MVV-LVA (this needs to query the board)
        // TODO: could use some improvement
    }
}