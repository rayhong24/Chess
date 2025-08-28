use crate::enums::Colour;
use crate::coords::Coords;

mod king;

pub trait Piece {
    fn colour(&self) -> Colour;
    fn get_representation(&self) -> char;
    fn get_destination_coords(&self, from: Coords) -> Vec<Coords>;
}