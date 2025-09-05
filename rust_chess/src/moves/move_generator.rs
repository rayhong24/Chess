use crate::enums::moves::NormalMove;
use crate::enums::{ChessMove, PieceType, Colour};
use crate::game_classes::game::Game;
use crate::moves::move_ray::MoveRay;
use crate::piece::Piece;
use crate::coords::Coords;

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn generate_pseudo_legal_moves(game: &Game) -> Vec<ChessMove> {
        let mut moves = vec![];


        for (piece, coords) in &game.get_player_pieces() {
            println!("{:?}", piece.get_move_rays(coords));
        }

        return moves;
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random() {
        let game = Game::new();

        MoveGenerator::generate_pseudo_legal_moves(&game);
    }
}