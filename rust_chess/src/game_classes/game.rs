use crate::game_classes::board_classes::board::Board;
use crate::coords::Coords;
use crate::piece::Piece;
use crate::enums::{Colour, PieceType, File, ChessMove};
use crate::game_classes::game_state::GameState;

pub struct Game {
    board: Board,
    game_state: GameState,
    move_history: Vec<ChessMove>,
    game_state_history: Vec<GameState>,
    ended: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::setup_startposition(),
            game_state: GameState::new(),
            move_history: Vec::new(),
            game_state_history: Vec::new(),
            ended: false,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn make_move(&mut self, chess_move: &ChessMove) {
        self.game_state_history.push(self.game_state.clone());
        self.game_state.update(chess_move);
        self.move_history.push(*chess_move);

        match chess_move {
            ChessMove::Normal(ref mv) => {
                self.board.move_piece(&mv.piece, &mv.colour, &mv.from, &mv.to);

            }
            // ChessMove::Castling(ref mv) => {
            //     self.board.move_piece(&PieceType::King, &mv.colour, &mv.king_from, &mv.king_to);
            //     self.board.move_piece(&PieceType::Rook, &mv.colour, &mv.rook_from, &mv.rook_to);
            // }
            // ChessMove::Promotion(ref mv) => {
            //     self.board.set_coords(&mv.from, None);
            //     self.board.set_coords(&mv.from, Some(mv.promoted_piece));
            // }
            _ => unimplemented!("This move type is not yet implemented."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::moves::{NormalMove, CastlingMove, PromotionMove};
    use crate::enums::{Colour, PieceType, File, ChessMove};

    // Helper to create a normal move
    fn make_normal_move(colour: Colour, piece: PieceType, from: Coords, to: Coords) -> ChessMove {
        ChessMove::Normal(NormalMove {
            colour,
            piece,
            from: from,
            to: to,
            captured_piece: None,
        })
    }

    #[test]
    #[should_panic()]
    fn test_move_from_empty_square_panics() {
        let mut game = Game::new();
        let mv = make_normal_move(Colour::White, PieceType::Pawn, Coords::new(3, File::E), Coords::new(4, File::E));
        game.make_move(&mv); // should panic because E3 is empty
    }

    #[test]
    #[should_panic()]
    fn test_move_wrong_piece_panics() {
        let mut game = Game::new();
        // Try to move a rook from E2 (actually has a pawn)
        let mv = make_normal_move(Colour::White, PieceType::Rook, Coords::new(2, File::E), Coords::new(4, File::E));
        game.make_move(&mv); // should panic
    }

    #[test]
    fn test_normal_move_updates_board() {
        let mut game = Game::new();

        // Move white pawn from E2 → E4
        let mv = make_normal_move(Colour::White, PieceType::Pawn, Coords::new(2, File::E), Coords::new(4, File::E));
        game.make_move(&mv);

        // Check the move history was updated
        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.move_history[0].from(), Coords::new(2, File::E));

        // Check the game state history was updated
        assert_eq!(game.game_state_history.len(), 1);

        // Check the board: pawn should be at E4, not at E2
        let piece_at_e4 = game.board.get_coords(&Coords::new(4, File::E));
        assert!(piece_at_e4.is_some());
        let piece = piece_at_e4.unwrap();
        assert_eq!(piece, Piece{kind: PieceType::Pawn, colour: Colour::White});

        let piece_at_e2 = game.board.get_coords(&Coords::new(2, File::E));
        assert!(piece_at_e2.is_none());
    }

    // #[test]
    // fn test_castling_kingside_white() {
    //     let mut game = Game::new();

    //     // Define castling move: King from E1 to G1, Rook from H1 to F1
    //     let castling_move = ChessMove::Castling(CastlingMove {
    //         colour: Colour::White,
    //         king_from: Coords::new(1, File::E),
    //         king_to: Coords::new(1, File::G),
    //         rook_from: Coords::new(1, File::H),
    //         rook_to: Coords::new(1, File::F),
    //     });

    //     game.make_move(&castling_move);

    //     // Assert king moved
    //     assert_eq!(
    //         game.board.get_coords(&Coords::new(1, File::G)),
    //         Some(Piece::King)
    //     );
    //     assert_eq!(game.board.get_coords(&Coords::new(1, File::E)), None);

    //     // Assert rook moved
    //     assert_eq!(
    //         game.board.get_coords(&Coords::new(1, File::F)),
    //         Some(Piece::Rook)
    //     );
    //     assert_eq!(game.board.get_coords(&Coords::new(1, File::H)), None);
    // }

    // #[test]
    // fn test_promotion_white_pawn() {
    //     let mut game = Game::new();

    //     // Define promotion move: Pawn promotes at E8 → Queen
    //     let promotion_move = ChessMove::Promotion(PromotionMove {
    //         colour: Colour::White,
    //         from: Coords::new(7, File::E), // pawn moves from 7th rank
    //         promoted_piece: Piece::Queen,
    //     });

    //     game.make_move(&promotion_move);

    //     // Assert square now has promoted piece
    //     assert_eq!(
    //         game.board.get_coords(&Coords::new(7, File::E)),
    //         Some(Piece::Queen)
    //     );
    // }
}

