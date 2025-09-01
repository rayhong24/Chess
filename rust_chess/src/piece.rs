use crate::enums::{Colour, PieceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceType,
    pub colour: Colour
}
