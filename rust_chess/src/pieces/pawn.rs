use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Pawn {
    pub colour: Colour,
}

impl Pawn {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Piece for Pawn {
    fn colour(&self) -> Colour {
        self.colour
    }

    fn get_representation(&self) -> char {
        match self.colour {
            Colour::White => 'P',
            Colour::Black => 'p',
        }
    }

    fn get_destination_coords(&self, coords: Coords) -> Vec<MoveRay> {
        let mut moves = Vec::new();
        let direction: i8 = if self.colour == Colour::White { 1 } else { -1 };

        // Forward moves
        let moves_forward = if (coords.rank == 2 && self.colour == Colour::White)
            || (coords.rank == 7 && self.colour == Colour::Black)
        {
            2
        } else {
            1
        };

        moves.push(MoveRay {
            rank_diff: direction,
            file_diff: 0,
            dist: moves_forward,
            capture_allowed: false,
            capture_forced: false,
        });

        // Capture left
        moves.push(MoveRay {
            rank_diff: direction,
            file_diff: -1,
            dist: 1,
            capture_allowed: true,
            capture_forced: true,
        });

        // Capture right
        moves.push(MoveRay {
            rank_diff: direction,
            file_diff: 1,
            dist: 1,
            capture_allowed: true,
            capture_forced: true,
        });

        moves
    }
}