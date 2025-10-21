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
    pub magic_bitboards: bool,
}

pub struct Minimax {
    pub engine_options: EngineOptions,
    pub tt: HashMap<u64, TTEntry>,

    // Debugging counters
    pub nodes: usize,
    pub tt_hits: usize,

    // move buffers: one Vec<ChessMove> per ply (0..=max_depth)
    pub move_buffers: Vec<Vec<ChessMove>>,
    // tactical buffers for quiescence (captures/promotions) per ply
    pub tactical_buffers: Vec<Vec<ChessMove>>,
}

impl Minimax {
    pub fn new(max_depth: usize, quiescence_max_depth: usize, tt_tables: bool, magic_bitboard: bool) -> Self {
        if magic_bitboard {
            MoveGenerator::init();
        }
        let options = EngineOptions {
            max_depth,
            quiescence_max_depth,
            use_transposition_tables: tt_tables,
            magic_bitboards: magic_bitboard,
        };

        // preallocate per-ply buffers: need max_depth + 2 to be safe (root + depths)
        let buffer_count = max_depth + 2;
        let mut move_buffers = Vec::with_capacity(buffer_count);
        for _ in 0..buffer_count {
            move_buffers.push(Vec::with_capacity(256));     // ~MAX_MOVES
        }

        let tact_buffer_count = quiescence_max_depth + 2;
        let mut tactical_buffers = Vec::with_capacity(tact_buffer_count);
        for _ in 0..tact_buffer_count {
            tactical_buffers.push(Vec::with_capacity(64));  // fewer tactical moves typically
        }

        Self {
            engine_options: options,
            tt: HashMap::new(),
            nodes: 0,
            tt_hits: 0,
            move_buffers,
            tactical_buffers,
        }
    }

    pub fn evaluate_move(&mut self, game: &mut Game, mv: &ChessMove) -> i32 {
        game.make_move(mv);
        let to_move = game.get_game_state().get_turn();

        // use ply 0 buffer for this temporary generation
        let ply = 0;
        self.move_buffers[ply].clear();
        MoveGenerator::generate_legal_moves_into(
            game,
            to_move,
            self.engine_options.magic_bitboards,
            &mut self.move_buffers[ply],
        );

        let game_result = game.is_game_over_with_moves(&self.move_buffers[ply], self.engine_options.magic_bitboards);
        let out = Evaluator::evaluate_game_result(game, game_result, 0, to_move);

        game.undo_last_move();
        out
    }

    pub fn find_best_move(&mut self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        let mut best_move: Option<ChessMove> = None;
        let mut best_score: i32 = -INF;

        for depth in 1..=self.engine_options.max_depth {
            let mut current_best: Option<ChessMove> = None;
            let mut current_best_score = -INF;

            // root is ply 0
            let root_ply = 0;
            self.move_buffers[root_ply].clear();
            MoveGenerator::generate_legal_moves_into(
                game,
                colour,
                self.engine_options.magic_bitboards,
                &mut self.move_buffers[root_ply],
            );
            order_moves(&mut self.move_buffers[root_ply], game);

            // PV move promotion
            if let Some(prev_best) = &best_move {
                if let Some(idx) = self.move_buffers[root_ply].iter().position(|m| m == prev_best) {
                    let mv = self.move_buffers[root_ply].remove(idx);
                    self.move_buffers[root_ply].insert(0, mv);
                }
            }

            let len = self.move_buffers[root_ply].len();
            for i in 0..len {
                // clone the move out (requires ChessMove: Clone)
                let mv = self.move_buffers[root_ply][i].clone();
                game.make_move(&mv);

                // recurse: pass ply = 1 for child
                let score = -self.minimax(game, depth - 1, -INF, INF, colour.other(), 1);

                game.undo_last_move();

                if score > current_best_score {
                    current_best_score = score;
                    current_best = Some(mv);
                }
            }

            if let Some(mv) = current_best {
                best_move = Some(mv);
                best_score = current_best_score;
            }
        }

        best_move
    }
    pub fn find_sorted_moves(&mut self, game: &mut Game, colour: Colour) -> Vec<(ChessMove, i32)> {
        let mut move_scores: Vec<(ChessMove, i32)> = Vec::new();

        // Use the configured max depth
        let depth = self.engine_options.max_depth;

        // root is ply 0
        let root_ply = 0;
        self.move_buffers[root_ply].clear();
        MoveGenerator::generate_legal_moves_into(
            game,
            colour,
            self.engine_options.magic_bitboards,
            &mut self.move_buffers[root_ply],
        );
        order_moves(&mut self.move_buffers[root_ply], game);

        let len = self.move_buffers[root_ply].len();
        for i in 0..len {
            let mv = self.move_buffers[root_ply][i].clone();
            game.make_move(&mv);

            // Recurse with minimax at ply 1
            let score = -self.minimax(game, depth - 1, -INF, INF, colour.other(), 1);

            game.undo_last_move();

            move_scores.push((mv, score));
        }

        // Sort descending by score
        move_scores.sort_by(|a, b| b.1.cmp(&a.1));

        move_scores
    }

    // minimax now takes an explicit ply parameter so each level uses its own buffers
    fn minimax(&mut self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32, colour: Colour, ply: usize) -> i32 {
        self.nodes += 1;
        let hash = game.get_current_hash();

        if self.engine_options.use_transposition_tables {
            if let Some(entry) = self.tt.get(&hash) {
                if !entry.is_quiescence && entry.depth >= depth {
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

        // generate moves into buffer for this ply
        self.move_buffers[ply].clear();
        MoveGenerator::generate_legal_moves_into(
            game,
            colour,
            self.engine_options.magic_bitboards,
            &mut self.move_buffers[ply],
        );
        order_moves(&mut self.move_buffers[ply], game);

        if let Some(result) = game.is_game_over_with_moves(&self.move_buffers[ply], self.engine_options.magic_bitboards) {
            return Evaluator::evaluate_game_result(game, Some(result), ply, colour);
        }

        if depth == 0 {
            return self.quiescence(game, alpha, beta, self.engine_options.quiescence_max_depth, 0);
        }

        let orig_alpha = alpha;
        let mut best_score = -INF;

        let len = self.move_buffers[ply].len();
        for i in 0..len {
            let mv = self.move_buffers[ply][i].clone();
            game.make_move(&mv);

            // recursive call will generate into move_buffers[ply + 1]
            let score = -self.minimax(game, depth - 1, -beta, -alpha, colour.other(), ply+1);

            game.undo_last_move();

            if score > best_score {
                best_score = score;
            }
            if best_score >= beta {
                break;
            }
            if best_score > alpha {
                alpha = best_score;
            }
        }

        if self.engine_options.use_transposition_tables {
            let bound = if best_score <= orig_alpha {
                Bound::Upper
            } else if best_score >= beta {
                Bound::Lower
            } else {
                Bound::Exact
            };

            let new_entry = TTEntry {
                depth,
                value: best_score,
                bound,
                is_quiescence: false,
            };

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

    // quiescence uses the tactical buffer for this ply
    fn quiescence(
        &mut self,
        game: &mut Game,
        mut alpha: i32,
        mut beta: i32,
        max_depth: usize,
        ply: usize,
    ) -> i32 {
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

        let to_move = game.get_game_state().get_turn();

        // stand pat
        let stand_pat = Evaluator::evaluate_game_result(game, None, ply, to_move);
        if max_depth == 0 || stand_pat >= beta {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        // generate only tactical moves into the tactical buffer for this ply
        self.tactical_buffers[ply].clear();
        MoveGenerator::generate_legal_moves_into(
            game,
            to_move,
            self.engine_options.magic_bitboards,
            &mut self.tactical_buffers[ply],
        );
        order_moves(&mut self.tactical_buffers[ply], game);

        if let Some(result) = game.is_game_over_with_moves(&self.tactical_buffers[ply], self.engine_options.magic_bitboards) {
            return Evaluator::evaluate_game_result(game, Some(result), ply, to_move);
        }

        let mut best_score = stand_pat;
        let len = self.tactical_buffers[ply].len();

        for i in 0..len {
            let mv = self.tactical_buffers[ply][i].clone();
            if !MoveGenerator::is_tactical_move(game, &mv, self.engine_options.magic_bitboards) {
                continue
            }
            game.make_move(&mv);
            let score = -self.quiescence(game, -beta, -alpha, max_depth - 1, ply + 1);
            game.undo_last_move();

            if score >= beta {
                return beta;
            }
            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        // store quiescence result if using TT (same logic as before)
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

        best_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::Coords;
    use crate::enums::moves::NormalMove;
    use crate::enums::{ChessMove, Colour, File, PieceType};
    use crate::game_classes::game::Game;

    /// Helper: set up a starting chess position
    fn starting_game() -> Game {
        Game::new()
    }

    #[test]
    fn test_evaluate_move_starting_position() {
        let mut game = starting_game();
        let mut engine = Minimax::new(3, 6, true, true);

        // Generate legal moves at the root
        engine.move_buffers[0].clear();
        MoveGenerator::generate_legal_moves_into(
            &mut game,
            Colour::White,
            false,
            &mut engine.move_buffers[0],
        );

        // Pick the first move to evaluate
        let mv = engine.move_buffers[0][0].clone();
        let eval = engine.evaluate_move(&mut game, &mv);

        // Evaluation should be within reasonable bounds for starting position
        assert!(eval.abs() < 1000, "Evaluation out of bounds: {}", eval);

        // Ensure the game state was restored
        assert_eq!(game.get_game_state().get_turn(), Colour::White);
    }

    #[test]
    fn test_quiescence_no_capture_starting_position() {
        let mut game = starting_game();
        let mut engine = Minimax::new(1, 2, false, false);

        // At the starting position, quiescence search should return the static evaluation
        let score = engine.quiescence(&mut game, -INF, INF, 2, 0);
        // The score should be within reasonable bounds for a balanced position
        assert!(score.abs() < 1000, "Quiescence evaluation out of bounds: {}", score);
    }

    #[test]
    fn test_find_best_move_returns_valid_move() {
        let mut game = starting_game();
        let mut engine = Minimax::new(1, 1, false, false);

        let best_move = engine.find_best_move(&mut game, Colour::White);

        assert!(best_move.is_some(), "Best move should not be None");

        // Make sure the move is legal
        let mv = best_move.unwrap();
        engine.move_buffers[0].clear();
        MoveGenerator::generate_legal_moves_into(
            &mut game,
            Colour::White,
            false,
            &mut engine.move_buffers[0],
        );
        assert!(engine.move_buffers[0].contains(&mv), "Best move is not legal");
    }

    #[test]
    fn test_minimax_basic_behavior() {
        let mut game = starting_game();
        let mut engine = Minimax::new(1, 1, false, false);

        // Pick one move from root and evaluate using minimax
        engine.move_buffers[0].clear();
        MoveGenerator::generate_legal_moves_into(
            &mut game,
            Colour::White,
            false,
            &mut engine.move_buffers[0],
        );
        let mv = engine.move_buffers[0][0].clone();

        game.make_move(&mv);
        let score = -engine.minimax(&mut game, 0, -INF, INF, Colour::Black, 1);
        game.undo_last_move();

        // Score should be finite
        assert!(score.abs() < 30_000, "Minimax returned unrealistic score: {}", score);
    }

    #[test]
    fn test_checkmate_detection() {
        let mut game = Game::new();
        let mut engine = Minimax::new(1, 1, false, false);

        // Set up Fool's Mate: 1. f3 e5 2. g4 Qh4#
        let f2_f3 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::F),
            to: Coords::new(4, File::F),
        });
        let e7_e5 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
            from: Coords::new(7, File::E),
            to: Coords::new(5, File::E),
        });
        let g2_g4 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::G),
            to: Coords::new(4, File::G),
        });
        let d8_h4 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Queen,
            from: Coords::new(8, File::D),
            to: Coords::new(4, File::H),
        });

        // Play the moves
        game.make_move(&f2_f3);
        game.make_move(&e7_e5);
        game.make_move(&g2_g4);

        let eval = engine.evaluate_move(&mut game, &d8_h4);
        println!("{:?}", eval);
        assert!(eval < 0, "Evaluation should indicate checkmate loss for White");
    }


    #[test]
    fn test_black_gets_checkmated() {
        let mut game = Game::new();
        let mut engine = Minimax::new(1, 0, false, false);

        let f2_f3 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::F),
            to: Coords::new(4, File::F),
        });
        let e7_e5 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
            from: Coords::new(7, File::E),
            to: Coords::new(5, File::E),
        });
        let g2_g4 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::G),
            to: Coords::new(4, File::G),
        });

        // Play the moves
        game.make_move(&f2_f3);
        game.make_move(&e7_e5);
        game.make_move(&g2_g4);


        let to_move = game.get_game_state().get_turn();
        let moves = engine.find_sorted_moves(&mut game, to_move);

        // Ensure there is at least one move
        assert!(!moves.is_empty(), "There should be at least one legal move for White");

        for (mv, eval) in &moves {
            println!("{}: {}", mv, eval);
        }

        // The best move should have the highest evaluation
        let (best_move, best_eval) = &moves[0];
        assert!(
            *best_eval > 28000,
            "Best move evaluation should indicate checkmate loss for White, got {}",
            best_eval
        );

        // Ensure the moves are sorted descending
        for w in moves.windows(2) {
            assert!(w[0].1 >= w[1].1, "Moves are not sorted by descending evaluation");
        }

    }

    #[test]
    fn test_white_gets_checkmated() {
        let mut game = Game::new();
        let mut engine = Minimax::new(1, 1, false, false);

        let e2_e4 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::E),
            to: Coords::new(4, File::E),
        });
        let f7_f6 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
            from: Coords::new(7, File::F),
            to: Coords::new(6, File::F),
        });
        let d2_d4 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::D),
            to: Coords::new(4, File::D),
        });
        let g7_g5 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
            from: Coords::new(7, File::G),
            to: Coords::new(5, File::G),
        });

        // Play the moves
        game.make_move(&e2_e4);
        game.make_move(&f7_f6);
        game.make_move(&d2_d4);
        game.make_move(&g7_g5);


        let to_move = game.get_game_state().get_turn();
        let moves = engine.find_sorted_moves(&mut game, to_move);

        // Ensure there is at least one move
        assert!(!moves.is_empty(), "There should be at least one legal move for White");

        for (mv, eval) in &moves {
            println!("{}: {}", mv, eval);
        }

        // The best move should have the highest evaluation
        let (best_move, best_eval) = &moves[0];
        assert!(
            *best_eval > 28000,
            "Best move evaluation should indicate checkmate loss for White, got {}",
            best_eval
        );

        // Ensure the moves are sorted descending
        for w in moves.windows(2) {
            assert!(w[0].1 >= w[1].1, "Moves are not sorted by descending evaluation");
        }

    }

//     fn test_checkmate_endgame() {
// position startpos moves e2e4 g8f6 e4e5 f6e4 d2d4 e7e6 f1d3 d8h4 g1h3 b8c6 c2c3 d7d5 e1g1
//                              h7h6 h3f4 e4g5 h2h3 c8d7 b1a3 f8a3 d1b3 a3e7 b3b7 a8c8 d3b5 c8b8 b7a6 c6e5 a2a4 d7b5 a4b5 e5d7 a6a7 e7d6 g2g3 g5h3 g1h1              
//                              h4g4 c3c4 g4f3 f4g2 d5c4 c1e3 f3h5 a7a8 h3f2 h1g1 f2h3 g1h1 h3f4 g2h4 f4d5 f1f5 e6f5 a8d5 d6g3 a1a7 h5h4 h1g1 h4e4 d5e4              
//                              f5e4 g1g2 g3d6 a7a4 b8b5 e3c1 d7b6 a4a2 e8g8 a2a7 c7c5 b2b3 c5d4 c1a3 d6a3 a7a3 c4b3 a3a6 b3b2 g2f2 b2b1q a6b6 b5f5                  
//                              f2g2 f5g5 g2h2 b1g1 h2h3 g1g4 h3h2 g4g1 h2h3 g1g4 h3h2 g4f4 h2h1 f4f1 h1h2 f1f4 h2h1 f4g4 h1h2
    // }
}
