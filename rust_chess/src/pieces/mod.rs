use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;

pub mod king;
pub mod pawn;
pub mod rook;
pub mod bishop;
pub mod queen;

pub trait Piece {
    fn colour(&self) -> Colour;
    fn get_representation(&self) -> char;
    fn get_destination_coords(&self, from: Coords) -> Vec<MoveRay>;
}