use crate::enums::{Colour, ChessMove, PieceType, File};
use crate::coords::Coords;
use crate::game_classes::game::Game;

#[derive(Debug, Clone)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
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
            castling_rights: CastlingRights {
                white_kingside: true,
                white_queenside: true,
                black_kingside: true,
                black_queenside: true,
            },
            en_passant_target: None,
            en_passant_piece_coords: None
        }
    }

    pub fn set_castling_rights_from_fenstr(&mut self, castling_rights_fenstr: &str) {
        self.castling_rights.white_kingside = false;
        self.castling_rights.white_queenside = false;
        self.castling_rights.black_kingside = false;
        self.castling_rights.black_queenside = false;

        if castling_rights_fenstr == "-" {
            return;
        }

        for ch in castling_rights_fenstr.chars() {
            match ch {
                'K' => self.castling_rights.white_kingside = true,
                'Q' => self.castling_rights.white_queenside = true,
                'k' => self.castling_rights.black_kingside = true,
                'q' => self.castling_rights.black_queenside = true,
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

    pub fn can_castle_white_kingside(&self) -> bool {
        self.castling_rights.white_kingside
    }
    pub fn can_castle_white_queenside(&self) -> bool {
        self.castling_rights.white_queenside
    }
    pub fn can_castle_black_kingside(&self) -> bool {
        self.castling_rights.black_kingside
    }
    pub fn can_castle_black_queenside(&self) -> bool {
        self.castling_rights.black_queenside
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

    pub fn update(&mut self, mv: &ChessMove) {
        if self.turn != mv.colour() {
            panic!(
                "It's {:?}'s turn, but got move for {:?}",
                self.turn,
                mv.colour()
            );
        }

        self.turn = self.turn.other();

        // Update castling rights
        if mv.piece() == PieceType::King {
            if mv.colour() == Colour::White {
                self.castling_rights.white_kingside = false;
                self.castling_rights.white_queenside = false;
            } else {
                self.castling_rights.black_kingside = false;
                self.castling_rights.black_queenside = false;
            }
        }
        if mv.piece() == PieceType::Rook {
            if mv.colour() == Colour::White {
                if mv.from() == Coords::new(1, File::A) {
                    self.castling_rights.white_queenside = false;
                } else if mv.from() == Coords::new(1, File::H) {
                    self.castling_rights.white_kingside = false;
                }
            } else {
                if mv.from() == Coords::new(8, File::A) {
                    self.castling_rights.black_queenside = false;
                } else if mv.from() == Coords::new(8, File::H) {
                    self.castling_rights.black_kingside = false;
                }
            }
        }
        // Update en passant target
        if mv.piece() == PieceType::Pawn && (mv.from().rank).abs_diff(mv.to().rank) == 2 {
            let ep_rank = (mv.from().rank + mv.to().rank) / 2;
            self.en_passant_target = Some(Coords::new(ep_rank, mv.from().file));
            self.en_passant_piece_coords = Some(mv.to());
        } else {
            self.en_passant_target = None;
            self.en_passant_piece_coords = None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::moves::NormalMove;
    use crate::enums::{ChessMove, Colour, File, PieceType};
    use crate::coords::Coords;

    fn make_move(piece: PieceType, colour: Colour, from: Coords, to: Coords) -> ChessMove {
        ChessMove::Normal(NormalMove {
            colour: colour,
            piece_type: piece,
            from: from,
            to: to,
        })
    }

    #[test]
    fn test_new_initial_state() {
        let gs = GameState::new();
        assert_eq!(gs.turn, Colour::White);
        assert!(gs.castling_rights.white_kingside);
        assert!(gs.castling_rights.white_queenside);
        assert!(gs.castling_rights.black_kingside);
        assert!(gs.castling_rights.black_queenside);
        assert!(gs.en_passant_target.is_none());
    }

    #[test]
    fn test_turn_alternates() {
        let mut gs = GameState::new();
        let mv = make_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv);
        assert_eq!(gs.turn, Colour::Black);
    }

    #[test]
    fn test_castling_rights_removed_after_king_move() {
        let mut gs = GameState::new();
        let mv = make_move(PieceType::King, Colour::White, Coords::new(1, File::E), Coords::new(2, File::E));
        gs.update(&mv);
        assert!(!gs.castling_rights.white_kingside);
        assert!(!gs.castling_rights.white_queenside);
    }

    #[test]
    fn test_castling_rights_removed_after_rook_move() {
        let mut gs = GameState::new();

        // White rook A1 → A2 should disable queenside castling
        let mv = make_move(PieceType::Rook, Colour::White, Coords::new(1, File::A), Coords::new(2, File::A));
        gs.update(&mv);
        assert!(!gs.castling_rights.white_queenside);
        assert!(gs.castling_rights.white_kingside);

        // Reset game state
        let mut gs = GameState::new();
        // Black rook H8 → H7 should disable kingside castling
        gs.turn = Colour::Black;
        let mv = make_move(PieceType::Rook, Colour::Black, Coords::new(8, File::H), Coords::new(7, File::H));
        gs.update(&mv);
        assert!(!gs.castling_rights.black_kingside);
        assert!(gs.castling_rights.black_queenside);
    }

    #[test]
    fn test_en_passant_set_for_double_pawn_push() {
        let mut gs = GameState::new();
        let mv = make_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv);
        assert_eq!(gs.en_passant_target, Some(Coords::new(3, File::E)));
    }

    #[test]
    fn test_en_passant_cleared_for_non_double_pawn_push() {
        let mut gs = GameState::new();
        let mv1 = make_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
        gs.update(&mv1);
        let mv2 = make_move(PieceType::Pawn, Colour::Black, Coords::new(7, File::A), Coords::new(6, File::A));
        gs.update(&mv2);
        assert!(gs.en_passant_target.is_none());
    }

    #[test]
    #[should_panic(expected = "It's White's turn")]
    fn test_wrong_turn_panics() {
        let mut gs = GameState::new();
        let mv = make_move(PieceType::Pawn, Colour::Black, Coords::new(7, File::E), Coords::new(5, File::E));
        gs.update(&mv); // should panic
    }
}
