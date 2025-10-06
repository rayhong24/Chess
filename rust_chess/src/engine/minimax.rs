use std::collections::HashMap;

use crate::game_classes::game::Game;
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour};
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
    pub is_quiescence: bool,
}


pub struct EngineOptions {
    pub max_depth: usize,
    pub quiescence_max_depth: usize,
    pub use_transposition_tables: bool,
}


pub struct Minimax {
    pub engine_options: EngineOptions,
    pub tt: HashMap<u64, TTEntry>,

    // Debugging counters
    pub nodes: usize,
    pub tt_hits: usize,
}

impl Minimax {
    pub fn new(max_depth: usize, quiescence_max_depth: usize, tt_tables: bool) -> Self {
        let options = EngineOptions { max_depth: max_depth, quiescence_max_depth: quiescence_max_depth, use_transposition_tables: tt_tables};
        Self { engine_options: options, tt: HashMap::new(), nodes: 0, tt_hits: 0}
    }

    pub fn evaluate_move(&mut self, game: &mut Game, mv: &ChessMove) -> i32 {
        game.make_move(mv);

        let to_move = game.get_game_state().get_turn();
        let moves = MoveGenerator::generate_legal_moves(game,to_move, false); 
        let game_result = game.is_game_over_with_moves(&moves);
        let out = Evaluator::evaluate_game_result(game, game_result, 0, to_move);

        game.undo_last_move();

        return out;
    }

    pub fn find_best_move(&mut self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        let mut best_score: i32 = -INF;
        let mut best_move: Option<ChessMove> = None;

        // Used for debugging mostly
        let mut move_scores = vec![];

        for depth in 1..=self.engine_options.max_depth {
            let mut current_best: Option<ChessMove> = None;
            let mut current_best_score = -INF;

            let mut moves = MoveGenerator::generate_legal_moves(game, colour, false);
            order_moves(&mut moves, game);

            // Try to promote previous iteration best (PV) to front for ordering:
            if let Some(prev_best) = &best_move {
                if let Some(idx) = moves.iter().position(|m| m == prev_best) {
                    let pv = moves.remove(idx);
                    moves.insert(0, pv);
                }
            }

            for mv in moves {
                game.make_move(&mv);
                let score = -self.minimax(game, depth-1, -INF, INF, colour.other());
                game.undo_last_move();

                move_scores.push((mv, score));

                if score > current_best_score {
                    current_best_score = score;
                    current_best = Some(mv);
                }
            }

            if let Some(mv) = current_best.clone() {
                best_move = Some(mv);
                best_score = current_best_score;
            }

            println!("Depth {}: best move = {:?}, score = {}", depth, best_move, best_score);
        }

        // move_scores.sort_by(|a, b| b.1.cmp(&a.1));

        // for (mv, score) in &move_scores {
        //     println!("{}: {}", mv, score);
        // }


        best_move
    }

    fn minimax(&mut self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32, colour: Colour) -> i32 {
        self.nodes += 1;
        let hash = game.get_current_hash();

        if self.engine_options.use_transposition_tables {
            if let Some(entry) = self.tt.get(&hash) {
                if !entry.is_quiescence && entry.depth >= depth {
                    self.tt_hits += 1;
                    match entry.bound {
                        Bound::Exact => return entry.value,
                        Bound::Lower => alpha = alpha.max(entry.value),
                        Bound::Upper => beta = beta.min(entry.value)
                    }
                    if alpha >= beta {
                        return entry.value;
                    }
            }   }
        }

        let moves = MoveGenerator::generate_legal_moves(game, colour, false);

        if let Some(result) = game.is_game_over_with_moves(&moves) {
            return Evaluator::evaluate_game_result(game, Some(result), depth, colour);
        }

        if depth == 0 {
            return self.quiescence(game, alpha, beta, self.engine_options.quiescence_max_depth);
        }
        

        let orig_alpha = alpha;
        let mut best_score = -INF;

        for mv in &moves {
            game.make_move(&mv);
            let score = -self.minimax(game, depth - 1, -beta, -alpha, colour.other());
            game.undo_last_move();

            best_score = best_score.max(score);

            if best_score >= beta {
                break;
            }
        }

        if self.engine_options.use_transposition_tables {
            let bound = if best_score <= orig_alpha {
                Bound::Upper
            }
            else if  best_score >= beta {
                Bound::Lower
            }
            else {
                Bound::Exact
            };

            let new_entry = TTEntry { depth: depth, value: best_score, bound: bound, is_quiescence: false };

            // Only overwrite if new entry is at least as deep as the existing one,
            // or if the existing entry is from quiescence (we prefer main-search entries).
            match self.tt.get(&hash) {
                Some(existing) => {
                    let should_replace = (!existing.is_quiescence && new_entry.depth >= existing.depth)
                        || (existing.is_quiescence && !new_entry.is_quiescence);
                    if should_replace {
                        self.tt.insert(hash, new_entry);
                    }
                }
                None => {
                    self.tt.insert(hash, new_entry);
                }
            }
        }
        best_score
    }



    fn quiescence(&mut self, game: &mut Game, mut alpha: i32, mut beta: i32, max_depth: usize) -> i32 {
        self.nodes += 1;

        let hash = game.get_current_hash();

        if self.engine_options.use_transposition_tables {
            if let Some(entry) = self.tt.get(&hash) {
                if entry.is_quiescence && entry.depth >= max_depth {
                    self.tt_hits += 1;
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
        }

        // Step 0: terminal positions
        let to_move = game.get_game_state().get_turn();
        let moves = MoveGenerator::generate_legal_moves(game, to_move, false);

        if let Some(result) = game.is_game_over_with_moves(&moves) {
            return Evaluator::evaluate_game_result(game, Some(result), self.engine_options.quiescence_max_depth, to_move);
        }

        // Step 1: stand pat evaluation
        let stand_pat = Evaluator::evaluate_game_result(game, None, self.engine_options.quiescence_max_depth, to_move);

        if max_depth == 0 || stand_pat >= beta {
            // store stand_pat as exact quiescence result (depth==0 case)
            if self.engine_options.use_transposition_tables {
                let new_entry = TTEntry {
                    depth: max_depth,
                    value: stand_pat,
                    bound: Bound::Exact,
                    is_quiescence: true,
                };
                match self.tt.get(&hash) {
                    Some(existing) => {
                        let should_replace = (existing.is_quiescence && new_entry.depth >= existing.depth)
                            || (!existing.is_quiescence); // keep main-search if it exists deeper
                        if should_replace {
                            self.tt.insert(hash, new_entry);
                        }
                    }
                    None => {
                        self.tt.insert(hash, new_entry);
                    }
                }
            }
            return stand_pat;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        // Step 2: generate only tactical moves (captures + promotions)
        let mut moves = MoveGenerator::generate_legal_moves(game, game.get_game_state().get_turn(), false);
        order_moves(&mut moves, game);

        let mut best_score = stand_pat;

        for mv in &moves {
            if !matches!(mv, ChessMove::Promotion(_)) && !game.is_capture(mv) {
                continue;
            }

            game.make_move(mv);
            let score = -self.quiescence(game, -beta, -alpha, max_depth - 1);
            game.undo_last_move();

            if score >= beta {
                // store a lower-bound (beta cutoff) for quiescence result
                if self.engine_options.use_transposition_tables {
                    let new_entry = TTEntry {
                        depth: max_depth,
                        value: beta,
                        bound: Bound::Lower,
                        is_quiescence: true,
                    };
                    match self.tt.get(&hash) {
                        Some(existing) => {
                            let should_replace = (existing.is_quiescence && new_entry.depth >= existing.depth)
                                || (!existing.is_quiescence);
                            if should_replace {
                                self.tt.insert(hash, new_entry);
                            }
                        }
                        None => {
                            self.tt.insert(hash, new_entry);
                        }
                    }
                }
                return beta;
            }

            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        // store quiescence result (Exact if no cutoff happened)
        if self.engine_options.use_transposition_tables {
            let new_entry = TTEntry {
                depth: max_depth,
                value: best_score,
                bound: Bound::Exact,
                is_quiescence: true,
            };
            match self.tt.get(&hash) {
                Some(existing) => {
                    let should_replace = (existing.is_quiescence && new_entry.depth >= existing.depth)
                        || (!existing.is_quiescence);
                    if should_replace {
                        self.tt.insert(hash, new_entry);
                    }
                }
                None => {
                    self.tt.insert(hash, new_entry);
                }
            }
        }

        alpha
    }
}