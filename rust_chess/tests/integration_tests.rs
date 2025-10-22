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
        ("Magic Bitboards", PyMinimax::new(2, 4, false, true)),
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
fn test_nodes_per_second_comparison_magic_bitboards_complicated() {
    let mut engines = vec![
        ("Move Rays", PyMinimax::new(2, 4, false, false)),
        ("Magic Bitboards", PyMinimax::new(2, 4, false, true)),
    ];


    let moves_str = "g1h3 d7d5 h1g1 c8h3 g2h3 e7e5 g1g4 h7h5 g4g1 b8c6 g1g3 g7g6 g3g1 d8f6 \
                     g1g3 f8c5 g3g2 f6f5 b1c3 g8f6 a1b1 e8c8 b1a1 e5e4 a1b1 c6b4 b1a1 e4e3 f2e3 d8e8";
    let moves: Vec<String> = moves_str.split_whitespace().map(|m| m.to_string()).collect();

    for (name, engine) in &mut engines {
        engine.set_position(STARTPOS, moves.clone());
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
fn test_castling_through_pawn_attack_real_game() {
    let moves: Vec<String> = "g1f3 c7c5 b1c3 b8c6 d2d4 c5d4 f3d4 e7e6 d4c6 b7c6 c1f4 g8f6 e2e4 d7d5 e4e5 f6d7 d1f3 h7h5 f1d3 d5d4".split(' ').map(|s| s.to_string()).collect();

    let mut engine = PyMinimax::new(3, 4, true, false);

    engine.set_position(STARTPOS, moves);

    let moves = engine.go();

    // Ensure there is at least one move
    assert!(!moves.is_empty(), "There should be at least one legal move for White");
}

#[test]
fn test_() {
    let moves: Vec<String> = "g1f3 g8f6 b1c3 d7d5 d2d4 c8f5 c1f4 b8c6 e2e3 f6e4 c3e4 f5e4 f1b5 f7f6 b5c6 b7c6 e1g1 a8b8 a1b1 h7h5 d1e2 e8f7 f4g3 e7e6 f1e1 f8d6 g3d6 d8d6 e1d1 c6c5 d4c5 d6c5 f3d4 e4g6 e2a6 c5d6 a6a7 b8a8 a7b7 h8b8 b7c6 e6e5 c6d6 c7d6 d4c6 b8b6 d1d5 f7e6 d5a5 a8h8 c6a7 g6c2 b1e1 b6b2 a7c6 h8c8 a5a6 c2d3 a6a3 d3e4 c6a5 b2e2".split(' ').map(|s| s.to_string()).collect();

    let mut engine = PyMinimax::new(2, 2, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.go();

    // Ensure there is at least one move
    // assert!(!moves.contains(ChessMove::), "There should be at least one legal move for White");
}


#[test]
fn real_test() {
    let moves: Vec<String> = "g1f3 d7d5 b1c3 d5d4 c3b5 c7c5 e2e4 a7a6 b5a3 b7b5 c2c4 e7e6 c4b5 c8b7 b5a6 a8a6 f1a6 b8a6 d1a4 d8d7 a4d7 e8d7 f3e5 d7e8 d2d3 f7f6 e5c4 a6b4 e1e2 f6f5 c4a5 b7c8 a5c4 g8f6 e4e5 f6h5 a3b5 b4c2 b5d6 e8d7 a1b1 f8d6 c4d6 c2b4 a2a3 b4d5 g2g3 c8a6 c1g5 h8b8 h1d1 d7c6 e2f1 d5c3 b2c3 b8b1 d1b1 a6d3 f1g1 d4c3 b1b3 c5c4 b3c3 c6d5 c3c1 h5f6 e5f6 g7f6 g5f4 e6e5 d6f5 d3f5 f4e3 f5e6 c1c3 e6d7 c3c2 d7g4 c2c3 d5e6 c3c4 g4e2 c4c7 e6d6 c7h7 d6d5 h7g7 e5e4 g7e7 e2g4 g1g2 g4f3 g2g1 d5d6 e7f7 d6e6 f7h7 f6f5 h7g7 e6e5 g7g6 f3d1 e3f4 e5d5 g6d6 d5c4 d6d1 c4b3 f4d6 b3a4 d1d5 f5f4 g3f4 e4e3 f2e3 a4b3 f4f5 b3c3 f5f6 c3c4 f6f7 c4d5 f7f8q d5e4 f8f4 e4d3 e3e4 d3c4 f4f5 c4b3 e4e5 b3a2 e5e6 a2a1 e6e7 a1b2 e7e8q b2c3 e8e2 c3b3 g1g2 b3c3 g2h3 c3b3 h3h4 b3c3 h4h5 c3b3 h5h6 b3c3 h6h7 c3b3 h7h8 b3c3 h8g8 c3b3 g8h8 b3c3 h8g8 c3b3".split(' ').map(|s| s.to_string()).collect();


    let mut engine = PyMinimax::new(3, 4, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.go();

    // Ensure there is at least one move
    // assert!(!moves.contains(ChessMove::), "There should be at least one legal move for White");
}


#[test]
fn real_test2() {
    let moves: Vec<String> = "e2e4 g8f6 e4e5 f6d5 c2c4 d5b6 d2d4 b8c6 c1f4 d7d6 g1f3".split(' ').map(|s| s.to_string()).collect();


    let mut engine = PyMinimax::new(4, 8, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.go();

    // Ensure there is at least one move
    // assert!(!moves.contains(ChessMove::), "There should be at least one legal move for White");
}

#[test]
fn test_detects_threefold_repetition() {
    let moves: Vec<String> = "g1f3 d7d5 b1c3 d5d4 c3b5 c7c5 e2e4 c8g4 h2h3 g4f3 d1f3 b8c6 f1c4 g8h6 d2d3 e7e6 c1h6 g7h6 e1g1 a7a6 b5a3 f8d6 c4b3 b7b5 f1e1 c6e5 f3g3 e8d7 g3g7 d8g8 g7g8 a8g8 g1h1 e5c6 a3b1 a6a5 c2c4 c6b4 c4b5 b4d3 e1f1 g8b8 a2a4 h8g8 a1a2 d3b4 a2a3 b4d3 b3e6 d7e6 a3d3 h6h5 h1g1 d6e7 f1e1 h5h4 d3d1 g8g7 f2f4 e7d6 e4e5 d6c7 b1d2 e6f5 d1c1 b8g8 e1e2 g7g3 c1c5 d4d3 e5e6 f5f6 d2e4 f6g7 e4g3 d3e2 c5g5 g7f6 g3e2 g8g5 f4g5 f6e7 e6f7 c7d6 f7f8b e7f8 e2c3 d6g3 b5b6 f8f7 c3e4 g3e5 b6b7 f7g6 g1f2 g6h5 f2f1 e5f4 f1g1 f4c7 g1h1 h5g6 h1g1 g6f5 e4f6 f5g5 f6d7 h7h6 b7b8n c7f4 b8c6 f4d2 d7e5 g5f6 e5c4 d2f4 c6a5 f6f7 a5c6 f7g6 c6e5 f4e5 c4e5 g6f6 e5f3 f6e6 f3h4 e6d5 h4f5 d5e4 f5h6 e4e3 h6f5  e3e4 f5d6 e4d5 d6f5 d5c5 b2b3 c5b6 f5d4 b6c5 d4e6 c5b4 e6f4 b4b3 a4a5 b3c2 a5a6 c2d2 a6a7 d2e3 f4d5 e3d2 a7a8q d2d1 a8c8 d1d2 c8c7 d2d3 c7c8 d3e4 c8e6 e4d3 e6c8 d3d4 c8c6 d4e4 c6e6 e4d3".split(' ').map(|s| s.to_string()).collect();


    let mut engine = PyMinimax::new(4, 8, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.go();

    // Ensure there is at least one move
    // assert!(!moves.contains(ChessMove::), "There should be at least one legal move for White");
}


#[test]
fn test_false_beta_pruning() {
    // In this game, d3d2 is evaluated as the best move. It doesn't realize it is getting mated because the stand-pat evaluation in quiescence is inaccurate.
    // This causes improper beta-pruning. Can be fixed with better evaluation.
    let moves: Vec<String> = "d2d4 g8f6 c1g5 b8c6 c2c3 f6e4 g5h4 d7d5 f2f4 d8d6 e2e3 c8e6 f1d3 e8c8 g1f3 c8b8 b2b4 h8g8 b4b5 c6a5 d1a4 a5c4 d3c4 d5c4 f3g5 e4g5 f4g5 d6d5 e1f2 d5e4 h4g3 e4f5 f2g1 f5g5 g3f4 g5f6 h2h4 f6f5 b1d2 f5d3 b5b6 d3d2 a4a7 b8c8 b6c7".split(' ').map(|s| s.to_string()).collect();

    let mut engine = PyMinimax::new(2, 8, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.evaluate_moves();

    let bad_move_str = "d8d5";

    let found = moves.iter().any(|(s, _)| s == bad_move_str);
    assert!(found, "Move {} was not found in the list", bad_move_str);

    for (mv, eval) in moves.iter().take(100) {
        println!("{mv}: {eval}");
    }

    let actual_score = moves
        .iter()
        .find(|(s, _)| s == bad_move_str)
        .map(|(_, score)| *score)
        .unwrap();
    assert!(actual_score < -25000, "{} should be losing (mate in 2). Eval: {}", bad_move_str, actual_score);
}



#[test]
fn test_mate_trick() {
    // Check if engine captures a piece and blunders back rank mate
    let moves: Vec<String> = "e2e4 g8f6 b1c3 d7d5 e4e5 d5d4 e5f6 d4c3 f6e7 c3d2 c1d2 f8e7 g1f3 b8c6 f1c4 c8f5 e1g1 d8d7 d1e2 f5c2 d2c3 c2g6 c3g7 h8g8 g7f6 e8c8 f6e7 d7e7 e2e7 c6e7 f3e5 g6e4 f2f3 e4d5 f1c1 d5c4 c1c4 d8d2 g2g4 f7f6 e5f7 g8f8 f7h6 d2b2 c4e4 e7d5 a1d1 d5c3 d1e1 c3e4 e1e4 b2a2 e4e3 a2a1 g1g2 a1a2 g2g3 c8b8 h6f5 f8d8 f5d4".split(' ').map(|s| s.to_string()).collect();

    let mut engine = PyMinimax::new(2, 8, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.evaluate_moves();

    let bad_move_str = "d8d4";

    let found = moves.iter().any(|(s, _)| s == bad_move_str);
    assert!(found, "Move {} was not found in the list", bad_move_str);

    for (mv, eval) in moves.iter().take(100) {
        println!("{mv}: {eval}");
    }

    let actual_score = moves
        .iter()
        .find(|(s, _)| s == bad_move_str)
        .map(|(_, score)| *score)
        .unwrap();
    assert!(actual_score < -25000, "{} should be losing (mate in 2). Eval: {}", bad_move_str, actual_score);
}

#[test]
fn test_find_mate_in_one() {
    // Check if engine finds mate in 1 quickly
    let moves: Vec<String> = "g1f3 c7c5 d2d4 c5d4 f3d4 g7g6 b1c3 f8g7 e2e4 g8f6 e4e5 f6g8 c1f4 b8c6 d4c6 b7c6 f1c4 c8b7 e1g1 e7e6 c3e4 g8e7 e4d6 e8f8 d6b7 d8b6 d1d7 a8b8 b7d6 b6b2 f4g5 f7f6 d7e6 e7d5".split(' ').map(|s| s.to_string()).collect();

    let mut engine = PyMinimax::new(2, 5, true, true);

    engine.set_position(STARTPOS, moves);

    let moves = engine.evaluate_moves();

    let mate_str = "e6f7";

    let found = moves.iter().any(|(s, _)| s == mate_str);
    assert!(found, "Move {} was not found in the list", mate_str);

    for (mv, eval) in moves.iter().take(100) {
        println!("{mv}: {eval}");
    }

    let actual_score = moves
        .iter()
        .find(|(s, _)| s == mate_str)
        .map(|(_, score)| *score)
        .unwrap();
    assert!(actual_score > 25000, "{} should be winning (mate in 1). Eval: {}", mate_str, actual_score);
}