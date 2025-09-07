use pyo3::prelude::*;

use crate::moves::move_generator;
use crate::game_classes::game::Game;

pub mod coords;
pub mod piece;
pub mod enums;
pub mod moves;
pub mod game_classes;

#[pymodule]
fn rust_chess(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGame>()?;
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
        println!("{}", mv);
        // let chess_move = self.inner.parse_move(mv).unwrap();
        // self.inner.make_move(&chess_move);
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


