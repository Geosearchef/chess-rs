use crate::chess::{board::{Board, Piece}, r#move::{Move, MoveKind}, vector::Vector};
use crate::chess::board::Color::White;
use crate::chess::board::PieceType::*;
use crate::chess::zobrist::ZobristTable;

#[allow(dead_code)]
pub mod board;
#[allow(dead_code)]
pub mod r#move;
#[allow(dead_code)]
pub mod vector;
#[allow(dead_code)]
pub mod evaluation;

pub mod visualizer;
mod negamax;
mod zobrist;
mod transposition;
mod ordering;

impl Board {
    pub fn execute_move(&mut self, r#move: Move, zobrist_table: &ZobristTable) {
        let player = self.next_player;

        let src_square = self.piece_at_mut(r#move.src);
        let mut piece = src_square.expect("expected move to be valid");
        let moved_before = piece.moved();

        piece.set_moved();

        // remove src piece
        *src_square = None;

        // Update zobrist hash for move
        self.zobrist_hash ^= zobrist_table.piece_key(&r#move.src, &piece); // source
        if r#move.is_capture_with_target() { // victim
            self.zobrist_hash ^= zobrist_table.piece_key(&r#move.dst, &self.piece_at(r#move.dst).unwrap()); // capture moves expects piece to be present
        }
        self.zobrist_hash ^= zobrist_table.piece_key(&r#move.dst, &piece); // destination

        // Castling rights + zobrist
        if piece.is_king() && !moved_before {
            if self.left_castling_rights[player.zobrist_index()] {
                self.left_castling_rights[player.zobrist_index()] = false;
                self.zobrist_hash ^= zobrist_table.left_castle[self.next_player.zobrist_index()];
            }

            if self.right_castling_rights[player.zobrist_index()] {
                self.right_castling_rights[player.zobrist_index()] = false;
                self.zobrist_hash ^= zobrist_table.right_castle[self.next_player.zobrist_index()];
            }
        }
        if piece.is_rook() && !moved_before {
            if r#move.src.0 == 0 && self.left_castling_rights[player.zobrist_index()] {
                self.left_castling_rights[player.zobrist_index()] = false;
                self.zobrist_hash ^= zobrist_table.left_castle[self.next_player.zobrist_index()];
            } else if self.right_castling_rights[player.zobrist_index()] {
                self.right_castling_rights[player.zobrist_index()] = false;
                self.zobrist_hash ^= zobrist_table.right_castle[self.next_player.zobrist_index()];
            }
        }

        // Move to destination
        let dst_square = self.piece_at_mut(r#move.dst);
        *dst_square = Some(piece);

        // Castling - not recorded as last move, as the king move encodes all information
        if r#move.kind == MoveKind::QueenCastle {
            self.execute_castle(&r#move, 0, 3, zobrist_table);
        } else if r#move.kind == MoveKind::KingCastle {
            self.execute_castle(&r#move, 7, 5, zobrist_table);
        }

        // En passant
        if r#move.kind == MoveKind::EPCapture {
            let captured_coord = r#move.dst + if r#move.dst.1 == 5 { Vector(0, -1) } else { Vector(0, 1) };
            let captured_square = self.piece_at_mut(captured_coord);
            let captured_piece = captured_square.unwrap(); // capture -> there needs to be a piece there

            *captured_square = None;

            self.zobrist_hash ^= zobrist_table.piece_key(&captured_coord, &captured_piece);
        }

        // En passant rights - Zobrist Hashing
        if r#move.kind == MoveKind::DoublePawnPush {
            self.zobrist_hash ^= zobrist_table.en_passant_file[r#move.src.0 as usize]; // enable
        }
        if let Some(last_move) = self.last_move {
            if last_move.kind == MoveKind::DoublePawnPush {
                self.zobrist_hash ^= zobrist_table.en_passant_file[last_move.src.0 as usize]; // disable
            }
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

            // Update zobrist hash
            self.zobrist_hash ^= zobrist_table.piece_key(&r#move.dst, &piece); // remove moved
            self.zobrist_hash ^= zobrist_table.piece_key(&r#move.dst, &new_piece); // add promoted
        }

        self.last_move = Some(r#move);

        debug_assert_eq!(piece.color(), self.next_player); // assert that the move was executed by the current player

        self.next_player = self.next_player.other();
        self.zobrist_hash ^= zobrist_table.black_to_move_key;
    }

    #[inline]
    fn execute_castle(&mut self, r#move: &Move, rook_src_x: i8, rook_dst_x: i8, zobrist_table: &ZobristTable) {
        let src = Vector(rook_src_x, r#move.src.1);
        let dst = Vector(rook_dst_x, r#move.src.1);

        let rook = self.piece_at_mut(src);
        let mut rook_piece = rook.expect("expected rook to be present for requested castling");
        *rook = None;

        rook_piece.set_moved();


        // update zobrist hash
        self.zobrist_hash ^= zobrist_table.piece_key(&src, &rook_piece);
        self.zobrist_hash ^= zobrist_table.piece_key(&dst, &rook_piece);


        *self.piece_at_mut(dst) = Some(rook_piece);
    }
}