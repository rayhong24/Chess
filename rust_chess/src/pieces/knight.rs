use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Knight {
    pub colour: Colour,
}

impl Knight {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Piece for Knight {
    fn colour(&self) -> Colour {
        self.colour
    }

    fn get_representation(&self) -> char {
        match self.colour {
            Colour::White => 'N',
            Colour::Black => 'n',
        }
    }

    fn get_move_rays(&self, _coords: Coords) -> Vec<MoveRay> {
        let mut move_rays = Vec::new();

        for (rank_diff, file_diff) in &[
            (-2, -1), (-2, 1),
            (-1, -2), (-1, 2),
            ( 1, -2), ( 1, 2),
            ( 2, -1), ( 2, 1)
        ] {
            move_rays.push(MoveRay {
                rank_diff: *rank_diff,
                file_diff: *file_diff,
                dist: 1,
                capture_allowed: true,
                capture_forced: false,
            });
        }

        return move_rays;
    }
}

