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
