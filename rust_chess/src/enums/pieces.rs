use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    Pawn=0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub const PIECE_COUNT: usize = Piece::King as usize + 1;
