use crate::enums::{ChessMove, Colour, File, PieceType};
use crate::coords::Coords;
use crate::game_classes::zobrist::Zobrist;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE  = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_KINGSIDE  = 0b0100;
        const BLACK_QUEENSIDE = 0b1000;
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    turn: Colour,
    castling_rights: CastlingRights,
    en_passant_target: Option<Coords>,
    en_passant_piece_coords: Option<Coords>
}

impl GameState {
    pub fn new() -> Self {
        Self {
            turn: Colour::White,
            castling_rights: CastlingRights::all(),
            en_passant_target: None,
            en_passant_piece_coords: None
        }
    }

    pub fn set_castling_rights_from_fenstr(&mut self, castling_rights_fenstr: &str) {
        self.castling_rights = CastlingRights::empty();

        if castling_rights_fenstr == "-" {
            return;
        }

        for ch in castling_rights_fenstr.chars() {
            match ch {
                'K' => self.castling_rights.insert(CastlingRights::WHITE_KINGSIDE),
                'Q' => self.castling_rights.insert(CastlingRights::WHITE_QUEENSIDE),
                'k' => self.castling_rights.insert(CastlingRights::BLACK_KINGSIDE),
                'q' => self.castling_rights.insert(CastlingRights::BLACK_QUEENSIDE),
                _ => panic!("Invalid castling rights character in FEN: {}", ch),
            }
        }
    }

    pub fn get_turn(&self) -> Colour {
        self.turn
    }

    pub fn set_turn(&mut self, colour: Colour) {
        self.turn = colour;
    }

    pub fn get_castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub fn can_castle(&self, right: CastlingRights) -> bool {
        self.castling_rights.contains(right)
    }

    pub fn get_en_passant_target(&self) -> Option<Coords> {
        self.en_passant_target
    }

    pub fn get_en_passant_piece_coords(&self) -> Option<Coords> {
        self.en_passant_piece_coords
    }

    pub fn set_en_passant_target(&mut self, target: Option<Coords>) {
        self.en_passant_target = target;
    }

    pub fn update(&mut self, mv: &ChessMove, hash: &mut u64, zobrist: &Zobrist) {
        if self.turn != mv.colour() {
            panic!(
                "It's {:?}'s turn, but got move for {:?}",
                self.turn,
                mv.colour()
            );
        }

        self.turn = self.turn.other();
        zobrist.toggle_side_to_move(hash);

        // Update castling rights
        let old_castling = self.castling_rights;

        if mv.piece() == PieceType::King {
            if mv.colour() == Colour::White {
                self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE);
            } else {
                self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE);
            }
        }
        if mv.piece() == PieceType::Rook {
            if mv.colour() == Colour::White {
                if mv.from() == Coords::new(1, File::A) {
                    self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE);
                } else if mv.from() == Coords::new(1, File::H) {
                    self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE);
                }
            } else {
                if mv.from() == Coords::new(8, File::A) {
                    self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE);
                } else if mv.from() == Coords::new(8, File::H) {
                    self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE);
                }
            }
        }

        // Toggle old castling rights out of hash
        zobrist.toggle_castle(hash, &old_castling);
        // Toggle new castling rights into hash
        zobrist.toggle_castle(hash, &self.castling_rights);

        // Update en passant target
        let old_ep = self.en_passant_target;
        if mv.piece() == PieceType::Pawn && (mv.from().rank).abs_diff(mv.to().rank) == 2 {
            let ep_rank = (mv.from().rank + mv.to().rank) / 2;
            self.en_passant_target = Some(Coords::new(ep_rank, mv.from().file));
            self.en_passant_piece_coords = Some(mv.to());
        } else {
            self.en_passant_target = None;
            self.en_passant_piece_coords = None
        }
        if let Some(file) = old_ep.map(|c| c.file) {
            zobrist.toggle_en_passant(hash, &file);
        }
        if let Some(file) = self.en_passant_target.map(|c| c.file) {
            zobrist.toggle_en_passant(hash, &file);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::moves::NormalMove;
    use crate::enums::{ChessMove, Colour, File, PieceType};
    use crate::coords::Coords;
    use crate::game_classes::zobrist::Zobrist;

    fn create_move(piece: PieceType, colour: Colour, from: Coords, to: Coords) -> ChessMove {
        ChessMove::Normal(NormalMove {
            colour,
            piece_type: piece,
            from,
            to,
        })
    }

    fn new_game_state_with_hash() -> (GameState, Zobrist, u64) {
        let gs = GameState::new();
        let zobrist = Zobrist::new();
        let hash = 0u64; // start with empty hash
        (gs, zobrist, hash)
    }

    #[test]
    fn test_new_initial_state() {
        let (gs, _, _) = new_game_state_with_hash();
        assert_eq!(gs.turn, Colour::White);
        assert!(gs.can_castle(CastlingRights::WHITE_KINGSIDE));
        assert!(gs.can_castle(CastlingRights::WHITE_QUEENSIDE));
        assert!(gs.can_castle(CastlingRights::BLACK_KINGSIDE));
        assert!(gs.can_castle(CastlingRights::BLACK_QUEENSIDE));
        assert!(gs.en_passant_target.is_none());
    }

    #[test]
    fn test_turn_alternates() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let mv = create_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv, &mut hash, &zobrist);
        assert_eq!(gs.turn, Colour::Black);
    }

    #[test]
    fn test_castling_rights_removed_after_king_move() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let mv = create_move(PieceType::King, Colour::White, Coords::new(1, File::E), Coords::new(2, File::E));
        gs.update(&mv, &mut hash, &zobrist);
        assert!(!gs.can_castle(CastlingRights::WHITE_KINGSIDE));
        assert!(!gs.can_castle(CastlingRights::WHITE_QUEENSIDE));
    }

    #[test]
    fn test_castling_rights_removed_after_rook_move() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        // White rook A1 → A2 should disable queenside castling
        let mv = create_move(PieceType::Rook, Colour::White, Coords::new(1, File::A), Coords::new(2, File::A));
        gs.update(&mv, &mut hash, &zobrist);
        assert!(gs.can_castle(CastlingRights::WHITE_KINGSIDE));
        assert!(!gs.can_castle(CastlingRights::WHITE_QUEENSIDE));

        // Reset game state
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        gs.turn = Colour::Black;
        // Black rook H8 → H7 should disable kingside castling
        let mv = create_move(PieceType::Rook, Colour::Black, Coords::new(8, File::H), Coords::new(7, File::H));
        gs.update(&mv, &mut hash, &zobrist);
        assert!(!gs.can_castle(CastlingRights::BLACK_KINGSIDE));
        assert!(gs.can_castle(CastlingRights::BLACK_QUEENSIDE));
    }

    #[test]
    fn test_en_passant_set_for_double_pawn_push() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let mv = create_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv, &mut hash, &zobrist);
        assert_eq!(gs.en_passant_target, Some(Coords::new(3, File::E)));
    }

    #[test]
    fn test_en_passant_cleared_for_non_double_pawn_push() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let mv1 = create_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv1, &mut hash, &zobrist);
        let mv2 = create_move(PieceType::Pawn, Colour::Black, Coords::new(7, File::A), Coords::new(6, File::A));
        gs.update(&mv2, &mut hash, &zobrist);
        assert!(gs.en_passant_target.is_none());
    }

    #[test]
    #[should_panic(expected = "It's White's turn")]
    fn test_wrong_turn_panics() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let mv = create_move(PieceType::Pawn, Colour::Black, Coords::new(7, File::E), Coords::new(5, File::E));
        gs.update(&mv, &mut hash, &zobrist); // should panic
    }

    #[test]
    fn test_piece_move_changes_hash() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let initial_hash = hash;

        let mv = create_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv, &mut hash, &zobrist);

        // hash should change after a move
        assert_ne!(hash, initial_hash);
    }

    #[test]
    fn test_castling_rights_change_hash() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let initial_hash = hash;

        let mv = create_move(PieceType::King, Colour::White, Coords::new(1, File::E), Coords::new(2, File::E));
        gs.update(&mv, &mut hash, &zobrist);

        // Removing castling rights should modify hash
        assert_ne!(hash, initial_hash);
    }

    #[test]
    fn test_en_passant_change_hash() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let initial_hash = hash;

        let mv = create_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv, &mut hash, &zobrist);

        // Double pawn push sets en passant target -> hash changes
        assert_ne!(hash, initial_hash);
    }

    #[test]
    fn test_side_to_move_change_hash() {
        let (mut gs, zobrist, mut hash) = new_game_state_with_hash();
        let initial_hash = hash;

        let mv = create_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(3, File::E));
        gs.update(&mv, &mut hash, &zobrist);

        // Side-to-move toggled -> hash changes
        assert_ne!(hash, initial_hash);
    }
}