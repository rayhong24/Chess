use std::time::Instant;
use rust_chess::PyMinimax;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn test_perft_startpos_depth1() {
    let mut engine = PyMinimax::new(0, 4, false, true); // Depth not used, disable TT for pure gen
    engine.set_position(STARTPOS, vec![]);

    let start = Instant::now();
    let nodes = engine.perft(1);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!("Perft depth 1: {} nodes in {:.3?}, {:.2} nodes/sec", nodes, duration, nps);
    assert_eq!(nodes, 20); // Standard perft value for startpos depth 1
}

#[test]
fn test_perft_startpos_depth2() {
    let mut engine = PyMinimax::new(0, 4, false, true);
    engine.set_position(STARTPOS, vec![]);

    let start = Instant::now();
    let nodes = engine.perft(2);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!("Perft depth 2: {} nodes in {:.3?}, {:.2} nodes/sec", nodes, duration, nps);
    assert_eq!(nodes, 400); // Standard perft value
}

#[test]
fn test_perft_startpos_depth3() {
    let mut engine = PyMinimax::new(0, 4, false, true);
    engine.set_position(STARTPOS, vec![]);

    let start = Instant::now();
    let nodes = engine.perft(3);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!("Perft depth 3: {} nodes in {:.3?}, {:.2} nodes/sec", nodes, duration, nps);
    assert_eq!(nodes, 8902); // Standard perft value
}

#[test]
fn test_perft_startpos_depth4() {
    let mut engine = PyMinimax::new(0, 4, false, true);
    engine.set_position(STARTPOS, vec![]);

    let start = Instant::now();
    let nodes = engine.perft(4);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!("Perft depth 4: {} nodes in {:.3?}, {:.2} nodes/sec", nodes, duration, nps);
    assert_eq!(nodes, 197281); // Standard perft value
}

#[test]
fn test_perft_magic_vs_no_magic() {
    let mut no_magic = PyMinimax::new(0, 4, false, false);
    let mut magic = PyMinimax::new(0, 4, false, true);

    no_magic.set_position(STARTPOS, vec![]);
    magic.set_position(STARTPOS, vec![]);

    let start = Instant::now();
    let nodes_no_magic = no_magic.perft(4);
    let duration_no_magic = start.elapsed();

    let start = Instant::now();
    let nodes_magic = magic.perft(4);
    let duration_magic = start.elapsed();

    let nps_no_magic = nodes_no_magic as f64 / duration_no_magic.as_secs_f64();
    let nps_magic = nodes_magic as f64 / duration_magic.as_secs_f64();

    println!("No Magic: {} nodes in {:.3?}, {:.2} nps", nodes_no_magic, duration_no_magic, nps_no_magic);
    println!("Magic: {} nodes in {:.3?}, {:.2} nps", nodes_magic, duration_magic, nps_magic);

    assert_eq!(nodes_no_magic, nodes_magic); // Should be same node count
    // Magic should be faster
    assert!(nps_magic > nps_no_magic, "Magic bitboards should be faster for move generation");
}