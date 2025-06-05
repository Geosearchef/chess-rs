use std::fmt::Display;

use crate::chess::Coord;

const COLOR_WHITE: u8 = 0b0000_0001;
const COLOR_BLACK: u8 = 0b0000_0010;
const TYPE_PAWN: u8 = 0b0000_0100;
const TYPE_KNIGHT: u8 = 0b0000_1000;
const TYPE_BISHOP: u8 = 0b0001_0000;
const TYPE_ROOK: u8 = 0b0010_0000;
const TYPE_QUEEN: u8 = 0b0100_0000;
const TYPE_KING: u8 = 0b1000_0000;

const SQUARE_EMPTY: u8 = 0;

pub struct Piece(u8);

impl Piece {
    pub fn new(color: u8, ptype: u8) -> Self {
        Self(color | ptype)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == SQUARE_EMPTY
    }

    pub fn is_white(&self) -> bool {
        (self.0 & COLOR_WHITE) != 0
    }
    pub fn is_black(&self) -> bool {
        (self.0 & COLOR_BLACK) != 0
    }

    pub fn is_pawn(&self) -> bool {
        (self.0 & TYPE_PAWN) != 0
    }
    pub fn is_knight(&self) -> bool {
        (self.0 & TYPE_KNIGHT) != 0
    }
    pub fn is_bishop(&self) -> bool {
        (self.0 & TYPE_BISHOP) != 0
    }
    pub fn is_rook(&self) -> bool {
        (self.0 & TYPE_ROOK) != 0
    }
    pub fn is_queen(&self) -> bool {
        (self.0 & TYPE_QUEEN) != 0
    }
    pub fn is_king(&self) -> bool {
        (self.0 & TYPE_KING) != 0
    }
}



type BoardIndex = u8;

pub struct Board {
    squares: [Piece; 64],
    white_king_move: bool,
    black_king_move: bool,
    white_castled: bool,
    black_castled: bool,
}

impl Board {
    pub fn index(coord: Coord) -> BoardIndex {
        coord.0 + coord.1 * 8
    }

    pub fn coords(index: BoardIndex) -> Coord {
        (index % 8, index / 8)
    }

    pub fn piece_at(&self, coord: Coord) -> &Piece {
        &self.squares[Board::index(coord) as usize]
    }

    pub fn piece_at_mut(&mut self, coord: Coord) -> &mut Piece {
        &mut self.squares[Board::index(coord) as usize]
    }
}

impl Default for Board {
    fn default() -> Self {
        let squares = [
            Piece::new(COLOR_BLACK, TYPE_ROOK), Piece::new(COLOR_BLACK, TYPE_KNIGHT), Piece::new(COLOR_BLACK, TYPE_BISHOP), Piece::new(COLOR_BLACK, TYPE_QUEEN), Piece::new(COLOR_BLACK, TYPE_KING), Piece::new(COLOR_BLACK, TYPE_BISHOP), Piece::new(COLOR_BLACK, TYPE_KNIGHT), Piece::new(COLOR_BLACK, TYPE_ROOK),
            Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN), Piece::new(COLOR_BLACK, TYPE_PAWN),
            Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY),
            Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY),
            Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY),
            Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY), Piece(SQUARE_EMPTY),
            Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN), Piece::new(COLOR_WHITE, TYPE_PAWN),
            Piece::new(COLOR_WHITE, TYPE_ROOK), Piece::new(COLOR_WHITE, TYPE_KNIGHT), Piece::new(COLOR_WHITE, TYPE_BISHOP), Piece::new(COLOR_WHITE, TYPE_QUEEN), Piece::new(COLOR_WHITE, TYPE_KING), Piece::new(COLOR_WHITE, TYPE_BISHOP), Piece::new(COLOR_WHITE, TYPE_KNIGHT), Piece::new(COLOR_WHITE, TYPE_ROOK),
        ];
        Self {
            squares,
            white_king_move: false,
            black_king_move: false,
            white_castled: false,
            black_castled: false,
        }
    }
}




impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " ")?;
        } else if self.is_white() {
            if self.is_pawn() {
                write!(f, "♙")?;
            } else if self.is_knight() {
                write!(f, "♘")?;
            } else if self.is_bishop() {
                write!(f, "♗")?;
            } else if self.is_rook() {
                write!(f, "♖")?;
            } else if self.is_queen() {
                write!(f, "♕")?;
            } else if self.is_king() {
                write!(f, "♔")?;
            } else {
                write!(f, "?")?;
            }
        } else if self.is_black() {
            if self.is_pawn() {
                write!(f, "♟")?;
            } else if self.is_knight() {
                write!(f, "♞")?;
            } else if self.is_bishop() {
                write!(f, "♝")?;
            } else if self.is_rook() {
                write!(f, "♜")?;
            } else if self.is_queen() {
                write!(f, "♛")?;
            } else if self.is_king() {
                write!(f, "♚")?;
            } else {
                write!(f, "?")?;
            }
        } else {
            write!(f, "?")?;
        }

        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0u8..8u8 {
            for x in 0u8..8u8 {
                write!(f, "{}", self.piece_at((x, y)))?;
                if x != 7 {
                    write!(f, " ")?;
                }
            }
            if y != 7 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

