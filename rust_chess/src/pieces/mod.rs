use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;

mod king;

pub trait Piece {
    fn colour(&self) -> Colour;
    fn get_representation(&self) -> char;
    fn get_destination_coords(&self, from: Coords) -> Vec<MoveRay>;
}