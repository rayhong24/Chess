use rand::Rng;

use crate::{coords::Coords, enums::File, piece::Piece};
use crate::game_classes::game_state::CastlingRights;

pub struct Zobrist {
    pub piece_square: [[[u64; 64]; 6]; 2], // [color][piece][square]
    pub castling: [u64; 4],                // 4 castling rights 
    pub en_passant: [u64; 8],              // en passant files
    pub side_to_move: u64,                 // white or black to move
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let mut piece_square = [[[0u64; 64]; 6]; 2];
        for color in 0..2 {
            for piece in 0..6 {
                for sq in 0..64 {
                    piece_square[color][piece][sq] = rng.random();
                }
            }
        }

        let mut castling = [0u64; 4];
        for i in 0..4 {
            castling[i] = rng.random();
        }

        let mut en_passant = [0u64; 8];
        for i in 0..8 {
            en_passant[i] = rng.random();
        }

        let side_to_move = rng.random();

        Zobrist {
            piece_square,
            castling,
            en_passant,
            side_to_move,
        }
    }

    pub fn toggle_piece(&self, hash: &mut u64, coords: &Coords, piece: &Piece) {
        let colour_idx = piece.colour as usize;
        let kind_idx = piece.kind as usize;
        let coords_idx = coords.to_index();
        *hash ^= self.piece_square[colour_idx][kind_idx][coords_idx];
    }

    // Toggle a specific castling right
    pub fn toggle_castle(&self, hash: &mut u64, right: &CastlingRights) {
        // Map bitflags to indices in the array
        if right.contains(CastlingRights::WHITE_KINGSIDE) {
            *hash ^= self.castling[0];
        }
        if right.contains(CastlingRights::WHITE_QUEENSIDE) {
            *hash ^= self.castling[1];
        }
        if right.contains(CastlingRights::BLACK_KINGSIDE) {
            *hash ^= self.castling[2];
        }
        if right.contains(CastlingRights::BLACK_QUEENSIDE) {
            *hash ^= self.castling[3];
        }
    }

    pub fn toggle_en_passant(&self, hash: &mut u64, file: &File) {
        *hash ^= self.en_passant[*file as usize];
    }

    // Toggle side to move
    pub fn toggle_side_to_move(&self, hash: &mut u64) {
        *hash ^= self.side_to_move;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::Coords;
    use crate::piece::{Piece};
    use crate::enums::{Colour, PieceType};

    #[test]
    fn test_toggle_piece() {
        let zob = Zobrist::new();
        let mut hash: u64 = 0;

        let piece = Piece { colour: Colour::White, kind: PieceType::Pawn };
        let coords = Coords::new(2, crate::enums::File::E);

        let original_hash = hash;
        zob.toggle_piece(&mut hash, &coords, &piece);
        assert_ne!(hash, original_hash, "Hash should change after toggling piece");

        // Toggling again should revert
        zob.toggle_piece(&mut hash, &coords, &piece);
        assert_eq!(hash, original_hash, "Hash should revert after toggling piece twice");
    }

    #[test]
    fn test_toggle_castle() {
        let zob = Zobrist::new();
        let mut hash: u64 = 0;

        let rights = CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_QUEENSIDE;
        let original_hash = hash;
        zob.toggle_castle(&mut hash, &rights);
        assert_ne!(hash, original_hash, "Hash should change after toggling castling rights");

        // Toggling again should revert
        zob.toggle_castle(&mut hash, &rights);
        assert_eq!(hash, original_hash, "Hash should revert after toggling castling rights twice");
    }

    #[test]
    fn test_toggle_en_passant() {
        let zob = Zobrist::new();
        let mut hash: u64 = 0;

        let file = File::E; // E file
        let original_hash = hash;
        zob.toggle_en_passant(&mut hash, &file);
        assert_ne!(hash, original_hash, "Hash should change after toggling en passant");

        // Toggling again should revert
        zob.toggle_en_passant(&mut hash, &file);
        assert_eq!(hash, original_hash, "Hash should revert after toggling en passant twice");
    }

    #[test]
    fn test_toggle_side_to_move() {
        let zob = Zobrist::new();
        let mut hash: u64 = 0;

        let original_hash = hash;
        zob.toggle_side_to_move(&mut hash);
        assert_ne!(hash, original_hash, "Hash should change after toggling side to move");

        // Toggling again should revert
        zob.toggle_side_to_move(&mut hash);
        assert_eq!(hash, original_hash, "Hash should revert after toggling side to move twice");
    }

    #[test]
    fn test_combined_toggles() {
        let zob = Zobrist::new();
        let mut hash: u64 = 0;

        let piece = Piece { colour: Colour::Black, kind: PieceType::Knight };
        let coords = Coords::new(8, crate::enums::File::G);
        let rights = CastlingRights::WHITE_QUEENSIDE | CastlingRights::BLACK_KINGSIDE;
        let file = File::D; // D file

        let original_hash = hash;

        zob.toggle_piece(&mut hash, &coords, &piece);
        zob.toggle_castle(&mut hash, &rights);
        zob.toggle_en_passant(&mut hash, &file);
        zob.toggle_side_to_move(&mut hash);

        assert_ne!(hash, original_hash, "Hash should change after multiple toggles");

        // Toggle everything again should revert
        zob.toggle_side_to_move(&mut hash);
        zob.toggle_en_passant(&mut hash, &file);
        zob.toggle_castle(&mut hash, &rights);
        zob.toggle_piece(&mut hash, &coords, &piece);

        assert_eq!(hash, original_hash, "Hash should revert after toggling all features twice");
    }
}
