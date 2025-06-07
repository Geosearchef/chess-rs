use crate::chess::board::{Board, Color};
use rand::prelude::*;

mod chess;

fn main() {
    let mut board = Board::default();

    for i in 0..20 {
        let moves = board.generate_moves(if i % 2 == 0 { Color::White } else { Color::Black });
        println!("{} moves found", moves.len());

        board.execute_move(*moves.choose(&mut rand::rng()).unwrap());


        println!("{}\n\n", board);
    }
}
