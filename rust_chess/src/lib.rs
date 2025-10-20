use pyo3::prelude::*;

use crate::game_classes::board_classes::magic_bitboard;
use crate::moves::move_generator;
use crate::game_classes::game::Game;
use crate::moves::move_parser::MoveParser;
use crate::engine::minimax::Minimax;

pub mod coords;
pub mod piece;
pub mod enums;
pub mod moves;
pub mod game_classes;
pub mod engine;
pub mod move_ordering;

#[pymodule]
fn rust_chess(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGame>()?;
    m.add_class::<PyMinimax>()?;
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}

#[pyclass]
pub struct PyGame {
    inner: Game,
}

#[pymethods]
impl PyGame {
    #[new]
    fn new() -> Self {
        Self { inner: Game::new() }
    }

    fn make_move(&mut self, mv: &str) {
        let chess_move = MoveParser::parse_str(mv, &self.inner).unwrap();
        self.inner.make_move(&chess_move);
    }

    fn undo_last_move(&mut self) {
        self.inner.undo_last_move();
    }

    fn set_fenstr(&mut self, fenstr: &str) {
        self.inner.set_fenstr(fenstr);
    }

    fn get_legal_moves(&mut self, colour: &str) -> Vec<String> {
        let colour = match colour.to_lowercase().as_str() {
            "white" => enums::Colour::White,
            "black" => enums::Colour::Black,
            _ => panic!("Invalid colour"),
        };

        let mut moves = Vec::new();
        move_generator::MoveGenerator::generate_legal_moves_into(&mut self.inner, colour, false, &mut moves);
        moves.iter().map(|m| m.to_string()).collect()
    }
}

#[pyclass]
pub struct PyMinimax {
    inner: Minimax,
    game: Game
}

#[pymethods]
impl PyMinimax {
    #[new]
    pub fn new(max_depth:usize, quiescence_max_depth: usize, selective_quiescence: bool, magic_bitboard: bool) -> Self {
        Self { inner: Minimax::new(max_depth, quiescence_max_depth, selective_quiescence, magic_bitboard) , game: Game::new() }
    }

    pub fn go(&mut self) -> String {
        let colour = self.game.get_game_state().get_turn();
        // println!("Current board eval: {}", self.inner.evaluate(&self.game, colour));
        let best = self.inner.find_best_move(&mut self.game, colour);
        return best.unwrap().to_string();
    }

    pub fn set_position(&mut self, fenstr: &str, moves: Vec<String>) {
        self.game.set_fenstr(fenstr);

        for m in moves {
            if let Some(chess_move) = MoveParser::parse_str(&m, &self.game) {
                self.game.make_move(&chess_move);
            }
        }
    }

    pub fn evaluate_move(&mut self, mv: &str) -> i32 {
        if let Some(chess_move) = MoveParser::parse_str(&mv, &self.game) {
            return self.inner.evaluate_move(&mut self.game, &chess_move);
        }

        panic!("Invalid move");
    }

    /// Engine option setters
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.inner.engine_options.max_depth = max_depth;
    }

    pub fn set_quiescence_max_depth(&mut self, quiescence_max_depth: usize) {
        self.inner.engine_options.quiescence_max_depth = quiescence_max_depth;
    }

    pub fn set_use_transposition_tables(&mut self, use_tt: bool) {
        self.inner.engine_options.use_transposition_tables = use_tt;
    }

    pub fn set_use_magic_bitboards(&mut self, use_magic_bitboards: bool) {
        self.inner.engine_options.magic_bitboards = use_magic_bitboards;
    }

    /// Engine option getters
    pub fn get_max_depth(&self) -> usize {
        self.inner.engine_options.max_depth
    }

    pub fn get_quiescence_max_depth(&self) -> usize {
        self.inner.engine_options.quiescence_max_depth
    }

    pub fn get_use_transposition_tables(&self) -> bool {
        self.inner.engine_options.use_transposition_tables
    }

    /// Optional: reset the engine TT if you want to start fresh
    pub fn clear_tt(&mut self) {
        self.inner.tt.clear();
    }

    pub fn reset_minimax_nodes_and_tt_hits(&mut self) {
        self.inner.nodes = 0;
        self.inner.tt_hits = 0;
    }

    pub fn get_minimax_nodes(&self) -> usize {
        self.inner.nodes
    }

    pub fn get_minimax_tt_hits(&self) -> usize {
        self.inner.tt_hits
    }
}


