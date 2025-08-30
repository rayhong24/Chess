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

    fn get_move_rays(&self, _coords: Coords) -> Vec<MoveRay> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]
    fn test_rook_representation() {
        let white_rook = Rook::new(Colour::White);
        let black_rook = Rook::new(Colour::Black);
        assert_eq!(white_rook.get_representation(), 'R');
        assert_eq!(black_rook.get_representation(), 'r');
    }

    #[test]
    fn test_rook_moves_initial() {
        let rook = Rook::new(Colour::White);
        let coords = Coords::new(1, File::A);
        let moves = rook.get_move_rays(coords);

        // Rook should have 4 move rays (up, down, left, right)
        assert_eq!(moves.len(), 4);

        // Check each direction
        let up = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == 0).unwrap();
        let down = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == 0).unwrap();
        let left = moves.iter().find(|m| m.rank_diff == 0 && m.file_diff == -1).unwrap();
        let right = moves.iter().find(|m| m.rank_diff == 0 && m.file_diff == 1).unwrap();

        assert_eq!(up.dist, 7);
        assert_eq!(down.dist, 7);
        assert_eq!(left.dist, 7);
        assert_eq!(right.dist, 7);

        assert!(up.capture_allowed && !up.capture_forced);
        assert!(down.capture_allowed && !down.capture_forced);
        assert!(left.capture_allowed && !left.capture_forced);
        assert!(right.capture_allowed && !right.capture_forced);
    }
}
