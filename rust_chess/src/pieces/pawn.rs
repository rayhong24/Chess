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

    fn get_move_rays(&self, coords: Coords) -> Vec<MoveRay> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;
    #[test]
    fn test_pawn_representation() {
        let white_pawn = Pawn::new(Colour::White);
        let black_pawn = Pawn::new(Colour::Black);
        assert_eq!(white_pawn.get_representation(), 'P');
        assert_eq!(black_pawn.get_representation(), 'p');
    }

    #[test]
    fn test_pawn_moves_white_initial() {
        let pawn = Pawn::new(Colour::White);
        let coords = Coords::new(2, File::E);
        let moves = pawn.get_move_rays(coords);

        // Forward move should allow 2 squares
        let forward = moves.iter().find(|m| m.file_diff == 0).unwrap();
        assert_eq!(forward.rank_diff, 1);
        assert_eq!(forward.dist, 2);

        // Captures
        let left = moves.iter().find(|m| m.file_diff == -1).unwrap();
        let right = moves.iter().find(|m| m.file_diff == 1).unwrap();
        assert_eq!(left.rank_diff, 1);
        assert_eq!(right.rank_diff, 1);
        assert!(left.capture_allowed && left.capture_forced);
        assert!(right.capture_allowed && right.capture_forced);
    }

    #[test]
    fn test_pawn_moves_black_initial() {
        let pawn = Pawn::new(Colour::Black);
        let coords = Coords::new(7, File::E);
        let moves = pawn.get_move_rays(coords);

        // Forward move should allow 2 squares
        let forward = moves.iter().find(|m| m.file_diff == 0).unwrap();
        assert_eq!(forward.rank_diff, -1);
        assert_eq!(forward.dist, 2);

        // Captures
        let left = moves.iter().find(|m| m.file_diff == -1).unwrap();
        let right = moves.iter().find(|m| m.file_diff == 1).unwrap();
        assert_eq!(left.rank_diff, -1);
        assert_eq!(right.rank_diff, -1);
        assert!(left.capture_allowed && left.capture_forced);
        assert!(right.capture_allowed && right.capture_forced);
    }

    #[test]
    fn test_pawn_moves_white_non_initial() {
        let pawn = Pawn::new(Colour::White);
        let coords = Coords::new(3, File::E);
        let moves = pawn.get_move_rays(coords);

        let forward = moves.iter().find(|m| m.file_diff == 0).unwrap();
        assert_eq!(forward.rank_diff, 1);
        assert_eq!(forward.dist, 1);
    }
}