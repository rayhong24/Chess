use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::{self, MoveRay};
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Queen {
    pub colour: Colour,
}

impl Queen {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Piece for Queen {
    fn colour(&self) -> Colour {
        self.colour
    }

    fn get_representation(&self) -> char {
        match self.colour {
            Colour::White => 'Q',
            Colour::Black => 'q',
        }
    }

    fn get_destination_coords(&self, from: Coords) -> Vec<MoveRay> {
        let mut move_rays = Vec::new();

        for (di, dj) in &[
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1)         , ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ] {
            move_rays.push(MoveRay {
                rank_diff: *di,
                file_diff: *dj,
                dist: 7,
                capture_allowed: true,
                capture_forced: false,
            });
        }
    }
}