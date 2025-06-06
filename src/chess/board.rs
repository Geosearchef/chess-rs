use std::{fmt::Display, num::NonZeroU8};
use itertools::Itertools;

use crate::chess::Coord;

const PIECE_TYPE_MASK: u8 = 0b0000_0111;
const PIECE_COLOR_MASK: u8 = 0b0000_1000;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = PIECE_COLOR_MASK * 0,
    Black = PIECE_COLOR_MASK * 1,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::White,
            PIECE_COLOR_MASK => Self::Black,
            _ => unreachable!()
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl From<u8> for PieceType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Pawn,
            2 => Self::Knight,
            3 => Self::Bishop,
            4 => Self::Rook,
            5 => Self::Queen,
            6 => Self::King,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece(NonZeroU8);

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        unsafe {
            Self(NonZeroU8::new_unchecked(color as u8 | piece_type as u8))
        }
    }

    #[inline]
    pub fn color_code(&self) -> u8 {
        self.0.get() & PIECE_COLOR_MASK
    }

    #[inline]
    pub fn piece_type_code(&self) -> u8 {
        self.0.get() & PIECE_TYPE_MASK
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color_code().into()
    }

    #[inline]
    pub fn piece_type(&self) -> PieceType {
        self.piece_type_code().into()
    }

    pub fn is_white(&self) -> bool {
        matches!(self.color(), Color::White)
    }
    pub fn is_black(&self) -> bool {
        matches!(self.color(), Color::Black)
    }

    pub fn is_pawn(&self) -> bool {
        matches!(self.piece_type(), PieceType::Pawn)
    }
    pub fn is_knight(&self) -> bool {
        matches!(self.piece_type(), PieceType::Knight)
    }
    pub fn is_bishop(&self) -> bool {
        matches!(self.piece_type(), PieceType::Bishop)
    }
    pub fn is_rook(&self) -> bool {
        matches!(self.piece_type(), PieceType::Rook)
    }
    pub fn is_queen(&self) -> bool {
        matches!(self.piece_type(), PieceType::Queen)
    }
    pub fn is_king(&self) -> bool {
        matches!(self.piece_type(), PieceType::King)
    }
}



type BoardIndex = u8;

pub struct Board {
    squares: [[Option<Piece>; 8]; 8], // encoded as Option<Piece<NonZeroU8>> which is u8 with 0 representing None, Rust is cool
    white_king_move: bool,
    black_king_move: bool,
    white_castled: bool,
    black_castled: bool,
}

impl Board {
    pub fn piece_at(&self, coord: Coord) -> &Option<Piece> {
        &self.squares[coord.1 as usize][coord.0 as usize]
    }

    pub fn piece_at_mut(&mut self, coord: Coord) -> &mut Option<Piece> {
        &mut self.squares[coord.1 as usize][coord.0 as usize]
    }
}

impl Default for Board {
    fn default() -> Self {
        use Color::*;
        use PieceType::*;

        let base_row = |color| {
            [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook].map(|t| Some(Piece::new(color, t)))
        };
        let pawn_row = |color| {
            [Pawn; 8].map(|t| Some(Piece::new(color, t)))
        };
        let empty_row = [None; 8];

        let squares = [
            base_row(Black),
            pawn_row(Black),
            empty_row,
            empty_row,
            empty_row,
            empty_row,
            pawn_row(White),
            base_row(White),
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
        let char = if self.is_white() {
            match self.piece_type() {
                PieceType::Pawn => "♙",
                PieceType::Knight => "♘",
                PieceType::Bishop => "♗",
                PieceType::Rook => "♖",
                PieceType::Queen => "♕",
                PieceType::King => "♔",
            }
        } else if self.is_black() {
            match self.piece_type() {
                PieceType::Pawn => "♟",
                PieceType::Knight => "♞",
                PieceType::Bishop => "♝",
                PieceType::Rook => "♜",
                PieceType::Queen => "♛",
                PieceType::King => "♚",
            }
        } else {
            "?"
        };

        write!(f, "{char}")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.squares.iter().map(|row| {
            row.iter().map(|square| {
                match square {
                    Some(p) => p.to_string(),
                    None => " ".to_string(),
                }
            }).join(" ")
        }).join("\n");

        write!(f, "{str}")
    }
}

