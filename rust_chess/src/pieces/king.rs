use std::fmt::DebugSet;

use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct King {
    pub colour: Colour,
    pub position: Coords,
}

impl Piece for King {
    fn colour(&self) -> Colour {
        self.colour
    }
    fn get_representation(&self) -> char {
        match self.colour {
            Colour::White => 'K',
            Colour::Black => 'k',
        }
    }
    fn get_move_rays(&self, from: Coords) -> Vec<MoveRay> {
        let mut destinations = Vec::new();
        for (rank_diff, file_diff) in &[
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ] {
            if let Some(new_coords) = from.get_neighbour(*rank_diff, *file_diff) {
                let move_ray = MoveRay::new(*rank_diff, *file_diff, 1, true, false);
                destinations.push(move_ray);
            }
        }

        return destinations;
    }
}