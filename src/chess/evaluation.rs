use crate::chess::board::{Board, Piece, PieceType};
use crate::chess::board::Color::{Black, White};
use crate::chess::r#move::Move;

impl Board {
    pub fn evaluate_position_for_current_player(&self) -> f64 {
        let evaluation = self.evaluate_position();

        if self.next_player == White {
            evaluation
        } else {
            evaluation * -1.0
        }
    }

    pub fn evaluate_position(&self) -> f64 {
        let mut evaluation = 0.0;

        let white_moves = self.generate_moves(White); // TODO: could be used to detect check
        let black_moves = self.generate_moves(Black);

        evaluation += self.evaluate_material();
        evaluation += self.evaluate_mobility(&white_moves, &black_moves);
        evaluation += self.evaluate_castle();

        evaluation
    }

    fn evaluate_material(&self) -> f64 { // TODO: faster?
        self.squares.iter()
            .flat_map(|row|
                row.iter().map(|s| {
                    if let Some(p) = s {
                        p.piece_value()
                    } else {
                        0.0
                    }
                })
            ).sum()
    }

    fn evaluate_mobility(&self, white_moves: &Vec<Move>, black_moves: &Vec<Move>) -> f64 {
        (white_moves.len() as f64 - black_moves.len() as f64) * 0.1
    }

    fn evaluate_castle(&self) -> f64 {
        let mut evaluation = 0.0;

        if self.white_castled {
            evaluation += 0.7;
        }
        if self.black_castled {
            evaluation -= 0.7;
        }

        evaluation
    }

    // fn evaluate_check(&self, white_moves: &Vec<Move>, black_moves: &Vec<Move>) -> f64 { // could check if king -> return 200, but: very expensive
    //     if white_moves.iter().any(|m| self.piece_at(m.dst)) // check move kind for capture instead?
    // }

    // TODO: doubled, isolated, blocked pawns
    // TODO: development
}

impl Piece {
    // Piece value based on AlphaZero
    pub fn piece_value(&self) -> f64 {
        let base_value = match self.piece_type() {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.05,
            PieceType::Bishop => 3.33,
            PieceType::Rook => 5.63,
            PieceType::Queen => 9.5,
            PieceType::King => 200.0,
        };

        base_value * if self.is_white() { 1.0 } else { -1.0 }
    }
}