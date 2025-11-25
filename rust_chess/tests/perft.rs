use std::time::Instant;
use rust_chess::game_classes::game::Game;
use rust_chess::moves::move_generator::MoveGenerator;
use rust_chess::enums::ChessMove;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

fn perft(game: &mut Game, depth: u32, magic_bitboard: bool, moves: &mut Vec<ChessMove>) -> u64 {
    if depth == 0 {
        return 1;
    }

    let colour = game.get_game_state().get_turn();
    let start_moves_len = moves.len();
    MoveGenerator::generate_legal_moves_into(game, colour, magic_bitboard, moves);

    if depth == 1 {
        let out = moves.len() as u64 - start_moves_len as u64;
        moves.truncate(start_moves_len);
        return out;
    }

    let mut count = 0;

    while moves.len() > start_moves_len {
        let mv = moves.pop().unwrap();
        game.make_move(&mv);
        count += perft(game, depth - 1, magic_bitboard, moves);
        game.undo_last_move();
    }

    count
}

fn run_perft_startpos(depth: u32, expected_nodes: u64, magic_bitboard: bool) {
    let mut game = Game::new();
    game.set_fenstr(STARTPOS);
    let mut moves = Vec::with_capacity(2048);

    let start = Instant::now();
    let nodes = perft(&mut game, depth, magic_bitboard, &mut moves);
    let duration = start.elapsed();

    let nps = nodes as f64 / duration.as_secs_f64();
    println!(
        "Perft depth {}: {} nodes in {:.3?}, {:.2} nodes/sec",
        depth, nodes, duration, nps
    );
    assert_eq!(nodes, expected_nodes, "Perft node count mismatch at depth {}", depth);
}

fn run_perft_kiwipete(depth: u32, expected_nodes: u64, magic_bitboard: bool) {
    let mut game = Game::new();
    game.set_fenstr(KIWIPETE);
    let mut moves = Vec::with_capacity(2048);

    let start = Instant::now();
    let nodes = perft(&mut game, depth, magic_bitboard, &mut moves);
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
    run_perft_startpos(1, 20, true); // Standard perft value for startpos depth 1
}

#[test]
fn test_perft_startpos_depth2() {
    run_perft_startpos(2, 400, true); // Standard perft value
}

#[test]
fn test_perft_startpos_depth3() {
    run_perft_startpos(3, 8902, true); // Standard perft value
}

#[test]
fn test_perft_startpos_depth4() {
    run_perft_startpos(4, 197281, true); // Standard perft value
}

#[test]
fn test_perft_kiwipete_depth1() {
    run_perft_kiwipete(1, 48, true);
}

#[test]
fn test_perft_kiwipete_depth2() {
    run_perft_kiwipete(2, 2039, true);
}

#[test]
fn test_perft_kiwipete_depth3() {
    run_perft_kiwipete(3, 97862, true);
}

#[test]
fn test_perft_kiwipete_depth4() {
    run_perft_kiwipete(4, 4085603, true);
}

#[test]
fn test_perft_magic_vs_no_magic() {
    run_perft_startpos(4, 197281, true);
    run_perft_startpos(4, 197281, false);
}