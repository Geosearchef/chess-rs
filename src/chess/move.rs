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
            moves.push(Move { src: coord, dst: pawn_push_coord, kind: MoveKind::Quiet }); // TODO: might be promote -> might be multiple -> set later?
            
            if (piece.color() == White && coord.1 == 6) || (piece.color() == Black && coord.1 == 1) {
                let pawn_double_push_coord = coord + y_offset * 2;
                if pawn_double_push_coord.is_on_board() && self.piece_at(pawn_double_push_coord).is_none() {
                    moves.push(Move { src: coord, dst: pawn_double_push_coord, kind: MoveKind::DoublePawnPush });
                }
            }
        }

        // Attack
        let x_offsets = [Vector(0, -1), Vector(0, 1)];
        let attack_coords = x_offsets.map(|off| coord + off);

        for attack_coord in attack_coords {
            if attack_coord.is_on_board() {
                let Some(victim) = self.piece_at(attack_coord) else {
                    continue;
                };

                if victim.color() != piece.color() {
                    moves.push(Move { src: coord, dst: attack_coord, kind: MoveKind::Capture  }); // TODO: might be promote -> might be multiple -> set later?
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
                        moves.push(Move { src: coord, dst: attack_coord, kind: MoveKind::Capture  }); // TODO: might be promote -> might be multiple -> set later?
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

        for dst in dsts {
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
    pub fn generate_bishop_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        vec![]
    }
    pub fn generate_rook_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        vec![]
    }
    pub fn generate_queen_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        vec![]
    }
    pub fn generate_king_moves(&self, coord: Vector, piece: Piece) -> Vec<Move> {
        vec![]
    }
}