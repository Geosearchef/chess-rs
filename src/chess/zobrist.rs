use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use crate::chess::board::Piece;
use crate::chess::vector::Vector;
// The hash of a new board is 0, preexisting piece are not taken into account

// white, black
#[derive(Clone)]
pub struct ZobristTable {
    pub piece_keys: [[[[u64; 2]; 6]; 8]; 8],
    pub black_to_move_key: u64,
    pub left_castle: [u64; 2],
    pub right_castle: [u64; 2],
    pub en_passant_file: [u64; 8],
}

impl Default for ZobristTable {
    fn default() -> Self {
        let seed = [
            0x38, 0xfc, 0xdd, 0xc3, 0xde, 0x1f, 0x00, 0x2a,
            0xe2, 0x48, 0x18, 0x69, 0xa0, 0x54, 0x25, 0x56,
            0xae, 0x8b, 0x51, 0x45, 0x91, 0xec, 0x8b, 0x6f,
            0x99, 0xe7, 0x6a, 0x71, 0x20, 0xaa, 0x72, 0xc4,
        ];
        let mut rng = ChaCha20Rng::from_seed(seed);

        let mut table = Self {
            piece_keys: Default::default(),
            black_to_move_key: Default::default(),
            left_castle: Default::default(),
            right_castle: Default::default(),
            en_passant_file: Default::default(),
        };

        table.piece_keys.iter_mut()
            .for_each(|row|
                row.iter_mut().for_each(|square|
                    square.iter_mut().for_each(|piece|
                        piece.iter_mut().for_each(|color|
                            *color = rng.random()
                        )
                    )
                )
            );

        table.black_to_move_key = rng.random();

        table.left_castle.iter_mut().for_each(|c| *c = rng.random());
        table.right_castle.iter_mut().for_each(|c| *c = rng.random());

        table.en_passant_file.iter_mut().for_each(|c| *c = rng.random());

        table
    }
}

impl ZobristTable {
    pub fn piece_key(&self, pos: &Vector, piece: &Piece) -> u64 {
        self.piece_keys[pos.1 as usize][pos.0 as usize][piece.piece_type().zobrist_index()][piece.color().zobrist_index()]
    }
}