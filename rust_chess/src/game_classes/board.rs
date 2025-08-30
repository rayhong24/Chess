use crate::enums::Colour;
use crate::coords::Coords;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Board {
    pub turn: Colour,
}