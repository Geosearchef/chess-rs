use crate::chess::{board::{Board, Piece}, r#move::{Move, MoveKind}, vector::Vector};
use crate::chess::board::PieceType::*;

#[allow(dead_code)]
pub mod board;
#[allow(dead_code)]
pub mod r#move;
#[allow(dead_code)]
pub mod vector;



impl Board {
    pub fn execute_move(&mut self, r#move: Move) {
        let src_square = self.piece_at_mut(r#move.src);
        let mut piece = src_square.expect("expected move to be valid");

        piece.set_moved();

        // remove src piece
        *src_square = None;

        let dst_square = self.piece_at_mut(r#move.dst);

        // Move to destination
        *dst_square = Some(piece);

        // TODO: update zobrist hash
        // src
        // dst
        // dst piece (detectable from move.kind == Capture / EPCapture / CapturePromote)

        // Castling - not recorded as last move, as the king move encodes all information
        if r#move.kind == MoveKind::QueenCastle {
            self.execute_castle(&r#move, 0, 3);
        } else if r#move.kind == MoveKind::KingCastle {
            self.execute_castle(&r#move, 7, 5);
        }

        // En passant
        if r#move.kind == MoveKind::EPCapture {
            let captured_coord = r#move.dst + if r#move.dst.1 == 5 { Vector(0, -1) } else { Vector(0, 1) };
            *self.piece_at_mut(captured_coord) = None;

            // TODO: zobrist
        }

        // Promotion
        let promote_to = match r#move.kind {
            MoveKind::PromotionKnight => Some(Knight),
            MoveKind::PromotionBishop => Some(Bishop),
            MoveKind::PromotionRook => Some(Rook),
            MoveKind::PromotionQueen => Some(Queen),
            MoveKind::CapturePromotionKnight => Some(Knight),
            MoveKind::CapturePromotionBishop => Some(Bishop),
            MoveKind::CapturePromotionRook => Some(Rook),
            MoveKind::CapturePromotionQueen => Some(Queen),
            _ => None,
        };
        if let Some(new_piece_type) = promote_to {
            let mut new_piece = Piece::new(piece.color(), new_piece_type);
            new_piece.set_moved();
            *self.piece_at_mut(r#move.dst) = Some(new_piece);

            // TODO: zobrist: remove old, add new
        }

        self.last_move = Some(r#move);
    }

    #[inline]
    fn execute_castle(&mut self, r#move: &Move, rook_src_x: i8, rook_dst_x: i8) {
        let rook = self.piece_at_mut(Vector(rook_src_x, r#move.src.1));
            let mut rook_piece = rook.expect("expected rook to be present for requested castling");
            *rook = None;

            rook_piece.set_moved();

            *self.piece_at_mut(Vector(rook_dst_x, r#move.src.1)) = Some(rook_piece);
    }
}