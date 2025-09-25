use std::fmt::Binary;

use strum::IntoEnumIterator;

use crate::engine::piece_square_tables::{
    MG_PAWN_PST, EG_PAWN_PST,
    MG_KNIGHT_PST, EG_KNIGHT_PST,
    MG_BISHOP_PST, EG_BISHOP_PST,
    MG_ROOK_PST, EG_ROOK_PST,
    MG_QUEEN_PST, EG_QUEEN_PST,
    MG_KING_PST, EG_KING_PST,
};
use crate::enums::{Colour, PieceType};
use crate::coords::Coords;
use crate::piece::Piece;
use crate::game_classes::game::Game;
use crate::game_classes::game::GameResult;
use crate::engine::minimax::INF;

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 20000;
const vec

pub const PAWN_PHASE: i32 = 0;
pub const KNIGHT_PHASE: i32 = 1;
pub const BISHOP_PHASE: i32 = 1;
pub const ROOK_PHASE: i32 = 2;
pub const QUEEN_PHASE: i32 = 4;
pub const TOTAL_PHASE: i32 = PAWN_PHASE * 16 + KNIGHT_PHASE * 4
    + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2; // = 24


pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(game: &mut Game) -> i32 {
        let colour = game.get_game_state().get_turn();

        match game.is_game_over() {
            Some(GameResult::Checkmate(loser)) => {
                if loser == colour {
                    -INF
                }
                else {
                    INF
                }
            }
            Some(GameResult::Stalemate) => 0,
            None => Self::evaluate_tapered(game)
        }
    }

    fn evaluate_tapered(game: &mut Game) -> i32 {
        let current_player = game.get_game_state().get_turn();

        let mut mg_score = 0;
        let mut eg_score = 0;
        let mut phase = 0;

        for (piece, coords) in game.get_board().get_all_pieces() {
            let (mg_val, eg_val, piece_phase) = Self::evaluate_piece_tapered(piece, coords);

            if piece.colour == current_player {
                mg_score += mg_val;
                eg_score += eg_val;
            } else {
                mg_score -= mg_val;
                eg_score -= eg_val;
            }

            phase += piece_phase;
        }

        let mg_weight = phase;
        let eg_weight = TOTAL_PHASE - phase;

        (mg_score * mg_weight + eg_score * eg_weight) / TOTAL_PHASE
    }

    fn evaluate_piece_tapered(piece: Piece, coords: Coords) -> (i32, i32, i32) {
        match piece.kind {
            PieceType::Pawn => (
                PAWN_VALUE + Self::pst_value(&MG_PAWN_PST, coords, piece.colour),
                PAWN_VALUE + Self::pst_value(&EG_PAWN_PST, coords, piece.colour),
                PAWN_PHASE,
            ),
            PieceType::Knight => (
                KNIGHT_VALUE + Self::pst_value(&MG_KNIGHT_PST, coords, piece.colour),
                KNIGHT_VALUE + Self::pst_value(&EG_KNIGHT_PST, coords, piece.colour),
                KNIGHT_PHASE,
            ),
            PieceType::Bishop => (
                BISHOP_VALUE + Self::pst_value(&MG_BISHOP_PST, coords, piece.colour),
                BISHOP_VALUE + Self::pst_value(&EG_BISHOP_PST, coords, piece.colour),
                BISHOP_PHASE,
            ),
            PieceType::Rook => (
                ROOK_VALUE + Self::pst_value(&MG_ROOK_PST, coords, piece.colour),
                ROOK_VALUE + Self::pst_value(&EG_ROOK_PST, coords, piece.colour),
                ROOK_PHASE,
            ),
            PieceType::Queen => (
                QUEEN_VALUE + Self::pst_value(&MG_QUEEN_PST, coords, piece.colour),
                QUEEN_VALUE + Self::pst_value(&EG_QUEEN_PST, coords, piece.colour),
                QUEEN_PHASE,
            ),
            PieceType::King => (
                KING_VALUE + Self::pst_value(&MG_KING_PST, coords, piece.colour),
                KING_VALUE + Self::pst_value(&EG_KING_PST, coords, piece.colour),
                0, // kings don’t contribute to phase
            ),
        }
    }

    pub fn get_piece_value(piece_type: PieceType) -> i32 {
        match piece_type {
            PieceType::Pawn => PAWN_VALUE,
            PieceType::Knight => KNIGHT_VALUE,
            PieceType::Bishop => BISHOP_VALUE,
            PieceType::Rook => ROOK_VALUE,
            PieceType::Queen => QUEEN_VALUE,
            PieceType::King => KING_VALUE,
        }
    }

    fn pst_value(pst: &[[i32; 8]; 8], coords: Coords, colour: Colour) -> i32 {
        match colour {
            Colour::White => pst[8 - coords.rank as usize][coords.file as usize], // rank 0 is White’s back rank
            Colour::Black => pst[coords.rank as usize - 1][coords.file as usize],     // rank 7 is Black’s back rank
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    fn eval_fen(fen: &str) -> i32 {
        let mut game = Game::new();
        if fen != "startpos" {
            game.set_fenstr(fen);
        }

        Evaluator::evaluate(&mut game)
    }

    #[test]
    fn starting_position_is_near_equal() {
        let score = eval_fen("startpos");
        assert!(score.abs() < 50, "Score too imbalanced: {}", score);
    }

    #[test]
    fn material_advantage_is_reflected() {
        // White has an extra queen
        let score = eval_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(score > 800, "Expected strong advantage for White, got {}", score);
    }

    #[test]
    fn piece_square_tables_affect_score() {
        // Knight in the center vs knight on the rim
        let center_score = eval_fen("K7/8/8/3N4/8/8/8/4k3 w - - 0 1");
        let rim_score = eval_fen("K7/8/8/N7/8/8/8/4k3 w - - 0 1");
        assert!(
            center_score > rim_score,
            "Knight in center should score higher ({} vs {})",
            center_score,
            rim_score
        );
    }

    #[test]
    fn phase_transitions_to_endgame() {
        // Lone kings should evaluate to near 0
        let score = eval_fen("8/8/8/8/8/8/4K3/4k3 w - - 0 1");
        assert!(score.abs() < 50, "Expected drawish eval in king vs king: {}", score);
    }
}
