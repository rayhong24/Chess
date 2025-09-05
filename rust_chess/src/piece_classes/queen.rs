use crate::enums::Colour;
use crate::coords::Coords;
use crate::moves::move_ray::MoveRay;
use crate::piece_classes::Piece;

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

    fn get_move_rays(&self, _from: Coords) -> Vec<MoveRay> {
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]
    fn test_queen_representation() {
        let white_queen = Queen::new(Colour::White);
        let black_queen = Queen::new(Colour::Black);
        assert_eq!(white_queen.get_representation(), 'Q');
        assert_eq!(black_queen.get_representation(), 'q');
    }

    #[test]
    fn test_queen_moves_initial() {
        let queen = Queen::new(Colour::White);
        let coords = Coords::new(1, File::D);
        let moves = queen.get_move_rays(coords);

        // Queen should have 8 move rays (all directions)
        assert_eq!(moves.len(), 8);

        // Check each direction
        let up_left = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == -1).unwrap();
        let up = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == 0).unwrap();
        let up_right = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == 1).unwrap();
        let left = moves.iter().find(|m| m.rank_diff == 0 && m.file_diff == -1).unwrap();
        let right = moves.iter().find(|m| m.rank_diff == 0 && m.file_diff == 1).unwrap();
        let down_left = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == -1).unwrap();
        let down = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == 0).unwrap();
        let down_right = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == 1).unwrap();

        assert_eq!(up_left.dist, 7);
        assert_eq!(up.dist, 7);
        assert_eq!(up_right.dist, 7);
        assert_eq!(left.dist, 7);
        assert_eq!(right.dist, 7);
        assert_eq!(down_left.dist, 7);
        assert_eq!(down.dist, 7);
        assert_eq!(down_right.dist, 7);

        assert!(up_left.capture_allowed && !up_left.capture_forced);
        assert!(up.capture_allowed && !up.capture_forced);
        assert!(up_right.capture_allowed && !up_right.capture_forced);
        assert!(left.capture_allowed && !left.capture_forced);
        assert!(right.capture_allowed && !right.capture_forced);
        assert!(down_left.capture_allowed && !down_left.capture_forced);
        assert!(down.capture_allowed && !down.capture_forced);
        assert!(down_right.capture_allowed && !down_right.capture_forced);
    }
}