use crate::enums::{Colour, PieceType};
use crate::moves::move_ray::MoveRay;
use crate::coords::Coords;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceType,
    pub colour: Colour
}

// impl fmt::Display for Piece {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // For example, print like "White Pawn" or "Black Queen"
//         write!(f, "{:?} {:?}", self.colour, self.kind)
//     }
// }

impl Piece {
    pub fn get_move_rays(&self, coords: &Coords) -> Vec<MoveRay> {
        match self.kind {
            PieceType::Pawn => {
                Self::pawn_move_rays(self.colour, coords)
            },
            PieceType::Knight => {
                Self::knight_move_rays(&self, coords)
            }
            PieceType::Bishop => {
                Self::bishop_move_rays(&self, coords)
            }
            PieceType::Rook => {
                Self::rook_move_rays(&self, coords)
            }
            PieceType::Queen => {
                Self::queen_move_rays(&self, coords)
            }
            PieceType::King => {
                Self::king_move_rays(&self, coords)
            }
        }
    }

    fn pawn_move_rays(colour: Colour, coords: &Coords) -> Vec<MoveRay> {
        let mut move_rays = Vec::new();

        let direction: i8 = if colour == Colour::White { 1 } else { -1 };

        // Forward moves
        let moves_forward = if (coords.rank == 2 && colour == Colour::White)
            || (coords.rank == 7 && colour == Colour::Black)
        {
            2
        } else {
            1
        };

        move_rays.push(MoveRay {
            rank_diff: direction,
            file_diff: 0,
            dist: moves_forward,
            capture_allowed: false,
            capture_forced: false,
        });

        // Capture left
        move_rays.push(MoveRay {
            rank_diff: direction,
            file_diff: -1,
            dist: 1,
            capture_allowed: true,
            capture_forced: true,
        });

        // Capture right
        move_rays.push(MoveRay {
            rank_diff: direction,
            file_diff: 1,
            dist: 1,
            capture_allowed: true,
            capture_forced: true,
        });

        move_rays
    }

    fn knight_move_rays(&self, _: &Coords) -> Vec<MoveRay> {
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
    
    fn bishop_move_rays(&self, _: &Coords) -> Vec<MoveRay> {
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

    fn rook_move_rays(&self, _: &Coords) -> Vec<MoveRay> {
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

    fn queen_move_rays(&self, _: &Coords) -> Vec<MoveRay> {
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

        return move_rays;
    }
    fn king_move_rays(&self, _: &Coords) -> Vec<MoveRay> {
        let mut move_rays = Vec::new();
        for (rank_diff, file_diff) in &[
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ] {
            let move_ray = MoveRay::new(*rank_diff, *file_diff, 1, true, false);
            move_rays.push(move_ray);
        }

        return move_rays;
    }
}

