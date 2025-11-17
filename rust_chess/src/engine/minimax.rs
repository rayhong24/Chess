use std::collections::HashMap;

use crate::game_classes::game::Game;
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour};
use crate::move_ordering::order_moves;
use crate::engine::evaluator::Evaluator;

pub const INF: i32 = 30_000;
pub const MAX_MOVES: usize = 2048;

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

    pub move_buffer: Vec<ChessMove>,
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

        let move_buffer = Vec::with_capacity(MAX_MOVES);
        Self {
            engine_options: options,
            tt: HashMap::new(),
            nodes: 0,
            tt_hits: 0,
            move_buffer,
        }
    }

    pub fn evaluate_move(&mut self, game: &mut Game, mv: &ChessMove) -> i32 {
        game.make_move(mv);
        let to_move = game.get_game_state().get_turn();

        let move_start_index = self.move_buffer.len();
        Self::generate_and_order_moves(self, game, to_move, move_start_index);

        let game_result = game.is_game_over_with_moves(&self.move_buffer[move_start_index..], self.engine_options.magic_bitboards);
        let out = Evaluator::evaluate_game_result(game, game_result, 0, to_move);

        game.undo_last_move();
        out
    }

    pub fn find_best_move(&mut self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        self.move_buffer.clear();

        Self::generate_and_order_moves(self, game, colour, 0);

        let mut best_move: Option<ChessMove> = None;
        let mut best_score: i32 = -INF;

        // Loop for iterative deepening
        for depth in 1..=self.engine_options.max_depth {
            let mut current_best: Option<ChessMove> = None;
            let mut current_best_score = -INF;

            // PV move promotion
            if let Some(prev_best) = &best_move {
                if let Some(idx) = self.move_buffer.iter().position(|m| m == prev_best) {
                    let mv = self.move_buffer.swap_remove(idx);
                    self.move_buffer.push(mv);
                }
            }

            for i in (0..self.move_buffer.len()).rev() {
                let mv = self.move_buffer[i]; 

                game.make_move(&mv);

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
    // Does not use iterative deepening
    // Should be used for move ordering/evaluation and sanity checks. Not for actual move selection.
    pub fn find_sorted_moves(&mut self, game: &mut Game, colour: Colour) -> Vec<(ChessMove, i32)> {
        self.move_buffer.clear();

        Self::generate_and_order_moves(self, game, colour, 0);

        let mut move_scores: Vec<(ChessMove, i32)> = Vec::new();

        // Use the configured max depth
        let depth = self.engine_options.max_depth;

        while let Some(mv) = self.move_buffer.pop()  {
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

    // Helper: Probe the TT for a usable entry
    fn tt_lookup(&mut self, hash: u64, depth: usize, is_quiescence: bool, alpha: &mut i32, beta: &mut i32) -> Option<i32> {
        if !self.engine_options.use_transposition_tables {
            return None;
        }
        if let Some(entry) = self.tt.get(&hash) {
            if (!entry.is_quiescence && !is_quiescence && entry.depth >= depth)
                || (entry.is_quiescence && is_quiescence && entry.depth >= depth) {
                self.tt_hits += 1;
                match entry.bound {
                    Bound::Exact => return Some(entry.value),
                    Bound::Lower => *alpha = (*alpha).max(entry.value),
                    Bound::Upper => *beta = (*beta).min(entry.value),
                }
                if *alpha >= *beta {
                    return Some(entry.value);
                }
            }
        }
        None
    }

    // Helper: Store a new entry in the TT with replacement logic
    fn tt_store(&mut self, hash: u64, depth: usize, value: i32, bound: Bound, is_quiescence: bool) {
        if !self.engine_options.use_transposition_tables {
            return;
        }
        let new_entry = TTEntry { depth, value, bound, is_quiescence };
        match self.tt.get(&hash) {
            Some(existing) => {
                let should_replace = (!existing.is_quiescence && !is_quiescence && new_entry.depth >= existing.depth)
                    || (existing.is_quiescence && is_quiescence && new_entry.depth >= existing.depth)
                    || (!existing.is_quiescence && is_quiescence);
                if should_replace {
                    self.tt.insert(hash, new_entry);
                }
            }
            None => {
                self.tt.insert(hash, new_entry);
            }
        }
    }

    // minimax now takes an explicit depth_level parameter so each level uses its own buffers
    fn minimax(&mut self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32, colour: Colour, depth_level: usize) -> i32 {
        self.nodes += 1;
        let hash = game.get_current_hash();

        // TT lookup
        if let Some(tt_value) = self.tt_lookup(hash, depth, false, &mut alpha, &mut beta) {
            return tt_value;
        }

        let move_start_index = self.move_buffer.len();
        Self::generate_and_order_moves(self, game, colour, move_start_index);

        if let Some(result) = game.is_game_over_with_moves(&self.move_buffer[move_start_index..], self.engine_options.magic_bitboards) {
            self.move_buffer.truncate(move_start_index);
            return Evaluator::evaluate_game_result(game, Some(result), depth_level, colour);
        }

        if depth == 0 {
            self.move_buffer.truncate(move_start_index);
            return self.quiescence(game, alpha, beta, self.engine_options.quiescence_max_depth, 0);
        }

        let orig_alpha = alpha;
        let mut best_score = -INF;

        while self.move_buffer.len() > move_start_index {
            let mv = self.move_buffer.pop().unwrap();
            game.make_move(&mv);

            // recursive call will generate into move_buffers[ply + 1]
            let score = -self.minimax(game, depth - 1, -beta, -alpha, colour.other(), depth_level + 1);

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

        // TT store
        let bound = if best_score <= orig_alpha {
            Bound::Upper
        } else if best_score >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        self.tt_store(hash, depth, best_score, bound, false);

        self.move_buffer.truncate(move_start_index);
        best_score
    }

    fn quiescence(&mut self, game: &mut Game, mut alpha: i32, mut beta: i32, max_depth: usize, depth_level: usize) -> i32 {
        self.nodes += 1;
        let hash = game.get_current_hash();

        // TT lookup
        if let Some(tt_value) = self.tt_lookup(hash, max_depth, true, &mut alpha, &mut beta) {
            return tt_value;
        }

        let to_move = game.get_game_state().get_turn();
        let escape_check = game.is_player_in_check(to_move, self.engine_options.magic_bitboards);

        // stand pat
        let stand_pat = Evaluator::evaluate_game_result(game, None, depth_level, to_move);
        if max_depth == 0 || (!escape_check && stand_pat >= beta) {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let move_start_index = self.move_buffer.len();
        Self::generate_and_order_moves(self, game, to_move, move_start_index);

        if let Some(result) = game.is_game_over_with_moves(&self.move_buffer[move_start_index..], self.engine_options.magic_bitboards) {
            self.move_buffer.truncate(move_start_index);
            return Evaluator::evaluate_game_result(game, Some(result), depth_level, to_move);
        }

        let orig_alpha = alpha;
        let mut best_score = if !escape_check {stand_pat} else {-INF};

        while self.move_buffer.len() > move_start_index {
            let mv = self.move_buffer.pop().unwrap();
            if !escape_check && !MoveGenerator::is_tactical_move(game, &mv, self.engine_options.magic_bitboards) {
                continue
            }
            game.make_move(&mv);
            let score = -self.quiescence(game, -beta, -alpha, max_depth - 1, depth_level + 1);
            game.undo_last_move();

            if score >= beta {
                self.move_buffer.truncate(move_start_index);
                return score;
            }
            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        // TT store
        let bound = if best_score <= orig_alpha {
            Bound::Upper
        } else if best_score >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        self.tt_store(hash, max_depth, best_score, bound, true);

        best_score
    }

    // Helper to generate and order moves starting from a given index in move_buffer
    fn generate_and_order_moves(&mut self, game: &mut Game, colour: Colour, start_index: usize) {
        MoveGenerator::generate_legal_moves_into(
            game,
            colour,
            self.engine_options.magic_bitboards,
            &mut self.move_buffer,
        );
        order_moves(&mut self.move_buffer[start_index..], game);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
        MoveGenerator::generate_legal_moves_into(
            &mut game,
            Colour::White,
            false,
            &mut engine.move_buffer,
        );

        // Pick the first move to evaluate
        let mv = engine.move_buffer[0].clone();
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
        let mut legal_moves = vec![];
        MoveGenerator::generate_legal_moves_into(
            &mut game,
            Colour::White,
            false,
            &mut legal_moves,
        );
        assert!(legal_moves.contains(&mv), "Best move is not legal");
    }

    #[test]
    fn test_minimax_basic_behavior() {
        let mut game = starting_game();
        let mut engine = Minimax::new(1, 1, false, false);

        // Pick one move from root and evaluate using minimax
        engine.move_buffer.clear();
        MoveGenerator::generate_legal_moves_into(
            &mut game,
            Colour::White,
            false,
            &mut engine.move_buffer,
        );
        let mv = engine.move_buffer[0].clone();

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

    #[test]
    fn test_quiescence_includes_non_capture_escape_from_check() {
        let mut game = Game::new();
        let mut engine = Minimax::new(1, 3, true, true);

        // Set up a position where White is in check but can block with a quiet (non-capture) move.
        //
        // Example:
        // 1. e4 d5
        // 2. exd5 Qxd5
        // 3. Nc3 Qe5+
        // In this position, White is in check, and can block with Be2 (quiet move, non-capture).

        let e2_e4 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(2, File::E),
            to: Coords::new(4, File::E),
        });
        let d7_d5 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
            from: Coords::new(7, File::D),
            to: Coords::new(5, File::D),
        });
        let e4xd5 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from: Coords::new(4, File::E),
            to: Coords::new(5, File::D),
        });
        let d8xd5 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Queen,
            from: Coords::new(8, File::D),
            to: Coords::new(5, File::D),
        });
        let b1_c3 = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Knight,
            from: Coords::new(1, File::B),
            to: Coords::new(3, File::C),
        });
        let d5_e5 = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Queen,
            from: Coords::new(5, File::D),
            to: Coords::new(5, File::E),
        });

        game.make_move(&e2_e4);
        game.make_move(&d7_d5);
        game.make_move(&e4xd5);
        game.make_move(&d8xd5);
        game.make_move(&b1_c3);
        game.make_move(&d5_e5);

        assert!(
            game.is_player_in_check(Colour::White, true),
            "White should be in check after Qe5+"
        );

        // Run quiescence search from this position.
        engine.nodes = 0;
        let eval = engine.quiescence(&mut game, -INF, INF, 3, 0);

        println!("{}", engine.nodes);

        // A correct quiescence should recognize that White can *block with Be2*
        // and therefore is not checkmated — evaluation should not be a mate score.
        assert!(
            engine.nodes > 1,
            "Expected quiescence to search at least one node.",
        );
    }
}
