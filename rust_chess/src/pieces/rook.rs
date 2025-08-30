use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;
use crate::pieces::Piece;


#[derive(Debug, Clone)]
pub struct Rook {
    pub colour: Colour,
}   

impl Rook {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Piece for Rook {
    fn colour(&self) -> Colour {
        self.colour
    }

    fn get_representation(&self) -> char {
        match self.colour {
            Colour::White => 'R',
            Colour::Black => 'r',
        }
    }

    fn get_destination_coords(&self, _coords: Coords) -> Vec<MoveRay> {
        let mut move_rays = Vec::new();

        for (di, dj )in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            move_rays.push(MoveRay {
                rank_diff: *di,
                file_diff: *dj,
                dist: 7,
                capture_allowed: true,
                capture_forced: false,
            });
        }

        return move_rays;
    }
}