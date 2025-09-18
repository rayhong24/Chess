use crate::enums::{File, ChessMove, Colour, PieceType};
use crate::coords::Coords;
use crate::game_classes::game::Game;
use crate::enums::moves::{CastlingMove, NormalMove, PromotionMove, EnPassantMove};

pub struct MoveParser;

impl MoveParser {
    pub fn parse_str(move_str: &str, game: &Game) -> Option<ChessMove> {
        let mv = move_str.trim();

        if mv.len() < 4 {
            return None;
        }

        let from = Coords::from_str(&mv[0..2])?;
        let to = Coords::from_str(&mv[2..4])?;
        let colour = game.get_game_state().get_turn();
        let piece = game.get_board().get_coords(&from)?;

        // Castling
        if piece.kind == PieceType::King && (to.file - from.file).abs() == 2 {
            let (rook_from, rook_to) = if to.file.value() > from.file.value() {
                // Kingside
                match colour {
                    Colour::White => (Coords::new(1, File::H), Coords::new(1, File::F)),
                    Colour::Black => (Coords::new(8, File::H), Coords::new(8, File::F)),
                }
            } else {
                // Queenside
                match colour {
                    Colour::White => (Coords::new(1, File::A), Coords::new(1, File::D)),
                    Colour::Black => (Coords::new(8, File::A), Coords::new(8, File::D)),
                }
            };

            return Some(ChessMove::Castling(CastlingMove {
                colour,
                king_from: from,
                king_to: to,
                rook_from,
                rook_to,
            }));
        }

        // Promotion
        if mv.len() == 5 {
            let promoted_piece = match mv.chars().nth(4)? {
                'q' => PieceType::Queen,
                'r' => PieceType::Rook,
                'b' => PieceType::Bishop,
                'n' => PieceType::Knight,
                _ => return None,
            };

            return Some(ChessMove::Promotion(PromotionMove {
                colour: colour,
                from: from,
                to: to,
                promotion_piece_type: promoted_piece 
            }));
        }

        // En Passant
        if piece.kind == PieceType::Pawn && game.get_game_state().get_en_passant_target() == Some(to) {
            return Some(ChessMove::EnPassant(EnPassantMove {
                colour: colour,
                from: from,
                to: to,
                captured_coords: game.get_game_state().get_en_passant_piece_coords()?
            }));
        }

        // Normal move
        Some(ChessMove::Normal(NormalMove { 
            colour: colour, 
            piece_type: piece.kind,
            from: from,
            to: to 
        }))
    }

}

