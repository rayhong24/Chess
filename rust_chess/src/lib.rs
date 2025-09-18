use pyo3::prelude::*;

use crate::moves::move_generator;
use crate::game_classes::game::Game;
use crate::moves::move_parser::MoveParser;
use crate::minimax::Minimax;

pub mod coords;
pub mod piece;
pub mod enums;
pub mod moves;
pub mod game_classes;
pub mod minimax;
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

        let moves = move_generator::MoveGenerator::generate_legal_moves(&mut self.inner, colour);
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
    pub fn new(max_depth:usize) -> Self {
        Self { inner: Minimax::new(max_depth) , game: Game::new() }
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
}


