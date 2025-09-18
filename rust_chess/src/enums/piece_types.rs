use std::fmt;

use strum_macros::EnumIter;
use crate::{enums::Colour, piece};

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn=0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub const PIECE_COUNT: usize = PieceType::King as usize + 1;

impl PieceType {
    pub fn value(&self) -> i32{
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };


        write!(f, "{}", symbol)
    }
}