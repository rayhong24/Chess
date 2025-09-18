use crate::game_classes::game::{Game, GameResult};
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour};
use crate::move_ordering::order_moves;

const INF: i32 = 30_000;

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
        if depth == 0 || game.is_game_over().is_some() {
            return self.quiescence(game, -INF, INF);
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

    pub fn evaluate_material(&self, game: &Game) -> i32 {
        let colour = game.get_game_state().get_turn();
        game.get_board().get_material(colour) - game.get_board().get_material(colour.other())
    }


    pub fn evaluate(&self, game: &mut Game) -> i32 {
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
            None => self.evaluate_material(game)
        }

    }


    fn quiescence(&self, game: &mut Game, mut alpha: i32, beta: i32) -> i32 {
        // Step 1: stand pat evaluation
        let stand_pat = self.evaluate(game);

        if stand_pat >= beta {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        // Step 2: generate only "tactical" moves (captures/promotions/checks)
        let mut moves = MoveGenerator::generate_legal_moves(game, game.get_game_state().get_turn());
        order_moves(&mut moves, game);


        for mv in &moves {
            if !matches!(mv, ChessMove::Promotion(_)) && !game.is_capture(mv) && !game.is_check(mv) {
                continue;
            }

            game.make_move(&mv);
            let score = -self.quiescence(game, -beta, -alpha);
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