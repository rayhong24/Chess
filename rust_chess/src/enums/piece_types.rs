use std::fmt;

use strum_macros::EnumIter;
use crate::enums::Colour;

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