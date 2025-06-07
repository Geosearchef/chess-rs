use crate::chess::board::{Board, Color};
use rand::prelude::*;

mod chess;

fn main() {
    let mut board = Board::default();

    let moves = board.generate_moves(Color::White);
    println!("{} moves found", moves.len());

    board.execute_move(*moves.choose(&mut rand::rng()).unwrap());


    println!("{}", board);
}
