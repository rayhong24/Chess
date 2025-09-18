use crate::enums::moves::{NormalMove, PromotionMove};
use crate::game_classes::board_classes::board::Board;
use crate::coords::Coords;
use crate::moves::move_generator::MoveGenerator;
use crate::piece::Piece;
use crate::enums::{Colour, PieceType, File, ChessMove, ExecutedMove};
use crate::game_classes::game_state::GameState;


pub enum GameResult {
    Checkmate(Colour),
    Stalemate
}

pub struct Game {
    board: Board,
    game_state: GameState,
    move_history: Vec<ExecutedMove>,
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

    pub fn clear_board(&mut self) {
        self.board = Board::new();
    }

    pub fn set_fenstr(&mut self, fenstr: &str) {
        let fenstr_parts: Vec<&str> = fenstr.split(' ').collect();
        if fenstr_parts.len() < 4 {
            panic!("Invalid FEN string: expected at least 4 fields, got {}", fenstr_parts.len());
        }

        // 1. Board setup
        self.board.set_board_from_fenstr(fenstr_parts[0]);

        // 2. Active colour
        self.game_state.set_turn(match fenstr_parts[1] {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => {panic!("Invalid active colour field in fenstring")}
        });

        // 3. Castling rights
        self.game_state.set_castling_rights_from_fenstr(fenstr_parts[2]);


        // 4. En passant target
        if fenstr_parts[3] != "-" {
            if let Some(coords) = Coords::from_str(fenstr_parts[3]) {
                self.game_state.set_en_passant_target(Some(coords));
            }
            else {
                panic!("Invalid Enpassant target");
            }
        }
        else {
            self.game_state.set_en_passant_target(None);
        }


        // Halfmove clock and fullmove number not yet implemented
        
    }

    pub fn is_game_over(&mut self) -> Option<GameResult> {
        let player = self.get_game_state().get_turn();
        let moves = MoveGenerator::generate_legal_moves(self, player);

        if moves.len() > 0 {
            return None;
        }


        if self.is_player_in_check(player) {
            return Some(GameResult::Checkmate((player)))
        }
        else {
            return Some(GameResult::Stalemate)
        }
        
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn set_turn(&mut self, colour: Colour) {
        self.game_state.set_turn(colour);
    }

    pub fn get_player_pieces(&self, player: Colour) -> Vec<(Piece, Coords)> {
        self.board.get_player_pieces(player)
    }

    pub fn make_move(&mut self, chess_move: &ChessMove) {
        self.game_state_history.push(self.game_state.clone());
        self.game_state.update(chess_move);


        match chess_move {
            ChessMove::Normal(ref mv) => {
                let executed_move = ExecutedMove::Normal { mv: *mv, captured_piece: self.board.get_coords(&mv.to)};
                self.move_history.push(executed_move);

                let piece = Piece { kind: mv.piece_type, colour: mv.colour};
                self.board.move_piece(&piece, &mv.from, &mv.to);

            }
            ChessMove::Castling(ref mv) => {
                let executed_move = ExecutedMove::Castling { mv: *mv };
                self.move_history.push(executed_move);

                let king = Piece { kind: PieceType::King, colour: mv.colour};
                let rook = Piece { kind: PieceType::Rook, colour: mv.colour};

                self.board.move_piece(&king, &mv.king_from, &mv.king_to);
                self.board.move_piece(&rook, &mv.rook_from, &mv.rook_to);
            }
            ChessMove::Promotion(ref mv) => {
                let executed_move = ExecutedMove::Promotion { mv: *mv , captured_piece: self.board.get_coords(&mv.to)};
                self.move_history.push(executed_move);

                let promotion_piece = Piece {kind: mv.promotion_piece_type, colour: mv.colour};
                self.board.set_coords(&mv.from, None);
                self.board.set_coords(&mv.to, Some(promotion_piece));
            }
            ChessMove::EnPassant(ref mv) => {
                let executed_move = ExecutedMove::EnPassant { mv:*mv };
                self.move_history.push(executed_move);

                let pawn = Piece { kind: PieceType::Pawn, colour: mv.colour };
                self.board.move_piece(&pawn, &mv.from, &mv.to);
                self.board.set_coords(&mv.captured_coords, None);
            }
            _ => unimplemented!("This move type is not yet implemented."),
        }
    }

    pub fn undo_last_move(&mut self) {
        if self.move_history.is_empty() || self.game_state_history.is_empty() {
            panic!("No move to undo.");
        }

        let executed_move = self.move_history.pop().unwrap();
        self.game_state = self.game_state_history.pop().unwrap();

        match executed_move {
            ExecutedMove::Normal {mv, captured_piece} => {
                let piece = Piece { kind: mv.piece_type, colour: mv.colour};

                self.board.move_piece(&piece, &mv.to, &mv.from);
                self.board.set_coords(&mv.to, captured_piece);
            }
            ExecutedMove::Castling {mv} => {
                let king = Piece { kind: PieceType::King, colour: mv.colour };
                let rook = Piece { kind: PieceType::Rook, colour: mv.colour };
                self.board.move_piece(&king, &mv.king_to, &mv.king_from);
                self.board.move_piece(&rook, &mv.rook_to, &mv.rook_from);
            }
            ExecutedMove::Promotion {mv, captured_piece} => {
                let pawn = Piece { kind: PieceType::Pawn, colour: mv.colour };

                self.board.set_coords(&mv.to, captured_piece);
                self.board.set_coords(&mv.from, Some(pawn));
            }
            ExecutedMove::EnPassant {mv} => {
                let pawn = Piece { kind: PieceType::Pawn, colour: mv.colour };
                self.board.move_piece(&pawn, &mv.to, &mv.from);

                let captured_pawn = Piece { kind: PieceType::Pawn, colour: mv.colour.other() };
                self.board.set_coords(&mv.captured_coords, Some(captured_pawn));
            }
            _ => unimplemented!("This move type is not yet implemented."),
        }
    }

    pub fn is_capture(&mut self, chess_move: &ChessMove) -> bool {
        self.board.get_coords(&chess_move.to()).is_some()
    }

    pub fn is_player_in_check(&self, player: Colour) -> bool {
        let player_king = Piece {kind: PieceType::King, colour: player };
        let player_king_coords = self.board.get_piece_coords(player_king);

        if player_king_coords.len() != 1 {
            panic!("Multiple king coords found: {:?}", player_king_coords);
        }

        let king_coords = player_king_coords[0];

        MoveGenerator::is_square_under_attack(self, &player.other(), &king_coords)
    }

    pub fn is_check(&mut self, chess_move: &ChessMove) -> bool {
        self.make_move(chess_move);

        let out = self.is_player_in_check(chess_move.colour().other()); 

        self.undo_last_move();


        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::moves::{NormalMove, CastlingMove, PromotionMove, EnPassantMove};
    use crate::enums::{Colour, PieceType, File, ChessMove};

    // Helper to create a normal move
    fn make_normal_move(colour: Colour, piece: PieceType, from: Coords, to: Coords) -> ChessMove {
        ChessMove::Normal(NormalMove {
            colour,
            piece_type: piece,
            from: from,
            to: to,
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

    #[test]
    fn test_castling_kingside_white() {
        let mut game = Game::new();
        game.clear_board(); // assumes you have a way to reset the board

        // Place only the king and rook in starting positions
        let white_king = Piece { kind: PieceType::King, colour: Colour::White };
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White };
        let king_start = Coords::new(1, File::E);
        let rook_start = Coords::new(1, File::H);

        game.board.set_coords(&king_start, Some(white_king));
        game.board.set_coords(&rook_start, Some(white_rook));

        let castling_move = ChessMove::Castling(CastlingMove {
            colour: Colour::White,
            king_from: king_start,
            king_to: Coords::new(1, File::G),
            rook_from: rook_start,
            rook_to: Coords::new(1, File::F),
        });

        game.make_move(&castling_move);

        // Assert king moved
        assert_eq!(
            game.board.get_coords(&Coords::new(1, File::G)),
            Some(white_king)
        );
        assert_eq!(game.board.get_coords(&Coords::new(1, File::E)), None);

        // Assert rook moved
        assert_eq!(
            game.board.get_coords(&Coords::new(1, File::F)),
            Some(white_rook)
        );
        assert_eq!(game.board.get_coords(&Coords::new(1, File::H)), None);
    }

    #[test]
    fn test_promotion_white_pawn() {
        let mut game = Game::new();
        game.clear_board();

        // Place a white pawn at rank 7
        let pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords::new(7, File::E);
        let to = Coords::new(8, File::E);
        game.board.set_coords(&from, Some(pawn));

        let promotion_move = ChessMove::Promotion(PromotionMove {
            colour: Colour::White,
            from,
            to,
            promotion_piece_type: PieceType::Queen,
        });

        game.make_move(&promotion_move);

        // Assert promoted piece is now on the board
        assert_eq!(
            game.board.get_coords(&to),
            Some(Piece { kind: PieceType::Queen, colour: Colour::White })
        );
        assert_eq!(game.board.get_coords(&from), None);
    }

    #[test]
    fn test_en_passant_move() {
        let mut game = Game::new();
        game.clear_board();

        // White pawn at e5
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords::new(5, File::E);
        game.board.set_coords(&from, Some(white_pawn));

        // Black pawn at d5
        let black_pawn = Piece { kind: PieceType::Pawn, colour: Colour::Black };
        let captured = Coords::new(5, File::D);
        game.board.set_coords(&captured, Some(black_pawn));

        let to = Coords::new(6, File::D);
        let mv = ChessMove::EnPassant(EnPassantMove {
            colour: Colour::White,
            from,
            to,
            captured_coords: captured,
        });

        game.make_move(&mv);

        assert_eq!(game.board.get_coords(&to), Some(white_pawn));
        assert_eq!(game.board.get_coords(&captured), None);
        assert_eq!(game.board.get_coords(&from), None);
    }

    #[test]
    fn test_undo_last_move() {
        let mut game = Game::new();
        game.clear_board();

        // Place two pawns manually
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let black_pawn = Piece { kind: PieceType::Pawn, colour: Colour::Black };

        let from = Coords::new(2, File::E); // e2
        let to = Coords::new(4, File::E);   // e4

        game.board.set_coords(&from, Some(white_pawn));
        game.board.set_coords(&Coords::new(7, File::E), Some(black_pawn));

        // Construct move (white pawn e2 -> e4)
        let mv = ChessMove::Normal(NormalMove {
            piece_type: PieceType::Pawn,
            colour: Colour::White,
            from,
            to,
        });

        let initial_state = game.board.clone();

        // Make the move
        game.make_move(&mv);

        assert!(game.board.get_coords(&from).is_none()); // pawn moved
        assert_eq!(game.board.get_coords(&to), Some(white_pawn));
        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.game_state_history.len(), 1);

        // Undo the move
        game.undo_last_move();

        // Board should match original state
        assert_eq!(game.board, initial_state);
        assert_eq!(game.move_history.len(), 0);
        assert_eq!(game.game_state_history.len(), 0);
    }

    #[test]
    fn test_undo_castling_move() {
        let mut game = Game::new();
        game.clear_board();

        // Place king and rook in castling positions
        let white_king = Piece { kind: PieceType::King, colour: Colour::White };
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White };

        let king_from = Coords::new(1, File::E);
        let king_to = Coords::new(1, File::G);
        let rook_from = Coords::new(1, File::H);
        let rook_to = Coords::new(1, File::F);

        game.board.set_coords(&king_from, Some(white_king));
        game.board.set_coords(&rook_from, Some(white_rook));

        let castling_move = ChessMove::Castling(CastlingMove {
            colour: Colour::White,
            king_from,
            king_to,
            rook_from,
            rook_to,
        });

        // Execute castling
        game.make_move(&castling_move);

        assert_eq!(game.board.get_coords(&king_to), Some(white_king));
        assert_eq!(game.board.get_coords(&rook_to), Some(white_rook));

        // Undo castling
        game.undo_last_move();

        assert_eq!(game.board.get_coords(&king_from), Some(white_king));
        assert_eq!(game.board.get_coords(&rook_from), Some(white_rook));
        assert_eq!(game.board.get_coords(&king_to), None);
        assert_eq!(game.board.get_coords(&rook_to), None);
    }

    #[test]
    fn test_undo_promotion_move() {
        let mut game = Game::new();
        game.clear_board();

        // Place white pawn ready to promote
        let from = Coords::new(7, File::A);
        let to = Coords::new(8, File::A);
        let pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        game.board.set_coords(&from, Some(pawn));

        let promotion_move = ChessMove::Promotion(PromotionMove {
            from,
            to,
            colour: Colour::White,
            promotion_piece_type: PieceType::Queen,
        });

        // Make the promotion
        game.make_move(&promotion_move);

        let promoted_piece = game.board.get_coords(&to).unwrap();
        assert_eq!(promoted_piece.kind, PieceType::Queen);
        assert_eq!(promoted_piece.colour, Colour::White);

        // Undo the promotion
        game.undo_last_move();

        let reverted_piece = game.board.get_coords(&from).unwrap();
        assert_eq!(reverted_piece.kind, PieceType::Pawn);
        assert_eq!(reverted_piece.colour, Colour::White);
        assert!(game.board.get_coords(&to).is_none());
    }

    #[test]
    fn test_undo_en_passant_move() {
        let mut game = Game::new();
        game.clear_board();

        // White pawn on e5
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords::new(5, File::E);
        game.board.set_coords(&from, Some(white_pawn));

        // Black pawn on d5 (target for en passant)
        let black_pawn = Piece { kind: PieceType::Pawn, colour: Colour::Black };
        let captured = Coords::new(5, File::D);
        game.board.set_coords(&captured, Some(black_pawn));

        // White plays e5 → d6 (en passant capture)
        let to = Coords::new(6, File::D);
        let en_passant_move = ChessMove::EnPassant(EnPassantMove {
            colour: Colour::White,
            from,
            to,
            captured_coords: captured,
        });

        // Execute en passant
        game.make_move(&en_passant_move);

        // White pawn should be on d6
        assert_eq!(game.board.get_coords(&to), Some(white_pawn));
        // Black pawn should be removed
        assert_eq!(game.board.get_coords(&captured), None);
        // e5 should now be empty
        assert!(game.board.get_coords(&from).is_none());

        // Undo the en passant
        game.undo_last_move();

        // White pawn should be back on e5
        assert_eq!(game.board.get_coords(&from), Some(white_pawn));
        // Black pawn should be restored on d5
        assert_eq!(game.board.get_coords(&captured), Some(black_pawn));
        // d6 should be empty again
        assert!(game.board.get_coords(&to).is_none());
    }
    #[test]
    fn test_is_capture_detects_capture() {
        let mut game = Game::new();
        game.clear_board();

        // Place a white pawn at e2
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords::new(2, File::E);
        game.board.set_coords(&from, Some(white_pawn));

        // Place a black pawn at e3 (target square)
        let black_pawn = Piece { kind: PieceType::Pawn, colour: Colour::Black };
        let to = Coords::new(3, File::E);
        game.board.set_coords(&to, Some(black_pawn));

        // Construct a move: white pawn e2 -> e3
        let mv = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from,
            to,
        });

        // Check if the move is considered a capture
        let is_capture = game.is_capture(&mv);

        assert!(is_capture, "Expected e2→e3 to be a capture because e3 is occupied");
    }

    #[test]
    fn test_is_capture_non_capture_move() {
        let mut game = Game::new();
        game.clear_board();

        // Place a white pawn at e2
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords::new(2, File::E);
        game.board.set_coords(&from, Some(white_pawn));

        // Target square is empty (e4)
        let to = Coords::new(4, File::E);

        // Construct a move: white pawn e2 -> e4
        let mv = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
            from,
            to,
        });

        // Check if the move is considered a capture
        let is_capture = game.is_capture(&mv);

        assert!(!is_capture, "Expected e2→e4 not to be a capture because e4 is empty");
    }

    #[test]
    fn test_is_check_detects_check() {
        let mut game = Game::new();
        game.clear_board();

        // Place black king on e8
        let black_king = Piece { kind: PieceType::King, colour: Colour::Black };
        let black_king_pos = Coords::new(8, File::E);
        game.board.set_coords(&black_king_pos, Some(black_king));

        // Place white queen on g2
        let white_queen = Piece { kind: PieceType::Queen, colour: Colour::White };
        let white_queen_pos = Coords::new(2, File::G);
        game.board.set_coords(&white_queen_pos, Some(white_queen));

        let white_queen_dest = Coords::new(8, File::G);

        // Construct move: white queen e1 -> e8 (puts black king in check)
        let mv = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Queen,
            from: white_queen_pos,
            to: white_queen_dest,
        });

        // Check if is_check detects check
        let is_check = game.is_check(&mv);
        assert!(is_check, "Expected move to put Black in check");
    }

    #[test]
    fn test_is_check_non_check_move() {
        let mut game = Game::new();
        game.clear_board();

        // Place black king on e8
        let black_king = Piece { kind: PieceType::King, colour: Colour::Black };
        let black_king_pos = Coords::new(8, File::E);
        game.board.set_coords(&black_king_pos, Some(black_king));

        // Place white queen somewhere not attacking king
        let white_queen = Piece { kind: PieceType::Queen, colour: Colour::White };
        let white_queen_pos = Coords::new(1, File::D);
        game.board.set_coords(&white_queen_pos, Some(white_queen));

        // Move queen to d2 (does not put king in check)
        let mv = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Queen,
            from: white_queen_pos,
            to: Coords::new(2, File::D),
        });

        let is_check = game.is_check(&mv);
        assert!(!is_check, "Expected move not to put Black in check");
    }

}

