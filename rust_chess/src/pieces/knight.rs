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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]fn test_knight_representation() {
        let white_knight = Knight::new(Colour::White);
        let black_knight = Knight::new(Colour::Black);  
        assert_eq!(white_knight.get_representation(), 'N');
        assert_eq!(black_knight.get_representation(), 'n');
    }

    #[test]
    fn test_knight_moves_initial() {
        let knight = Knight::new(Colour::White);
        let coords = Coords::new(1, File::B);
        let moves = knight.get_move_rays(coords);

        // Knight should have 8 move rays (L-shaped moves)
        assert_eq!(moves.len(), 8);

        // Check each L-shaped move
        let move_positions = [
            (2, 1), (2, -1),
            (1, 2), (1, -2),
            (-1, 2), (-1, -2),
            (-2, 1), (-2, -1)
        ];

        for (rank_diff, file_diff) in &move_positions {
            let mv = moves.iter().find(|m| m.rank_diff == *rank_diff && m.file_diff == *file_diff).unwrap();
            assert_eq!(mv.dist, 1);
            assert!(mv.capture_allowed && !mv.capture_forced);
        }
    }
}