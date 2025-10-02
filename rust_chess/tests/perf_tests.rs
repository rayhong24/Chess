#[cfg(test)]
mod perf_tests {
    use super::*;
    use std::time::Instant;
    use rust_chess::{coords::Coords, enums::{Colour, File, PieceType}, game_classes::game::Game, moves::{move_generator::MoveGenerator, move_parser}, piece::Piece, PyMinimax};
    use strum::IntoEnumIterator;

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// Measure nodes per second for different depths
    #[test]
    fn test_minimax_nodes_per_second() {
        let mut depths = vec![1, 2, 3, 4];
        for depth in depths {
            let mut mini = PyMinimax::new(depth, 4, true);
            mini.set_position(STARTPOS, vec![]);

            mini.reset_minimax_nodes_and_tt_hits();
            let start = Instant::now();
            let _best_move = mini.go();
            let duration = start.elapsed();

            let nodes = mini.get_minimax_nodes() as f64;
            let secs = duration.as_secs_f64();
            let nps = nodes / secs;

            println!(
                "Depth {}: Nodes searched = {}, Duration = {:?}, Nodes/sec = {:.2}",
                depth, nodes, duration, nps
            );
        }
    }

    /// Compare TT vs No-TT performance
    #[test]
    fn test_transposition_table_speedup() {
        let mut no_tt = PyMinimax::new(3, 4, false);
        let mut tt = PyMinimax::new(3, 4, true);

        no_tt.set_position(STARTPOS, vec![]);
        tt.set_position(STARTPOS, vec![]);

        // Measure no-TT
        no_tt.reset_minimax_nodes_and_tt_hits();
        let start = Instant::now();
        let _ = no_tt.go();
        let duration_no_tt = start.elapsed();

        // Measure TT
        tt.reset_minimax_nodes_and_tt_hits();
        let start = Instant::now();
        let _ = tt.go();
        let duration_tt = start.elapsed();

        let nodes_no_tt = no_tt.get_minimax_nodes() as f64;
        let nodes_tt = tt.get_minimax_nodes() as f64;

        println!(
            "No-TT: Nodes = {}, Duration = {:?}, NPS = {:.2}",
            nodes_no_tt,
            duration_no_tt,
            nodes_no_tt / duration_no_tt.as_secs_f64()
        );
        println!(
            "TT: Nodes = {}, Duration = {:?}, NPS = {:.2}",
            nodes_tt,
            duration_tt,
            nodes_tt / duration_tt.as_secs_f64()
        );

        assert!(duration_tt <= duration_no_tt, "TT should be faster than no-TT");
    }

    #[test]
    fn test_move_generation_perf() {
        let mut game = Game::new();

        let start = Instant::now();
        let legal_moves = MoveGenerator::generate_legal_moves(&mut game, Colour::White);
        let duration = start.elapsed();

        println!(
            "Generated {} legal moves in {:?} ({:.2} moves/sec)",
            legal_moves.len(),
            duration,
            legal_moves.len() as f64 / duration.as_secs_f64()
        );
    }

    #[test]
    fn test_move_generation_perf_crowded() {
        let mut game = Game::new();

        game.set_fenstr("rnbqkb1r/pppp1ppp/8/4p3/3P4/2N1PN2/PPP2PPP/R1BQKB1R w KQkq - 0 1");

        let start = Instant::now();
        let legal_moves = MoveGenerator::generate_legal_moves(&mut game, Colour::White);
        let duration = start.elapsed();

        println!(
            "Generated {} legal moves in {:?} ({:.2} moves/sec)",
            legal_moves.len(),
            duration,
            legal_moves.len() as f64 / duration.as_secs_f64()
        );
    }
}
