use crate::chess::board::Board;


mod chess;

fn main() {
    let board = Board::default();

    println!("{}", board);
}
