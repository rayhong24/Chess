use strum::IntoEnumIterator;

use crate::enums::moves::{EnPassantMove, NormalMove, PromotionMove};
use crate::enums::{ChessMove, PieceType, Colour};
use crate::game_classes::game::Game;
use crate::moves::move_ray::MoveRay;
use crate::piece::Piece;
use crate::coords::Coords;

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn generate_pseudo_legal_moves(game: &Game, player: Colour) -> Vec<ChessMove> {
        let mut moves = vec![];


        for (piece, coords) in &game.get_player_pieces(player) {
            moves.extend(
                Self::move_rays_to_chess_moves(
                    game,
                    piece,
                    coords,
                    &piece.get_move_rays(coords)
                )
            );
        }

        return moves;
    }

    fn move_rays_to_chess_moves(game: &Game, piece: &Piece, start_coords: &Coords, move_rays: &Vec<MoveRay>) -> Vec<ChessMove> {
        fn init_move(chess_moves: &mut Vec<ChessMove>, piece: &Piece, start_coords: &Coords, end_coords: &Coords) {
            // Promotion case
            if piece.kind == PieceType::Pawn && (end_coords.rank == 1 || end_coords.rank == 8) {
                for promotion_piece_type in PieceType::iter() {
                    if promotion_piece_type != PieceType::Pawn && promotion_piece_type != PieceType::King {
                        chess_moves.push(
                            ChessMove::Promotion(PromotionMove { 
                                colour: piece.colour,
                                from: *start_coords,
                                to: *end_coords,
                                promotion_piece_type
                            })
                        );
                    }
                }
            }
            // Normal case
            else {
                chess_moves.push(
                    ChessMove::Normal(NormalMove { 
                        colour: piece.colour,
                        piece_type: piece.kind,
                        from: *start_coords,
                        to: *end_coords
                    })
                );
            }
        }
        let mut chess_moves = Vec::new();

        for move_ray in move_rays {
            for end_coords in move_ray.generate_coords(start_coords) {
                if let Some(blocking_piece) = game.get_board().get_coords(&end_coords) {
                    if move_ray.capture_allowed && blocking_piece.colour != piece.colour {
                        init_move(&mut chess_moves, piece, start_coords, &end_coords);
                    }
                    break;
                }
                else {
                    if move_ray.capture_forced {
                        // En passant
                        if piece.kind == PieceType::Pawn && move_ray.capture_allowed && Some(end_coords) == game.get_game_state().get_en_passant_target() {
                            chess_moves.push(
                                ChessMove::EnPassant(EnPassantMove {
                                    colour: piece.colour,
                                    from: *start_coords,
                                    to: end_coords,
                                    captured_coords: game.get_game_state().get_en_passant_piece_coords().unwrap()
                                })
                            )
                        }
                        break;
                    }
                    init_move(&mut chess_moves, piece, start_coords, &end_coords);
                }
            }
        }

        chess_moves
    }

    fn is_square_under_attack(game: &Game, attacker: &Colour, coords: &Coords) -> bool {
        let moves = Self::generate_pseudo_legal_moves(game, *attacker);

        moves.iter().any(|m| m.to() == *coords)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]
    fn test_generate_pseudo_legal_moves_startposition() {
        let game = Game::new(); // sets up standard start position

        let moves = MoveGenerator::generate_pseudo_legal_moves(&game, game.get_game_state().get_turn());

        // At the start, white pawns can move 1 or 2 squares forward
        let mut pawn_moves = moves.iter().filter(|mv| {
            match mv {
                ChessMove::Normal(nm) => nm.piece_type == PieceType::Pawn && nm.colour == Colour::White,
                _ => false
            }
        });

        assert!(pawn_moves.clone().count() >= 16, "Expected at least 16 white pawn moves");
        assert!(moves.clone().len() == 20, "Expected 20 moves from starting position");
    }

    #[test]
    fn test_generate_pseudo_legal_moves_pawn_promotion() {
        let mut game = Game::new();

        // Place a white pawn at rank 7 for promotion
        let promotion_coords = Coords::new(7, crate::enums::File::A);
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        game.clear_board();
        game.get_board_mut().set_coords(&promotion_coords, Some(white_pawn));

        let moves = MoveGenerator::generate_pseudo_legal_moves(&game, game.get_game_state().get_turn());

        // Check for promotion moves
        let promotion_moves: Vec<_> = moves.into_iter().filter(|mv| {
            matches!(mv, ChessMove::Promotion(_))
        }).collect();

        assert!(!promotion_moves.is_empty(), "Expected at least one promotion move");
    }

    #[test]
    fn test_move_rays_blocked_by_piece() {
        let mut game = Game::new();

        // Put a white rook at d4
        let rook_coords = Coords::new(4, crate::enums::File::D);
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White };
        game.clear_board();
        game.get_board_mut().set_coords(&rook_coords, Some(white_rook));

        // Put a blocking white pawn at d5
        let blocking_coords = Coords::new(5, crate::enums::File::D);
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        game.get_board_mut().set_coords(&blocking_coords, Some(white_pawn));

        let moves = MoveGenerator::generate_pseudo_legal_moves(&game, game.get_game_state().get_turn());

        // The rook should not be able to move past the blocking pawn
        for mv in moves {
            if let ChessMove::Normal(nm) = mv {
                if nm.piece_type == PieceType::Rook && nm.from == rook_coords {
                    assert!(nm.to != blocking_coords, "Rook cannot move onto friendly piece");
                    assert!(nm.to.rank < 5, "Rook should be blocked by pawn at d5");
                }
            }
        }
    }

    #[test]
    fn test_en_passant_generation() {
        let mut game = Game::new();

        // Clear board and set up only the pawns needed
        game.clear_board();
        game.set_turn(Colour::Black);

        // Place white pawn on e5
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let white_pawn_coords = Coords::new(5, File::E);
        game.get_board_mut().set_coords(&white_pawn_coords, Some(white_pawn));

        // Place black pawn on d7
        let black_pawn = Piece { kind: PieceType::Pawn, colour: Colour::Black };
        let black_pawn_start = Coords::new(7, File::D);
        game.get_board_mut().set_coords(&black_pawn_start, Some(black_pawn));

        // Black moves pawn d7 -> d5
        let black_pawn_move = ChessMove::Normal(NormalMove {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
            from: black_pawn_start,
            to: Coords::new(5, File::D),
        });
        game.make_move(&black_pawn_move);

        // Now generate White moves
        let moves = MoveGenerator::generate_pseudo_legal_moves(&game, game.get_game_state().get_turn());

        // Expect en passant capture on d6
        let expected_en_passant = ChessMove::EnPassant(EnPassantMove {
            colour: Colour::White,
            from: white_pawn_coords,
            to: Coords::new(6, File::D),
            captured_coords: Coords::new(5, File::D),
        });

        println!("sdafasdfasdfasdfasdf");
        for m in &moves {
            println!("{:?}", m);
            println!()
        }

        assert!(
            moves.contains(&expected_en_passant),
            "Expected en passant move {:?}, but got {:?}",
            expected_en_passant,
            moves
        );
    }

    #[test]
    fn test_square_under_attack_by_rook() {
        let mut game = Game::new();
        game.clear_board();

        // Place a white rook on d4
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White };
        let rook_coords = Coords::new(4, File::D);
        game.get_board_mut().set_coords(&rook_coords, Some(white_rook));

        // Square d6 should be attacked
        let target = Coords::new(6, File::D);
        assert!(
            MoveGenerator::is_square_under_attack(&game, &Colour::White, &target),
            "Expected d6 to be attacked by rook on d4"
        );

        // Square e5 should NOT be attacked
        let not_attacked = Coords::new(5, File::E);
        assert!(
            !MoveGenerator::is_square_under_attack(&game, &Colour::White, &not_attacked),
            "Expected e5 not to be attacked by rook on d4"
        );
    }

    #[test]
    fn test_square_under_attack_by_knight() {
        let mut game = Game::new();
        game.clear_board();

        // Place a black knight on g5
        let black_knight = Piece { kind: PieceType::Knight, colour: Colour::Black };
        let knight_coords = Coords::new(5, File::G);
        game.get_board_mut().set_coords(&knight_coords, Some(black_knight));

        // Square e4 should be attacked (knight move)
        let target = Coords::new(4, File::E);
        assert!(
            MoveGenerator::is_square_under_attack(&game, &Colour::Black, &target),
            "Expected e4 to be attacked by knight on g5"
        );

        // Square g6 should NOT be attacked
        let not_attacked = Coords::new(6, File::G);
        assert!(
            !MoveGenerator::is_square_under_attack(&game, &Colour::Black, &not_attacked),
            "Expected g6 not to be attacked by knight on g5"
        );
    }
}