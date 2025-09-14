use crate::game_classes::game::Game;
use crate::moves::move_generator::MoveGenerator;
use crate::enums::{ChessMove, Colour};

pub struct Minimax {
    pub max_depth: usize,
}

impl Minimax {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }

    pub fn find_best_move(&self, game: &mut Game, colour: Colour) -> Option<ChessMove> {
        let mut best_score = i32::MIN;
        let mut best_move: Option<ChessMove> = None;

        let moves = MoveGenerator::generate_legal_moves(game, colour);
        for mv in moves {
            game.make_move(&mv);

            let score = -self.minimax(game, self.max_depth - 1, -i32::MAX, i32::MAX, colour.other());

            game.undo_last_move();



            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        best_move
    }

    fn minimax(&self, game: &mut Game, depth: usize, mut alpha: i32, mut beta: i32, colour: Colour) -> i32 {
        if depth == 0 || game.is_game_over() {
            return self.evaluate(game, colour);
        }

        let moves = MoveGenerator::generate_legal_moves(game, colour);
        let mut best_score = i32::MIN;

        for mv in moves {
            game.make_move(&mv);
            let score = -self.minimax(game, depth - 1, -beta, -alpha, colour.other());
            game.undo_last_move();

            best_score = best_score.max(score);

            if best_score >= beta {
                return best_score;
            }
        }
        best_score
    }

    fn evaluate(&self, game: &Game, colour: Colour) -> i32 {
        game.get_board().get_material(colour) - game.get_board().get_material(colour.other())
    }
}