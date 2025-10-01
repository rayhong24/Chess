use crate::game_classes::game::{Game, GameResult};
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour, ExecutedMove};
use crate::move_ordering::order_moves;
use crate::engine::evaluator::Evaluator;

pub const INF: i32 = 30_000;

pub struct Minimax {
    pub max_depth: usize,
    pub quiescence_max_depth: usize,
    pub selective_quiescence: bool
}

impl Minimax {
    pub fn new(max_depth: usize, quiescence_max_depth: usize, selective_quiescence: bool) -> Self {
        Self { max_depth, quiescence_max_depth, selective_quiescence}
    }

    pub fn find_best_move(&self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        let mut best_score = i32::MIN;
        let mut best_move: Option<ChessMove> = None;
        
        let mut move_scores = vec![];

        let mut moves = MoveGenerator::generate_legal_moves(game, colour);
        order_moves(&mut moves, game);
        for mv in moves {
            game.make_move(&mv);

            let score = -self.minimax(game, self.max_depth - 1, -INF, INF, colour.other());

            game.undo_last_move();

            move_scores.push((mv, score));

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        // move_scores.sort_by(|a, b| b.1.cmp(&a.1));

        // for (mv, score) in &move_scores {
        //     println!("{}: {}", mv, score);
        // }

        best_move
    }

    fn minimax(&self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32, colour: Colour) -> i32 {
        let moves = MoveGenerator::generate_legal_moves(game, colour);

        if let Some(result) = game.is_game_over_with_moves(&moves) {
            return Evaluator::evaluate_game_result(game, Some(result), depth, colour);
        }

        if depth == 0 {
            return if self.selective_quiescence {
                self.quiescence(game, alpha, beta, self.quiescence_max_depth)
            } else {
                Evaluator::evaluate_game_result(game, None, depth, colour)
            };
        }
        

        let mut best_score = i32::MIN;

        for mv in &moves {
            game.make_move(&mv);
            let score = -self.minimax(game, depth - 1, -beta, -alpha, colour.other());
            game.undo_last_move();

            best_score = best_score.max(score);

            if best_score >= beta {
                return best_score;
            }
        }
        best_score
    }



    fn quiescence(&self, game: &mut Game, mut alpha: i32, beta: i32, max_depth: usize) -> i32 {
        // Step 0: terminal positions
        let to_move = game.get_game_state().get_turn();
        let moves = MoveGenerator::generate_legal_moves(game, to_move);

        if let Some(result) = game.is_game_over_with_moves(&moves) {
            return Evaluator::evaluate_game_result(game, Some(result), self.max_depth, to_move);
        }

        // Step 1: stand pat evaluation
        let stand_pat = Evaluator::evaluate_game_result(game, None, self.max_depth, to_move);

        if max_depth == 0 || stand_pat >= beta {
            return stand_pat;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        // Step 2: generate only tactical moves (captures + promotions)
        let mut moves = MoveGenerator::generate_legal_moves(game, game.get_game_state().get_turn());
        order_moves(&mut moves, game);

        for mv in &moves {
            if !matches!(mv, ChessMove::Promotion(_)) && !game.is_capture(mv) {
                continue;
            }

            game.make_move(mv);
            let score = -self.quiescence(game, -beta, -alpha, max_depth - 1);
            game.undo_last_move();

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}