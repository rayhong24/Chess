use std::collections::HashMap;

use crate::game_classes::game::{Game, GameResult};
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour, ExecutedMove};
use crate::move_ordering::order_moves;
use crate::engine::evaluator::Evaluator;

pub const INF: i32 = 30_000;

#[derive(Clone, Copy)]
pub enum Bound {
    Exact,
    Lower,
    Upper,
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub depth: usize,
    pub value: i32,
    pub bound: Bound,
}



pub struct Minimax {
    pub max_depth: usize,
    pub quiescence_max_depth: usize,
    pub selective_quiescence: bool,
    pub tt: HashMap<u64, TTEntry>
}

impl Minimax {
    pub fn new(max_depth: usize, quiescence_max_depth: usize, selective_quiescence: bool) -> Self {
        Self { max_depth, quiescence_max_depth, selective_quiescence, tt: HashMap::new()}
    }

    pub fn find_best_move(&mut self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        self.tt.clear(); // clear TT for fresh search

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

    fn minimax(&mut self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32, colour: Colour) -> i32 {
        let hash = game.get_current_hash();

        if let Some(entry) = self.tt.get(&hash) {
            if entry.depth >= depth {
                match entry.bound {
                    Bound::Exact => return entry.value,
                    Bound::Lower => alpha = alpha.max(entry.value),
                    Bound::Upper => beta = beta.min(entry.value)
                }
                if alpha >= beta {
                    return entry.value;
                }
            }
        }

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
                break;
            }
        }

        let bound = if best_score <= alpha {
            Bound::Upper
        }
        else if  best_score >= beta {
            Bound::Lower
        }
        else {
            Bound::Exact
        };

        self.tt.insert(hash, TTEntry { depth: depth, value: best_score, bound: bound });
        best_score
    }



    fn quiescence(&mut self, game: &mut Game, mut alpha: i32, mut beta: i32, max_depth: usize) -> i32 {
        let hash = game.get_current_hash();

        if let Some(entry) = self.tt.get(&hash) {
            if entry.depth >= max_depth {
                match entry.bound {
                    Bound::Exact => return entry.value,
                    Bound::Lower => alpha = alpha.max(entry.value),
                    Bound::Upper => beta = beta.min(entry.value),
                }
                if alpha >= beta {
                    return entry.value;
                }
            }
        }
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

        let bound = if stand_pat <= alpha {
            Bound::Upper
        } else if stand_pat >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        self.tt.insert(hash, TTEntry { depth: self.max_depth, value: stand_pat, bound: bound });


        alpha
    }
}