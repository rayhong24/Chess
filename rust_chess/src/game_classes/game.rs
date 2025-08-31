use crate::game_classes::board_classes::board::Board;
use crate::coords::Coords;
use crate::enums::{Colour, Piece, ChessMove};
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

    pub fn make_move(&mut self, chess_move: &ChessMove) {
        if let Some((board_piece, piece_colour)) = self.board.get_piece_at(&chess_move.from()) {
            if board_piece != chess_move.piece() || piece_colour != chess_move.colour() {
                panic!(
                    "Piece at {:?} is {:?} {:?}, but move is for {:?} {:?}",
                    chess_move.from(),
                    piece_colour,
                    board_piece,
                    chess_move.colour(),
                    chess_move.piece()
                );
            }
        } else {
            panic!("No piece at the source coordinates {:?}", chess_move.from());
        }

        self.game_state_history.push(self.game_state.clone());


        match chess_move {
            ChessMove::Normal(ref mv) => {
                self.board.move_piece(&mv.piece, &mv.colour, &mv.from, &mv.to);
                self.game_state.update(chess_move);

                // Update move history
                self.move_history.push(*chess_move);
            }
            // Handle other move types (Castling, EnPassant) here
            _ => unimplemented!("This move type is not yet implemented."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::moves::NormalMove;
    use crate::enums::{Colour, Piece, File, ChessMove};

    // Helper to create a normal move
    fn make_normal_move(colour: Colour, piece: Piece, from: Coords, to: Coords) -> ChessMove {
        ChessMove::Normal(NormalMove {
            colour,
            piece,
            from: from,
            to: to,
            captured_piece: None,
        })
    }

    #[test]
    fn test_normal_move_updates_board() {
        let mut game = Game::new();

        // Move white pawn from E2 → E4
        let mv = make_normal_move(Colour::White, Piece::Pawn, Coords::new(2, File::E), Coords::new(4, File::E));
        game.make_move(&mv);

        // Check the move history was updated
        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.move_history[0].from(), Coords::new(2, File::E));

        // Check the game state history was updated
        assert_eq!(game.game_state_history.len(), 1);

        // Check the board: pawn should be at E4, not at E2
        let piece_at_e4 = game.board.get_piece_at(&Coords::new(4, File::E));
        assert!(piece_at_e4.is_some());
        let (piece, colour) = piece_at_e4.unwrap();
        assert_eq!(piece, Piece::Pawn);
        assert_eq!(colour, Colour::White);

        let piece_at_e2 = game.board.get_piece_at(&Coords::new(2, File::E));
        assert!(piece_at_e2.is_none());
    }

    #[test]
    #[should_panic(expected = "No piece at the source coordinates")]
    fn test_move_from_empty_square_panics() {
        let mut game = Game::new();
        let mv = make_normal_move(Colour::White, Piece::Pawn, Coords::new(3, File::E), Coords::new(4, File::E));
        game.make_move(&mv); // should panic because E3 is empty
    }

    #[test]
    #[should_panic(expected = "Piece at")]
    fn test_move_wrong_piece_panics() {
        let mut game = Game::new();
        // Try to move a rook from E2 (actually has a pawn)
        let mv = make_normal_move(Colour::White, Piece::Rook, Coords::new(2, File::E), Coords::new(4, File::E));
        game.make_move(&mv); // should panic
    }
}

