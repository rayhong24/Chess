use crate::enums::{ChessMove, PieceType};
use crate::game_classes::game::Game;
use crate::engine::evaluator::Evaluator;

fn mvv_lva_score(attacker: PieceType, victim: PieceType) -> i32 {
    Evaluator::get_piece_value(victim) - Evaluator::get_piece_value(attacker)
}


fn move_order_score(mv: &ChessMove, game: &Game) -> i32 {
    // Captures: use MVV-LVA
    if let Some(captured) = game.get_board().get_coords(&mv.to()) {
        let attacker = game.get_board().get_coords(&mv.from()).unwrap();
        return 100_000 + mvv_lva_score(attacker.kind, captured.kind);
    }

    // Promotions (if included in quiescence)
    if matches!(mv, ChessMove::Promotion(_)) {
        return 50_000;
    }

    // Quiet moves (not usually searched in quiescence)
    0
}

pub fn order_moves(moves: &mut Vec<ChessMove>, game: &Game) {
    moves.sort_by_key(|mv| move_order_score(mv, game));
    moves.reverse(); // highest score first
}