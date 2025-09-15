use crate::game_classes::game::Game;
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour};

pub struct Minimax {
    pub max_depth: usize,
}

impl Minimax {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }

    pub fn find_best_move(&self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        let mut best_score = i32::MIN;
        let mut best_move: Option<ChessMove> = None;
        
        let mut move_scores = vec![];

        let moves = MoveGenerator::generate_legal_moves(game, colour);
        for mv in moves {
            game.make_move(&mv);

            let score = -self.minimax(game, self.max_depth - 1, -i32::MAX, i32::MAX, colour.other());

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
        if depth == 0 || game.is_game_over() {
            return self.quiescence(game, -i32::MAX, i32::MAX, colour);
        }

        let moves = MoveGenerator::generate_legal_moves(game, colour);
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

    pub fn evaluate(&self, game: &Game, colour: Colour) -> i32 {
        game.get_board().get_material(colour) - game.get_board().get_material(colour.other())
    }

    fn quiescence(&self, game: &mut Game, mut alpha: i32, beta: i32, colour: Colour) -> i32 {
        // Step 1: stand pat evaluation
        let stand_pat = self.evaluate(game, colour);

        if stand_pat >= beta {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        // Step 2: generate only "tactical" moves (captures/promotions/checks)
        let moves = MoveGenerator::generate_legal_moves(game, colour);
        for mv in &moves {
            if !matches!(mv, ChessMove::Promotion(_)) && !game.is_capture(mv) && !game.is_check(mv) {
                continue;
            }

            game.make_move(&mv);
            let score = -self.quiescence(game, -beta, -alpha, colour.other());
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