use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Bishop {
    pub colour: Colour,
}

impl Bishop {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Piece for Bishop {
    fn colour(&self) -> Colour {
        self.colour
    }

    fn get_representation(&self) -> char {
        match self.colour {
            Colour::White => 'B',
            Colour::Black => 'b',
        }
    }

    fn get_move_rays(&self, _coords: Coords) -> Vec<MoveRay> {
        let mut move_rays = Vec::new();

        for (di, dj )in &[(-1, -1), (-1, 1), (1, -1), (1, 1)] {
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