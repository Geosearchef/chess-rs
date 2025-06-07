use crate::chess::{board::Board, r#move::{Move, MoveKind}, vector::Vector};

#[allow(dead_code)]
pub mod board;
#[allow(dead_code)]
pub mod r#move;
#[allow(dead_code)]
pub mod vector;



impl Board {
    pub fn execute_move(&mut self, r#move: Move) {
        let src_square = self.piece_at_mut(r#move.src);
        let piece = src_square.expect("expected move to be valid");

        // remove src piece
        *src_square = None;

        let dst_square = self.piece_at_mut(r#move.dst);

        // Move to destination
        *dst_square = Some(piece);

        // TODO: update zobrist hash
        // src
        // dst
        // dst piece (detectable from move.kind == Capture)

        // En passant
        if r#move.kind == MoveKind::EPCapture {
            let captured_coord = r#move.dst + if r#move.dst.1 == 5 { Vector(0, -1) } else { Vector(0, 1) };
            *self.piece_at_mut(captured_coord) = None;

            // TODO: zobrist
        }

        self.last_move = Some(r#move);
    }
}