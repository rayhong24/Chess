use std::time::Instant;
use rust_chess::{enums::moves, PyMinimax};

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn test_start_search() {
    let mut mini = PyMinimax::new(2, 4, true, true);
    mini.set_position(STARTPOS, vec![]);
    let best_move = mini.go();
    println!("Best move from start pos: {}", best_move);
}

#[test]
fn test_minimax() {
    let mut mini = PyMinimax::new(2, 4, true, true);

    let moves_str = "g1h3 d7d5 h1g1 c8h3 g2h3 e7e5 g1g4 h7h5 g4g1 b8c6 g1g3 g7g6 g3g1 d8f6 \
                     g1g3 f8c5 g3g2 f6f5 b1c3 g8f6 a1b1 e8c8 b1a1 e5e4 a1b1 c6b4 b1a1 e4e3 f2e3 d8e8";
    let moves: Vec<String> = moves_str.split_whitespace().map(|m| m.to_string()).collect();

    mini.set_position(STARTPOS, moves);
    let best_move = mini.go();
    println!("Best move after moves sequence: {}", best_move);
}

#[test]
fn test_repetition_draw_detection() {
    let mut mini = PyMinimax::new(2, 4, true, true);

    // Moves in long algebraic notation
    let moves_str = "g1f3 c7c5 d2d4 c5d4 f3d4 g7g6 b1c3 g8f6 e2e4 f8g7 e4e5 f6g8 c1f4 e7e6 f1c4 g6g5 \
                                d1g4 g8h6 g4g5 h6f5 g5d8 e8d8 d4f5 e6f5 c4f7 b8c6 f4g5 d8c7 g5f6 h8f8 f6g7 f8f7 \
                                c3d5 c7b8 g7f6 h7h5 e1g1 b7b6 f1d1 c8b7 d1e1 c6d8 d5b6 a7b6 f6d8 b8c8 d8b6 f7g7 g2g3 g7e7 \
                                b6c5 e7e8 c5d6 b7e4 e1e2 e4f3 e2e1 c8b7 h2h3 e8c8 e1e3 f3d5 a1d1 d5a2 e3e2 a2c4 e2e1 \
                                c4a2 d1d2 a2c4 d2d1 c4g8 e1e2 g8c4";


    let moves: Vec<String> = moves_str.split_whitespace().map(|m| m.to_string()).collect();
    println!("len_moves = {}", moves.len());

    mini.set_position(STARTPOS, moves);

    // Run the engine (best move not critical here)
    let mv = "e2e1";

    assert_eq!(0, mini.evaluate_move(mv), "e1e2 should result in threefold repetition");
    let best_move = mini.go();
    println!("Best move after moves sequence: {}", best_move);

    // Check e2e1 is not the best move 
    assert!(
        best_move != "e2e1",
        "Best move should not be e1e2 because that results in draw by threefold repetition"
    );
}

#[test]
fn test_transposition_tables() {
    let mut no_tt = PyMinimax::new(2, 4, false, true);
    let mut tt = PyMinimax::new(2, 4, true, true);

    let moves_str = "g1h3 d7d5 h1g1 c8h3 g2h3 e7e5 g1g4 h7h5 g4g1 b8c6 g1g3 g7g6 g3g1 d8f6 \
                     g1g3 f8c5 g3g2 f6f5 b1c3 g8f6 a1b1 e8c8 b1a1 e5e4 a1b1 c6b4 b1a1 e4e3 f2e3 d8e8";
    let moves: Vec<String> = moves_str.split_whitespace().map(|m| m.to_string()).collect();

    no_tt.set_position(STARTPOS, moves.clone());
    tt.set_position(STARTPOS, moves.clone());

    let start = Instant::now();
    let no_tt_bestmove = no_tt.go();
    let no_tt_duration = start.elapsed();

    let start = Instant::now();
    let tt_bestmove = tt.go();
    let tt_duration = start.elapsed();

    println!("no_tt time: {:?}, tt time: {:?}", no_tt_duration, tt_duration);
    println!("no_tt bestmove: {:?}, tt bestmove: {:?}", no_tt_bestmove, tt_bestmove);

    assert_eq!(no_tt_bestmove, tt_bestmove, "Best moves differ between TT and no-TT");
    assert!(tt_duration <= no_tt_duration, "TT should be faster (no_tt: {:?}, tt: {:?})", no_tt_duration, tt_duration);
}

#[test]
fn test_iterative_deepening_consistency() {
    let mut mini = PyMinimax::new(1, 4, true, true);
    mini.set_position(STARTPOS, vec![]);

    let mut last_best_move: Option<String> = None;

    for depth in 1..=3 {
        mini.set_max_depth(depth);
        let best_move = mini.go();
        println!("Depth {}: best move = {}", depth, best_move);

        // Should always return some move
        assert!(!best_move.is_empty(), "No best move found at depth {}", depth);

        if let Some(last) = &last_best_move {
            println!("Previous: {}, Current: {}", last, best_move);
        }

        last_best_move = Some(best_move);
    }
}

#[test]
fn test_iterative_deepening_tt_usage() {
    let mut mini = PyMinimax::new(3, 4, true, true);
    mini.set_position(STARTPOS, vec![]);
    
    mini.clear_tt();  // Ensure TT is empty
    mini.reset_minimax_nodes_and_tt_hits();

    mini.go();

    assert!(mini.get_minimax_tt_hits() > 0, "Expected transposition table hits during search");
    println!("Nodes searched: {}", mini.get_minimax_nodes());
    println!("TT hits: {}", mini.get_minimax_tt_hits());
}

#[test]
fn test_iterative_deepening_speedup() {
    let mut no_tt = PyMinimax::new(3, 4, false, true);
    let mut tt = PyMinimax::new(3, 4, true, true);

    no_tt.set_position(STARTPOS, vec![]);
    tt.set_position(STARTPOS, vec![]);

    let start = Instant::now();
    no_tt.go();
    let no_tt_duration = start.elapsed();

    let start = Instant::now();
    tt.go();
    let tt_duration = start.elapsed();

    println!("No TT: {:?}, TT: {:?}", no_tt_duration, tt_duration);
    assert!(tt_duration <= no_tt_duration, "TT search should be faster than no TT");
}

#[test]
fn test_iterative_deepening_tt_hits() {
    let mut mini = PyMinimax::new(3, 4, true, true);
    mini.set_position(STARTPOS, vec![]);

    for depth in 1..=4 {
        mini.set_max_depth(depth);
        mini.reset_minimax_nodes_and_tt_hits();

        mini.go();

        println!(
            "Depth {}: nodes searched = {}, TT hits = {}",
            depth, mini.get_minimax_nodes(), mini.get_minimax_tt_hits()
        );

        // TT hits should increase as depth increases
        // assert!(mini.get_minimax_tt_hits() > 0, "Expected TT hits at depth {}", depth);
    }
}

#[test]
fn test_nodes_per_second_comparison() {
    let mut engines = vec![
        ("No TT", PyMinimax::new(2, 4, false, false)),
        ("TT", PyMinimax::new(2, 4, true, false)),
    ];

    for (name, engine) in &mut engines {
        engine.set_position(STARTPOS, vec![]);
        engine.reset_minimax_nodes_and_tt_hits();

        let start = Instant::now();
        engine.go();
        let duration = start.elapsed();

        let nodes = engine.get_minimax_nodes();
        let nps = nodes as f64 / duration.as_secs_f64();

        println!(
            "{}: nodes = {}, time = {:.3?}, nodes/sec = {:.2}",
            name, nodes, duration, nps
        );
    }
}

#[test]
fn test_nodes_per_second_comparison_magic_bitboards() {
    let mut engines = vec![
        ("Move Rays", PyMinimax::new(2, 4, false, false)),
        // ("Magic Bitboards", PyMinimax::new(2, 4, false, true)),
    ];

    for (name, engine) in &mut engines {
        engine.set_position(STARTPOS, vec![]);
        engine.reset_minimax_nodes_and_tt_hits();

        let start = Instant::now();
        engine.go();
        let duration = start.elapsed();

        let nodes = engine.get_minimax_nodes();
        let nps = nodes as f64 / duration.as_secs_f64();

        println!(
            "{}: nodes = {}, time = {:.3?}, nodes/sec = {:.2}",
            name, nodes, duration, nps
        );
    }
}