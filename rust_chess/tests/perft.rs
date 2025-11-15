use std::time::Instant;
use rust_chess::game_classes::game::Game;
use rust_chess::moves::move_generator::MoveGenerator;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

fn perft(game: &mut Game, depth: u32, magic_bitboard: bool) -> u64 {
    if depth == 0 {
        return 1;
    }

    let colour = game.get_game_state().get_turn();
    let mut moves = Vec::new();
    MoveGenerator::generate_legal_moves_into(game, colour, magic_bitboard, &mut moves);

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut count = 0;
    for mv in &moves {
        game.make_move(mv);
        count += perft(game, depth - 1, magic_bitboard);
        game.undo_last_move();
    }
    count
}

fn run_perft_startpos(depth: u32, expected_nodes: u64) {
    let mut game = Game::new();
    game.set_fenstr(STARTPOS);

    let start = Instant::now();
    let nodes = perft(&mut game, depth, true);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!(
        "Perft depth {}: {} nodes in {:.3?}, {:.2} nodes/sec",
        depth, nodes, duration, nps
    );
    assert_eq!(nodes, expected_nodes, "Perft node count mismatch at depth {}", depth);
}

fn run_perft_kiwipete(depth: u32, expected_nodes: u64) {
    let mut game = Game::new();
    game.set_fenstr(KIWIPETE);

    let start = Instant::now();
    let nodes = perft(&mut game, depth, true);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!(
        "Kiwipete perft depth {}: {} nodes in {:.3?}, {:.2} nodes/sec",
        depth, nodes, duration, nps
    );
    assert_eq!(nodes, expected_nodes, "Kiwipete perft node count mismatch at depth {}", depth);
}

#[test]
fn test_perft_startpos_depth1() {
    run_perft_startpos(1, 20); // Standard perft value for startpos depth 1
}

#[test]
fn test_perft_startpos_depth2() {
    run_perft_startpos(2, 400); // Standard perft value
}

#[test]
fn test_perft_startpos_depth3() {
    run_perft_startpos(3, 8902); // Standard perft value
}

#[test]
fn test_perft_startpos_depth4() {
    run_perft_startpos(4, 197281); // Standard perft value
}

#[test]
fn test_perft_kiwipete_depth1() {
    run_perft_kiwipete(1, 48);
}

#[test]
fn test_perft_kiwipete_depth2() {
    run_perft_kiwipete(2, 2039);
}

#[test]
fn test_perft_kiwipete_depth3() {
    run_perft_kiwipete(3, 97862);
}

#[test]
fn test_perft_kiwipete_depth4() {
    run_perft_kiwipete(4, 4085603);
}

#[test]
fn test_perft_magic_vs_no_magic() {
    let mut game = Game::new();
    game.set_fenstr(STARTPOS);

    let start = Instant::now();
    let nodes = perft(&mut game, 4, true);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!("Magic enabled: {} nodes in {:.3?}, {:.2} nps", nodes, duration, nps);
    assert_eq!(nodes, 197281); // Standard value

    let mut game_no_magic = Game::new();
    game_no_magic.set_fenstr(STARTPOS);
    let start_no_magic = Instant::now();
    let nodes_no_magic = perft(&mut game_no_magic, 4, false);
    let duration_no_magic = start_no_magic.elapsed();

    let nps_no_magic = nodes_no_magic as f64 / duration_no_magic.as_secs_f64();
    println!("Magic disabled: {} nodes in {:.3?}, {:.2} nps", nodes_no_magic, duration_no_magic, nps_no_magic);
    assert_eq!(nodes_no_magic, 197281); // Standard value
}