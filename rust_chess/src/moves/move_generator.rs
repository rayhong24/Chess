use strum::IntoEnumIterator;

use crate::enums::moves::{NormalMove, PromotionMove};
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
            moves.extend(
                Self::move_rays_to_chess_moves(
                    game,
                    piece,
                    coords,
                    &piece.get_move_rays(coords)
                )
            );
        }

        for m in &moves {
            println!("{:?}", m);
        }


        return moves;
    }

    fn move_rays_to_chess_moves(game: &Game, piece: &Piece, start_coords: &Coords, move_rays: &Vec<MoveRay>) -> Vec<ChessMove> {
        fn init_move(chess_moves: &mut Vec<ChessMove>, piece: &Piece, start_coords: &Coords, end_coords: &Coords) {
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
                        break;
                    }
                    init_move(&mut chess_moves, piece, start_coords, &end_coords);
                }
            }
        }

        chess_moves
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pseudo_legal_moves_startposition() {
        let game = Game::new(); // sets up standard start position

        let moves = MoveGenerator::generate_pseudo_legal_moves(&game);

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

        let moves = MoveGenerator::generate_pseudo_legal_moves(&game);

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

        let moves = MoveGenerator::generate_pseudo_legal_moves(&game);

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
}