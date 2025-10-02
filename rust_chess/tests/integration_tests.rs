use std::time::Instant;

use rust_chess::PyMinimax;

const startpos: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn test_start_search() {
    let mut mini = PyMinimax::new(2, 4, true);

    mini.set_position(startpos, vec![]);


    mini.go();

}

#[test]
fn test_minimax() {
    let mut mini = PyMinimax::new(2, 4, true);

    let moves_str = "g1h3 d7d5 h1g1 c8h3 g2h3 e7e5 g1g4 h7h5 g4g1 b8c6 g1g3 g7g6 g3g1 d8f6 g1g3 f8c5 g3g2 f6f5 b1c3 g8f6 a1b1 e8c8 b1a1 e5e4 a1b1 c6b4 b1a1 e4e3 f2e3 d8e8";
    let moves: Vec<String> = moves_str
        .split_whitespace()
        .map(|m| m.to_string())
        .collect();



    mini.set_position(startpos, moves);


    mini.go();
}

#[test]
fn test_transposition_tables() {
    let mut no_tt = PyMinimax::new(2, 4, false);
    let mut tt = PyMinimax::new(2, 4, true);

    let moves_str = "g1h3 d7d5 h1g1 c8h3 g2h3 e7e5 g1g4 h7h5 g4g1 b8c6 g1g3 g7g6 g3g1 d8f6 g1g3 f8c5 g3g2 f6f5 b1c3 g8f6 a1b1 e8c8 b1a1 e5e4 a1b1 c6b4 b1a1 e4e3 f2e3 d8e8";
    let moves: Vec<String> = moves_str
        .split_whitespace()
        .map(|m| m.to_string())
        .collect();


    no_tt.set_position(startpos, moves.clone());
    tt.set_position(startpos, moves.clone());


    // Measure no_tt runtime
    let start = Instant::now();
    let no_tt_bestmove = no_tt.go();
    let no_tt_duration = start.elapsed();

    // Measure tt runtime
    let start = Instant::now();
    let tt_bestmove = tt.go();
    let tt_duration = start.elapsed();

    // Both should agree on evaluation and best move
    assert_eq!(no_tt_bestmove, tt_bestmove, "Best moves differ between TT and no-TT");

    // TT should be faster (give some leeway since timing is noisy)
    assert!(
        tt_duration < no_tt_duration,
        "Expected TT search to be faster (no_tt: {:?}, tt: {:?})",
        no_tt_duration,
        tt_duration
    );

}
