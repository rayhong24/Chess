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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]
    fn test_bishop_representation() {
        let white_bishop = Bishop::new(Colour::White);
        let black_bishop = Bishop::new(Colour::Black);
        assert_eq!(white_bishop.get_representation(), 'B');
        assert_eq!(black_bishop.get_representation(), 'b');
    }

    #[test]
    fn test_bishop_moves_initial() {
        let bishop = Bishop::new(Colour::White);
        let coords = Coords::new(1, File::C);
        let moves = bishop.get_move_rays(coords);

        // Bishop should have 4 move rays (diagonal directions)
        assert_eq!(moves.len(), 4);

        // Check each diagonal direction
        let up_left = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == -1).unwrap();
        let up_right = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == 1).unwrap();
        let down_left = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == -1).unwrap();
        let down_right = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == 1).unwrap();

        assert_eq!(up_left.dist, 7);
        assert_eq!(up_right.dist, 7);
        assert_eq!(down_left.dist, 7);
        assert_eq!(down_right.dist, 7);

        assert!(up_left.capture_allowed && !up_left.capture_forced);
        assert!(up_right.capture_allowed && !up_right.capture_forced);
        assert!(down_left.capture_allowed && !down_left.capture_forced);
        assert!(down_right.capture_allowed && !down_right.capture_forced);
    }
}