use std::vec;

use crate::chess::{board::{Board, Color, Piece}, vector::Vector};
use crate::chess::board::PieceType::*;
use crate::chess::board::Color::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub src: Vector,
    pub dst: Vector,
    pub kind: MoveKind, // this could be packaged in 16 bits total, spending time shifting bits for obtaining the coordinate indices
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum MoveKind {
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Capture,
    EPCapture,
    PromotionKnight,
    PromotionBishop,
    PromotionRook,
    PromotionQueen,
    CapturePromotionKnight,
    CapturePromotionBishop,
    CapturePromotionRook,
    CapturePromotionQueen,
}

impl MoveKind {
    fn is_capture_with_target(&self) -> bool {
        match self {
            MoveKind::Capture => true,
            MoveKind::CapturePromotionKnight => true,
            MoveKind::CapturePromotionBishop => true,
            MoveKind::CapturePromotionRook => true,
            MoveKind::CapturePromotionQueen => true,
            _ => false
        }
    }
}

impl Board {
    pub fn generate_moves(&self, player: Color) -> Vec<Move> { // TODO: check for legal, this is just pseudo legal
        self.coords_with_piece_of_color(player)
            .flat_map(|coord| self.generate_piece_moves(coord).into_iter())
            .collect()
    }

    pub fn generate_piece_moves(&self, coord: Vector) -> Vec<Move> {
        let piece = self.piece_at(coord).expect("Only coords with piece can be supplied");
        match piece.piece_type() {
            Pawn => self.generate_pawn_moves(coord, piece),
            Knight => self.generate_knight_moves(coord, piece),
            Bishop => self.generate_bishop_moves(coord, piece),
            Rook => self.generate_rook_moves(coord, piece),
            Queen => self.generate_queen_moves(coord, piece),
            King => self.generate_king_moves(coord, piece),
        }
    }

    pub fn generate_pawn_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        let mut moves = vec![];

        let y_offset = if piece.color() == White {
            Vector(0, -1)
        } else {
            Vector(0, 1)
        };

        // Pawn push
        let pawn_push_coord = coord + y_offset;

        if pawn_push_coord.is_on_board() && self.piece_at(pawn_push_coord).is_none() {
            if pawn_push_coord.1 == 0 || pawn_push_coord.1 == 7 {
                for kind in [MoveKind::PromotionKnight, MoveKind::PromotionBishop, MoveKind::PromotionRook, MoveKind::PromotionQueen] {
                    moves.push(Move { src: coord, dst: pawn_push_coord, kind })
                }
            } else {
                moves.push(Move { src: coord, dst: pawn_push_coord, kind: MoveKind::Quiet });
            }
            
            if (piece.color() == White && coord.1 == 6) || (piece.color() == Black && coord.1 == 1) {
                let pawn_double_push_coord = coord + y_offset * 2;
                if pawn_double_push_coord.is_on_board() && self.piece_at(pawn_double_push_coord).is_none() {
                    moves.push(Move { src: coord, dst: pawn_double_push_coord, kind: MoveKind::DoublePawnPush });
                }
            }
        }

        // Attack
        let x_offsets = [Vector(-1, 0), Vector(1, 0)];
        let offsets = x_offsets.map(|off| off + y_offset);
        let attack_coords = offsets.map(|off| coord + off);

        for attack_coord in attack_coords {
            if attack_coord.is_on_board() {
                let Some(victim) = self.piece_at(attack_coord) else {
                    continue;
                };

                if victim.color() != piece.color() {
                    if pawn_push_coord.1 == 0 || pawn_push_coord.1 == 7 {
                        for kind in [MoveKind::CapturePromotionKnight, MoveKind::CapturePromotionBishop, MoveKind::CapturePromotionRook, MoveKind::CapturePromotionQueen] {
                            moves.push(Move { src: coord, dst: attack_coord, kind })
                        }
                    } else {
                        moves.push(Move { src: coord, dst: attack_coord, kind: MoveKind::Capture });
                    }
                }
            }
        }

        // en passant
        if let Some(Move { kind: MoveKind::DoublePawnPush, dst: last_move_dst, .. }) = self.last_move {
            for attack_coord in attack_coords {
                if attack_coord.is_on_board() {
                    let attacked_piece_coord = Vector(attack_coord.0, coord.1);
                    let Some(victim) = self.piece_at(attacked_piece_coord) else {
                        continue;
                    };

                    if victim.color() != piece.color() && last_move_dst == attacked_piece_coord {
                        moves.push(Move { src: coord, dst: attack_coord, kind: MoveKind::EPCapture });
                    }
                }
            }
        }

        moves
    }

    pub fn generate_knight_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        let mut moves = vec![];

        let offsets = [
            Vector(-2, -1), Vector(-1, -2),
            Vector( 2, -1), Vector(-1,  2),
            Vector(-2,  1), Vector( 1, -2),
            Vector( 2,  1), Vector( 1,  2),
        ];

        let dsts = offsets.into_iter().map(|off| coord + off);

        moves.append(&mut self.generate_moves_to_squares(coord, piece, dsts));

        moves
    }

    pub fn generate_bishop_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        let mut moves = vec![];

        let offsets = [
            Vector(-1, -1),
            Vector( 1, -1),
            Vector(-1,  1),
            Vector( 1,  1),
        ];

        moves.append(&mut self.generate_continous_moves_from_offsets(coord, piece, offsets));

        moves
    }

    pub fn generate_rook_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        let mut moves = vec![];

        let offsets = [
            Vector( 0, -1),
            Vector( 0,  1),
            Vector(-1,  0),
            Vector( 1,  0),
        ];

        moves.append(&mut self.generate_continous_moves_from_offsets(coord, piece, offsets));

        moves
    }

    pub fn generate_queen_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        let mut moves = vec![];
        
        moves.append(&mut self.generate_bishop_moves(coord, piece));
        moves.append(&mut self.generate_rook_moves(coord, piece));

        moves
    }

    pub fn generate_king_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        let mut moves = vec![];

        let offsets = [
            Vector( 0,  1),
            Vector( 1,  1),
            Vector( 1,  0),
            Vector( 1, -1),
            Vector( 0, -1),
            Vector(-1, -1),
            Vector(-1,  0),
            Vector(-1,  1),
        ];

        let dsts = offsets.into_iter().map(|off| coord + off);

        moves.append(&mut self.generate_moves_to_squares(coord, piece, dsts));

        // Castling - TODO: missing check for blocking checks
        if !self.castled(piece.color()) {
            let y: i8 = if piece.color() == White { 7 } else { 0 };

            // Castle left (queen side)
            if let (Some(king_piece), Some(rook_piece)) = (self.piece_at(Vector(4, y)), self.piece_at(Vector(0, y))) {
                let our_king = king_piece.piece_type() == King && king_piece.color() == piece.color();
                let our_rook = rook_piece.piece_type() == Rook && rook_piece.color() == piece.color();
                let empty_between = self.piece_at(Vector(1, y)).is_none() && self.piece_at(Vector(2, y)).is_none() && self.piece_at(Vector(3, y)).is_none(); // TODO: no checks on these squares
                if our_king && our_rook && empty_between {
                    moves.push(Move { src: coord, dst: Vector(2, y), kind: MoveKind::QueenCastle });
                }
            }

            // Castle right (king side)
            if let (Some(king_piece), Some(rook_piece)) = (self.piece_at(Vector(4, y)), self.piece_at(Vector(7, y))) {
                let our_king = king_piece.piece_type() == King && king_piece.color() == piece.color();
                let our_rook = rook_piece.piece_type() == Rook && rook_piece.color() == piece.color();
                let empty_between = self.piece_at(Vector(5, y)).is_none() && self.piece_at(Vector(6, y)).is_none(); // TODO: no checks on these squares
                if our_king && our_rook && empty_between {
                    moves.push(Move { src: coord, dst: Vector(6, y), kind: MoveKind::KingCastle });
                }
            }

        }

        moves
    }

    #[inline]
    fn generate_moves_to_squares<I>(&self, coord: Vector, piece: Piece, dsts: I) -> Vec<Move> where I: IntoIterator<Item = Vector> {
        let mut moves = vec![];

        for dst in dsts { // TODO: this is shared with the king: can I go there?
            if !dst.is_on_board() {
                continue;
            }

            if let Some(victim) = self.piece_at(dst) {
                if victim.color() != piece.color() {
                    moves.push(Move { src: coord, dst, kind: MoveKind::Capture });
                }
            } else {
                moves.push(Move { src: coord, dst, kind: MoveKind::Quiet });
            }
        }

        moves
    }

    #[inline]
    fn generate_continous_moves_from_offsets<I>(&self, coord: Vector, piece: Piece, offsets: I) -> Vec<Move> where I: IntoIterator<Item = Vector> {
        let mut moves = vec![];

        for offset in offsets {
            let mut dst = coord + offset;
            while dst.is_on_board() {
                if let Some(victim) = self.piece_at(dst) {
                    if victim.color() != piece.color() {
                        moves.push(Move { src: coord, dst: dst, kind: MoveKind::Capture });
                    }
                    break; // can't go further
                } else {
                    moves.push(Move { src: coord, dst: dst, kind: MoveKind::Quiet });
                }
                dst = dst + offset;
            }
        }

        moves
    }
}

impl Move {
    #[inline]
    pub fn is_capture_king(&self, board: &Board) -> bool {
        if !self.is_capture_with_target() {
            return false;
        }

        board.piece_at(self.dst).unwrap().is_king() // This move is marked as capture, there should be a piece at dst
    }

    #[inline]
    pub fn is_capture_with_target(&self) -> bool {
        self.kind.is_capture_with_target()
    }
}