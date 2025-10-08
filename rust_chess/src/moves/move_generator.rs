use strum::IntoEnumIterator;

use crate::game_classes::board_classes::magic_bitboard::{self, MAGIC_TABLES};
use crate::game_classes::board_classes::piece_attacks::{WHITE_PAWN_ATTACKS, BLACK_PAWN_ATTACKS, KNIGHT_ATTACKS, KING_ATTACKS};
use crate::enums::moves::{EnPassantMove, NormalMove, PromotionMove, CastlingMove};
use crate::enums::{ChessMove, PieceType, Colour, File};
use crate::game_classes::game::Game;
use crate::game_classes::game_state::CastlingRights;
use crate::moves::move_ray::MoveRay;
use crate::piece::Piece;
use crate::coords::Coords;

const MAX_MOVES: usize = 218;
const MAX_CASTLING_MOVES: usize = 4;

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn init() {
        let _ = &*MAGIC_TABLES;
        let _ = &*WHITE_PAWN_ATTACKS;
        let _ = &*BLACK_PAWN_ATTACKS;
        let _ = &*KNIGHT_ATTACKS;
        let _ = &*KING_ATTACKS;
    }

    /// Generate legal moves into a preallocated buffer
    pub fn generate_legal_moves_into(
        game: &mut Game,
        player: Colour,
        magic_bitboard: bool,
        out_moves: &mut Vec<ChessMove>
    ) {
        out_moves.clear();

        // generate pseudo-legal moves
        if magic_bitboard {
            Self::generate_pseudo_legal_moves_magic_bitboards_into(game, player, out_moves);
        } else {
            Self::generate_pseudo_legal_moves_into(game, player, out_moves);
        }

        // filter illegal moves in-place
        let mut i = 0;
        while i < out_moves.len() {
            if Self::does_leave_player_in_check(game, &out_moves[i], magic_bitboard) {
                out_moves.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // append castling moves
        Self::generate_castling_moves_into(game, player, magic_bitboard, out_moves);
    }

    pub fn generate_tactical_moves_into(
        game: &mut Game,
        to_move: Colour,
        use_magic: bool,
        out: &mut Vec<ChessMove>,
    ) {
        // Clear the buffer first
        out.clear();

        // Generate all pseudo-legal moves first
        let mut pseudo_moves = Vec::new();
        if use_magic {
            MoveGenerator::generate_pseudo_legal_moves_magic_bitboards_into(game, to_move, &mut pseudo_moves);
        } else {
            MoveGenerator::generate_pseudo_legal_moves_into(game, to_move, &mut pseudo_moves);
        };

        // Filter tactical moves: captures, promotions, en passant, checks
        for mv in pseudo_moves {
            let is_tactical = Self::is_tactical_move(game, &mv, use_magic);

            if is_tactical {
                // Optionally, skip moves that leave the king in check
                if !MoveGenerator::does_leave_player_in_check(game, &mv, use_magic) {
                    out.push(mv);
                }
            }
        }
    }

    pub fn is_tactical_move(game: &mut Game, mv: &ChessMove, use_magic: bool) -> bool {
        let is_tactical = match mv {
            ChessMove::Normal(nm) => {
                // Captures
                game.get_board().get_coords(&nm.to).is_some()
            }
            ChessMove::Promotion(_)
            | ChessMove::EnPassant(_)
            | ChessMove::Castling(_) => true,
        } || game.is_check(mv, use_magic);

        // Only include moves that don't leave player in check
        is_tactical
    }
    fn generate_pseudo_legal_moves_magic_bitboards_into(
        game: &Game,
        player: Colour,
        out_moves: &mut Vec<ChessMove>
    ) {
        out_moves.clear();
        let all_occ = game.get_board().all_occ();
        let own_occ = game.get_board().get_colour_occ(player);

        for (piece, coords) in game.get_player_pieces(player) {
            let sq = coords.to_index();

            if piece.kind == PieceType::Pawn {
                Self::generate_pawn_moves_into(game, player, coords, out_moves);
                continue;
            }

            let attacks = match piece.kind {
                PieceType::Knight => KNIGHT_ATTACKS[sq],
                PieceType::Bishop => MAGIC_TABLES.get_bishop_attacks(sq, &all_occ).bits(),
                PieceType::Rook => MAGIC_TABLES.get_rook_attacks(sq, &all_occ).bits(),
                PieceType::Queen => {
                    MAGIC_TABLES.get_bishop_attacks(sq, &all_occ).bits()
                        | MAGIC_TABLES.get_rook_attacks(sq, &all_occ).bits()
                }
                PieceType::King => KING_ATTACKS[sq],
                _ => 0,
            };

            let legal_targets = attacks & !own_occ.bits();
            let mut targets = legal_targets;

            while targets != 0 {
                let to_sq = targets.trailing_zeros() as usize;
                targets &= targets - 1;

                let from = coords;
                let to = Coords::from_index(to_sq);

                out_moves.push(ChessMove::Normal(NormalMove {
                    colour: player,
                    piece_type: piece.kind,
                    from,
                    to,
                }));
            }
        }
    }

    fn generate_pseudo_legal_moves_into(
        game: &Game,
        player: Colour,
        out_moves: &mut Vec<ChessMove>
    ) {
        out_moves.clear();
        for (piece, coords) in &game.get_player_pieces(player) {
            for mv in Self::move_rays_to_chess_moves_into(game, piece, coords) {
                out_moves.push(mv);
            }
        }
    }

    fn move_rays_to_chess_moves_into(game: &Game, piece: &Piece, start_coords: &Coords) -> Vec<ChessMove> {
        let mut out_moves = Vec::with_capacity(MAX_MOVES);
        for move_ray in piece.get_move_rays(start_coords) {
            for end_coords in move_ray.generate_coords(start_coords) {
                if let Some(blocking_piece) = game.get_board().get_coords(&end_coords) {
                    if move_ray.capture_allowed && blocking_piece.colour != piece.colour {
                        Self::init_move(&mut out_moves, piece, start_coords, &end_coords);
                    }
                    break;
                } else {
                    if move_ray.capture_forced {
                        if piece.kind == PieceType::Pawn && move_ray.capture_allowed &&
                            Some(end_coords) == game.get_game_state().get_en_passant_target()
                        {
                            out_moves.push(ChessMove::EnPassant(EnPassantMove {
                                colour: piece.colour,
                                from: *start_coords,
                                to: end_coords,
                                captured_coords: game.get_game_state().get_en_passant_piece_coords().unwrap()
                            }));
                        }
                        break;
                    }
                    Self::init_move(&mut out_moves, piece, start_coords, &end_coords);
                }
            }
        }
        out_moves
    }

    fn init_move(out_moves: &mut Vec<ChessMove>, piece: &Piece, start_coords: &Coords, end_coords: &Coords) {
        if piece.kind == PieceType::Pawn && (end_coords.rank == 1 || end_coords.rank == 8) {
            for promotion_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                out_moves.push(ChessMove::Promotion(PromotionMove {
                    colour: piece.colour,
                    from: *start_coords,
                    to: *end_coords,
                    promotion_piece_type: promotion_type,
                }));
            }
        } else {
            out_moves.push(ChessMove::Normal(NormalMove {
                colour: piece.colour,
                piece_type: piece.kind,
                from: *start_coords,
                to: *end_coords,
            }));
        }
    }

    fn generate_pawn_moves_into(game: &Game, player: Colour, from: Coords, out_moves: &mut Vec<ChessMove>) {
        let all_occ = game.get_board().all_occ();
        let opp_occ = game.get_board().get_colour_occ(player.other());
        let from_sq = from.to_index();
        let rank = from.rank;

        let (forward_dir, start_rank, promotion_rank) = match player {
            Colour::White => (8i32, 2, 8),
            Colour::Black => (-8i32, 7, 1),
        };

        let forward_sq = (from_sq as i32 + forward_dir) as usize;
        if forward_sq < 64 {
            let to = Coords::from_index(forward_sq);
            if !all_occ.is_set(&to) {
                if to.rank == promotion_rank {
                    for promotion_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        out_moves.push(ChessMove::Promotion(PromotionMove {
                            colour: player,
                            from,
                            to,
                            promotion_piece_type: promotion_type,
                        }));
                    }
                } else {
                    out_moves.push(ChessMove::Normal(NormalMove {
                        colour: player,
                        piece_type: PieceType::Pawn,
                        from,
                        to,
                    }));

                    if rank == start_rank {
                        let double_sq = (from_sq as i32 + 2 * forward_dir) as usize;
                        let double_to = Coords::from_index(double_sq);
                        if !all_occ.is_set(&double_to) {
                            out_moves.push(ChessMove::Normal(NormalMove {
                                colour: player,
                                piece_type: PieceType::Pawn,
                                from,
                                to: double_to,
                            }));
                        }
                    }
                }
            }
        }

        let capture_dirs = match player {
            Colour::White => [7, 9],
            Colour::Black => [-7, -9],
        };

        for &dir in &capture_dirs {
            let target_sq = (from_sq as i32 + dir) as usize;
            if target_sq < 64 {
                let to = Coords::from_index(target_sq);
                if (from.file as i32 - to.file as i32).abs() > 1 { continue; }

                if opp_occ.is_set(&to) {
                    if to.rank == promotion_rank {
                        for promotion_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                            out_moves.push(ChessMove::Promotion(PromotionMove {
                                colour: player,
                                from,
                                to,
                                promotion_piece_type: promotion_type,
                            }));
                        }
                    } else {
                        out_moves.push(ChessMove::Normal(NormalMove {
                            colour: player,
                            piece_type: PieceType::Pawn,
                            from,
                            to,
                        }));
                    }
                }

                if let Some(ep_square) = game.get_game_state().get_en_passant_target() {
                    if target_sq == ep_square.to_index() {
                        out_moves.push(ChessMove::EnPassant(EnPassantMove {
                            colour: player,
                            from,
                            to: ep_square,
                            captured_coords: game.get_game_state().get_en_passant_piece_coords().unwrap(),
                        }));
                    }
                }
            }
        }
    }

    fn does_leave_player_in_check(game: &mut Game, chess_move: &ChessMove, magic_bitboard: bool) -> bool {
        game.make_move(chess_move);
        let out = game.is_player_in_check(chess_move.colour(), magic_bitboard);
        game.undo_last_move();
        out
    }

    fn generate_castling_moves_into(game: &mut Game, colour: Colour, magic_bitboard: bool, out_moves: &mut Vec<ChessMove>) {
        let rank = if colour == Colour::White { 1 } else { 8 };
        let king_start = Coords::new(rank, File::E);
        let king = Piece { kind: PieceType::King, colour };

        if let Some(piece) = game.get_board().get_coords(&king_start) {
            if piece != king { return; }
        } else { return; }

        if Self::can_castle_kingside(game, colour, &king_start, magic_bitboard) {
            out_moves.push(ChessMove::Castling(CastlingMove {
                colour,
                king_from: king_start,
                king_to: Coords::new(rank, File::G),
                rook_from: Coords::new(rank, File::H),
                rook_to: Coords::new(rank, File::F),
            }));
        }

        if Self::can_castle_queenside(game, colour, &king_start, magic_bitboard) {
            out_moves.push(ChessMove::Castling(CastlingMove {
                colour,
                king_from: king_start,
                king_to: Coords::new(rank, File::C),
                rook_from: Coords::new(rank, File::A),
                rook_to: Coords::new(rank, File::D),
            }));
        }
    }

    fn can_castle_kingside(game: &mut Game, colour: Colour, king_start: &Coords, magic_bitboard: bool) -> bool {
        let rank = king_start.rank;
        let rights_ok = match colour {
            Colour::White => game.get_game_state().can_castle(CastlingRights::WHITE_KINGSIDE),
            Colour::Black => game.get_game_state().can_castle(CastlingRights::BLACK_KINGSIDE),
        };
        if !rights_ok { return false; }

        let rook_coords = Coords::new(rank, File::H);
        let rook = Piece { kind: PieceType::Rook, colour };
        if game.get_board().get_coords(&rook_coords) != Some(rook) { return false; }

        if game.get_board().get_coords(&Coords::new(rank, File::F)).is_some() ||
           game.get_board().get_coords(&Coords::new(rank, File::G)).is_some() { return false; }

        if Self::is_square_under_attack(game, &colour.other(), king_start, magic_bitboard) ||
           Self::is_square_under_attack(game, &colour.other(), &Coords::new(rank, File::F), magic_bitboard) ||
           Self::is_square_under_attack(game, &colour.other(), &Coords::new(rank, File::G), magic_bitboard) { return false; }

        true
    }

    fn can_castle_queenside(game: &mut Game, colour: Colour, king_start: &Coords, magic_bitboard: bool) -> bool {
        let rank = king_start.rank;
        let rights_ok = match colour {
            Colour::White => game.get_game_state().can_castle(CastlingRights::WHITE_QUEENSIDE),
            Colour::Black => game.get_game_state().can_castle(CastlingRights::BLACK_QUEENSIDE),
        };
        if !rights_ok { return false; }

        let rook_coords = Coords::new(rank, File::A);
        let rook = Piece { kind: PieceType::Rook, colour };
        if game.get_board().get_coords(&rook_coords) != Some(rook) { return false; }

        if game.get_board().get_coords(&Coords::new(rank, File::B)).is_some() ||
           game.get_board().get_coords(&Coords::new(rank, File::C)).is_some() ||
           game.get_board().get_coords(&Coords::new(rank, File::D)).is_some() { return false; }

        if Self::is_square_under_attack(game, &colour.other(), king_start, magic_bitboard) ||
           Self::is_square_under_attack(game, &colour.other(), &Coords::new(rank, File::D), magic_bitboard) ||
           Self::is_square_under_attack(game, &colour.other(), &Coords::new(rank, File::C), magic_bitboard) { return false; }

        true
    }

    pub fn is_square_under_attack(game: &mut Game, attacker: &Colour, coords: &Coords, magic_bitboard: bool) -> bool {
        let mut temp_moves = Vec::with_capacity(MAX_MOVES);
        if magic_bitboard {
            Self::generate_pseudo_legal_moves_magic_bitboards_into(game, *attacker, &mut temp_moves);
        } else {
            Self::generate_pseudo_legal_moves_into(game, *attacker, &mut temp_moves);
        }
        temp_moves.iter().any(|m| m.to() == *coords)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]
    fn test_generate_pseudo_legal_moves_startposition() {
        let mut game = Game::new(); // sets up standard start position

        let to_move = game.get_game_state().get_turn();

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_into(&mut game, to_move, &mut moves);

        // At the start, white pawns can move 1 or 2 squares forward
        let pawn_moves = moves.iter().filter(|mv| {
            match mv {
                ChessMove::Normal(nm) => nm.piece_type == PieceType::Pawn && nm.colour == Colour::White,
                _ => false
            }
        });


        assert!(pawn_moves.clone().count() >= 16, "Expected at least 16 white pawn moves");
        assert!(moves.clone().len() == 20, "Expected 20 moves from starting position");
    }


    #[test]
    fn test_generate_pseudo_legal_moves_startposition_magic() {
        let mut game = Game::new(); // sets up standard start position

        let to_move = game.get_game_state().get_turn();

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_magic_bitboards_into(&mut game, to_move, &mut moves);

        // At the start, white pawns can move 1 or 2 squares forward
        let pawn_moves = moves.iter().filter(|mv| {
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

        let to_move = game.get_game_state().get_turn();

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_into(&mut game, to_move, &mut moves);

        // Check for promotion moves
        let promotion_moves: Vec<_> = moves.into_iter().filter(|mv| {
            matches!(mv, ChessMove::Promotion(_))
        }).collect();


        assert!(!promotion_moves.is_empty(), "Expected at least one promotion move");
    }

    #[test]
    fn test_generate_pseudo_legal_moves_pawn_promotion_magic() {
        let mut game = Game::new();

        // Place a white pawn at rank 7 for promotion
        let promotion_coords = Coords::new(7, crate::enums::File::A);
        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        game.clear_board();
        game.get_board_mut().set_coords(&promotion_coords, Some(white_pawn));

        let to_move = game.get_game_state().get_turn();

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_magic_bitboards_into(&mut game, to_move, &mut moves);

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

        let to_move = game.get_game_state().get_turn();

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_into(&mut game, to_move, &mut moves);

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
    fn test_move_blocked_by_piece_magic() {
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

        let to_move = game.get_game_state().get_turn();

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_magic_bitboards_into(&mut game, to_move, &mut moves);

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
        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_into(&game, game.get_game_state().get_turn(), &mut moves);

        // Expect en passant capture on d6
        let expected_en_passant = ChessMove::EnPassant(EnPassantMove {
            colour: Colour::White,
            from: white_pawn_coords,
            to: Coords::new(6, File::D),
            captured_coords: Coords::new(5, File::D),
        });


        assert!(
            moves.contains(&expected_en_passant),
            "Expected en passant move {:?}, but got {:?}",
            expected_en_passant,
            moves
        );
    }

    #[test]
    fn test_en_passant_generation_magic() {
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

        let to_move = game.get_game_state().get_turn();

        // Now generate White moves
        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_magic_bitboards_into(&mut game, to_move, &mut moves);

        // Expect en passant capture on d6
        let expected_en_passant = ChessMove::EnPassant(EnPassantMove {
            colour: Colour::White,
            from: white_pawn_coords,
            to: Coords::new(6, File::D),
            captured_coords: Coords::new(5, File::D),
        });

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
            MoveGenerator::is_square_under_attack(&mut game, &Colour::White, &target, false),
            "Expected d6 to be attacked by rook on d4"
        );

        // Square e5 should NOT be attacked
        let not_attacked = Coords::new(5, File::E);
        assert!(
            !MoveGenerator::is_square_under_attack(&mut game, &Colour::White, &not_attacked, false),
            "Expected e5 not to be attacked by rook on d4"
        );
    }

    #[test]
    fn test_square_under_attack_by_rook_magic() {
        let mut game = Game::new();
        game.clear_board();

        // Place a white rook on d4
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White };
        let rook_coords = Coords::new(4, File::D);
        game.get_board_mut().set_coords(&rook_coords, Some(white_rook));

        // Square d6 should be attacked
        let target = Coords::new(6, File::D);
        assert!(
            MoveGenerator::is_square_under_attack(&mut game, &Colour::White, &target, true),
            "Expected d6 to be attacked by rook on d4"
        );

        // Square e5 should NOT be attacked
        let not_attacked = Coords::new(5, File::E);
        assert!(
            !MoveGenerator::is_square_under_attack(&mut game, &Colour::White, &not_attacked, true),
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
            MoveGenerator::is_square_under_attack(&mut game, &Colour::Black, &target, false),
            "Expected e4 to be attacked by knight on g5"
        );

        // Square g6 should NOT be attacked
        let not_attacked = Coords::new(6, File::G);
        assert!(
            !MoveGenerator::is_square_under_attack(&mut game, &Colour::Black, &not_attacked, false),
            "Expected g6 not to be attacked by knight on g5"
        );
    }

    #[test]
    fn test_square_under_attack_by_knight_magic() {
        let mut game = Game::new();
        game.clear_board();

        // Place a black knight on g5
        let black_knight = Piece { kind: PieceType::Knight, colour: Colour::Black };
        let knight_coords = Coords::new(5, File::G);
        game.get_board_mut().set_coords(&knight_coords, Some(black_knight));

        // Square e4 should be attacked (knight move)
        let target = Coords::new(4, File::E);
        assert!(
            MoveGenerator::is_square_under_attack(&mut game, &Colour::Black, &target, true),
            "Expected e4 to be attacked by knight on g5"
        );

        // Square g6 should NOT be attacked
        let not_attacked = Coords::new(6, File::G);
        assert!(
            !MoveGenerator::is_square_under_attack(&mut game, &Colour::Black, &not_attacked, true),
            "Expected g6 not to be attacked by knight on g5"
        );
    }


    #[test]
    fn test_generate_castling_moves_white_kingside() {
        let mut game = Game::new();
        game.clear_board();

        // Place white king on e1
        let white_king = Piece { kind: PieceType::King, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::E),
            Some(white_king)
        );

        // Place white rook on h1
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::H),
            Some(white_rook)
        );

        let mut moves = Vec::new();
        MoveGenerator::generate_castling_moves_into(&mut game, Colour::White, false, &mut moves);

        // Expect a single kingside castling move
        let expected = ChessMove::Castling(CastlingMove {
            colour: Colour::White,
            king_from: Coords::new(1, File::E),
            king_to: Coords::new(1, File::G),
            rook_from: Coords::new(1, File::H),
            rook_to: Coords::new(1, File::F),
        });

        assert!(
            moves.contains(&expected),
            "Expected kingside castling move {:?}, got {:?}",
            expected,
            moves
        );
    }

    #[test]
    fn test_generate_castling_moves_white_kingside_magic() {
        let mut game = Game::new();
        game.clear_board();

        // Place white king on e1
        let white_king = Piece { kind: PieceType::King, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::E),
            Some(white_king)
        );

        // Place white rook on h1
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::H),
            Some(white_rook)
        );

        let mut moves = Vec::new();
        MoveGenerator::generate_castling_moves_into(&mut game, Colour::White, true, &mut moves);

        // Expect a single kingside castling move
        let expected = ChessMove::Castling(CastlingMove {
            colour: Colour::White,
            king_from: Coords::new(1, File::E),
            king_to: Coords::new(1, File::G),
            rook_from: Coords::new(1, File::H),
            rook_to: Coords::new(1, File::F),
        });

        assert!(
            moves.contains(&expected),
            "Expected kingside castling move {:?}, got {:?}",
            expected,
            moves
        );
    }

    #[test]
    fn test_generate_castling_moves_white_kingside_through_check() {
        let mut game = Game::new();
        game.clear_board();

        // Place white king on e1
        let white_king = Piece { kind: PieceType::King, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::E),
            Some(white_king)
        );

        // Place white rook on h1
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::H),
            Some(white_rook)
        );

        // Place black rook in f8
        let black_rook = Piece { kind: PieceType::Rook, colour: Colour::Black};
        game.get_board_mut().set_coords(
            &Coords::new(8, File::F),
            Some(black_rook)
        );


        let mut moves = Vec::new();
        MoveGenerator::generate_castling_moves_into(&mut game, Colour::White, false, &mut moves);

        assert!(
            moves.is_empty(),
            "Expected no castling moves. Got {:?}",
            moves
        );
    }

    #[test]
    fn test_generate_castling_moves_white_kingside_through_check_magic() {
        let mut game = Game::new();
        game.clear_board();

        // Place white king on e1
        let white_king = Piece { kind: PieceType::King, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::E),
            Some(white_king)
        );

        // Place white rook on h1
        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White};
        game.get_board_mut().set_coords(
            &Coords::new(1, File::H),
            Some(white_rook)
        );

        // Place black rook in f8
        let black_rook = Piece { kind: PieceType::Rook, colour: Colour::Black};
        game.get_board_mut().set_coords(
            &Coords::new(8, File::F),
            Some(black_rook)
        );


        let mut moves = Vec::new();
        MoveGenerator::generate_castling_moves_into(&mut game, Colour::White, true, &mut moves);

        assert!(
            moves.is_empty(),
            "Expected no castling moves. Got {:?}",
            moves
        );
    }

    fn setup_simple_game() -> Game {
        // Creates a minimal board with only a few pieces for testing.
        let mut game = Game::new(); // Assuming you have a constructor for an empty board
        game.clear_board();

        // Place white king on e1
        game.get_board_mut().set_coords(&Coords::new(1, File::E), Some(Piece { kind: PieceType::King, colour: Colour::White }));
        // Place black king on e8
        game.get_board_mut().set_coords(&Coords::new(8, File::E), Some(Piece { kind: PieceType::King, colour: Colour::Black }));
        game
    }

    #[test]
    fn test_generate_pseudo_legal_moves_pawn_push() {
        let mut game = setup_simple_game();
        // Add a white pawn on e2
        game.get_board_mut().set_coords(&Coords::new(2, File::E), Some(Piece { kind: PieceType::Pawn, colour: Colour::White }));

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_into(&game, Colour::White, &mut moves);

        assert!(moves.iter().any(|m| m.to() == Coords::new(3, File::E)),
            "Pawn should be able to move forward to e3");
    }

    #[test]
    fn test_generate_pseudo_legal_moves_pawn_push_magic() {
        let mut game = setup_simple_game();
        // Add a white pawn on e2
        game.get_board_mut().set_coords(&Coords::new(2, File::E), Some(Piece { kind: PieceType::Pawn, colour: Colour::White }));

        let mut moves = Vec::new();
        MoveGenerator::generate_pseudo_legal_moves_magic_bitboards_into(&game, Colour::White, &mut moves);

        assert!(moves.iter().any(|m| m.to() == Coords::new(3, File::E)),
            "Pawn should be able to move forward to e3");
    }


    #[test]
    fn test_does_leave_player_in_check_true() {
        let mut game = setup_simple_game();
        // Add a white rook on e2, black king already on e8
        game.get_board_mut().set_coords(&Coords::new(2, File::E), Some(Piece { kind: PieceType::Rook, colour: Colour::White }));
        // Add black rook on e7, pinning white king
        game.get_board_mut().set_coords(&Coords::new(7, File::E), Some(Piece { kind: PieceType::Rook, colour: Colour::Black }));

        // Try moving the white rook away, exposing king
        let chess_move = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Rook,
            from: Coords::new(2, File::E),
            to: Coords::new(2, File::F),
        });

        assert!(MoveGenerator::does_leave_player_in_check(&mut game, &chess_move, false),
            "Moving rook away should leave the white king in check");
    }

    #[test]
    fn test_does_leave_player_in_check_true_magic() {
        let mut game = setup_simple_game();
        // Add a white rook on e2, black king already on e8
        game.get_board_mut().set_coords(&Coords::new(2, File::E), Some(Piece { kind: PieceType::Rook, colour: Colour::White }));
        // Add black rook on e7, pinning white king
        game.get_board_mut().set_coords(&Coords::new(7, File::E), Some(Piece { kind: PieceType::Rook, colour: Colour::Black }));

        // Try moving the white rook away, exposing king
        let chess_move = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Rook,
            from: Coords::new(2, File::E),
            to: Coords::new(2, File::F),
        });

        assert!(MoveGenerator::does_leave_player_in_check(&mut game, &chess_move, false),
            "Moving rook away should leave the white king in check");
    }


    #[test]
    fn test_does_leave_player_in_check_false() {
        let mut game = setup_simple_game();
        // Add white rook on a1
        game.get_board_mut().set_coords(&Coords::new(1, File::A), Some(Piece { kind: PieceType::Rook, colour: Colour::White }));

        // Safe rook move
        let chess_move = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Rook,
            from: Coords::new(1, File::A),
            to: Coords::new(1, File::B),
        });

        assert!(!MoveGenerator::does_leave_player_in_check(&mut game, &chess_move, false),
            "Rook moving on a1->b1 should not expose king to check");
    }

    #[test]
    fn test_does_leave_player_in_check_false_magic() {
        let mut game = setup_simple_game();
        // Add white rook on a1
        game.get_board_mut().set_coords(&Coords::new(1, File::A), Some(Piece { kind: PieceType::Rook, colour: Colour::White }));

        // Safe rook move
        let chess_move = ChessMove::Normal(NormalMove {
            colour: Colour::White,
            piece_type: PieceType::Rook,
            from: Coords::new(1, File::A),
            to: Coords::new(1, File::B),
        });

        assert!(!MoveGenerator::does_leave_player_in_check(&mut game, &chess_move, true),
            "Rook moving on a1->b1 should not expose king to check");
    }

    #[test]
    fn test_generate_legal_moves_filters_illegal() {
        let mut game = setup_simple_game();
        // Place white pawn on f2 and black bishop on h4, directly checking king after pawn push
        game.get_board_mut().set_coords(&Coords::new(2, File::F), Some(Piece { kind: PieceType::Pawn, colour: Colour::White }));
        game.get_board_mut().set_coords(&Coords::new(4, File::H), Some(Piece { kind: PieceType::Bishop, colour: Colour::Black }));

        let mut legal_moves = Vec::new();
        MoveGenerator::generate_legal_moves_into(&mut game, Colour::White, false, &mut legal_moves);

        for mv in &legal_moves {
            println!("{}", mv);
        }

        assert!(
            !legal_moves.iter().any(|m| m.to() == Coords::new(3, File::F)),
            "Pawn move to f3 should be illegal because it leaves the king in check"
        );

        assert!(
            !legal_moves.iter().any(|m| m.to() == Coords::new(4, File::F)),
            "Pawn move to f4 should be illegal because it leaves the king in check"
        );
    }

    #[test]
    fn test_generate_legal_moves_filters_illegal_magic() {
        let mut game = setup_simple_game();
        // Place white pawn on f2 and black bishop on h4, directly checking king after pawn push
        game.get_board_mut().set_coords(&Coords::new(2, File::F), Some(Piece { kind: PieceType::Pawn, colour: Colour::White }));
        game.get_board_mut().set_coords(&Coords::new(4, File::H), Some(Piece { kind: PieceType::Bishop, colour: Colour::Black }));

        let mut legal_moves = Vec::new();
        MoveGenerator::generate_legal_moves_into(&mut game, Colour::White, true, &mut legal_moves);

        assert!(
            !legal_moves.iter().any(|m| m.to() == Coords::new(3, File::F)),
            "Pawn move to f3 should be illegal because it leaves the king in check"
        );

        assert!(
            !legal_moves.iter().any(|m| m.to() == Coords::new(4, File::F)),
            "Pawn move to f4 should be illegal because it leaves the king in check"
        );
    }

    #[test]
    fn test_blocked_pawn_push() {
        let mut game = setup_simple_game();

        // Place white pawn on e2 and black pawn on e3, directly checking king after pawn push
        game.get_board_mut().set_coords(&Coords::new(2, File::E), Some(Piece { kind: PieceType::Pawn, colour: Colour::White }));
        game.get_board_mut().set_coords(&Coords::new(3, File::E), Some(Piece { kind: PieceType::Pawn, colour: Colour::Black }));

        let mut legal_moves = Vec::new();
        MoveGenerator::generate_legal_moves_into(&mut game, Colour::White, false, &mut legal_moves);

        assert!(
            !legal_moves.iter().any(|m| m.to() == Coords::new(3, File::E)),
            "Pawn move to e3 should be illegal because it leaves the king in check"
        );
    }

    #[test]
    fn test_blocked_pawn_push_magic() {
        let mut game = setup_simple_game();

        // Place white pawn on e2 and black pawn on e3, directly checking king after pawn push
        game.get_board_mut().set_coords(&Coords::new(2, File::E), Some(Piece { kind: PieceType::Pawn, colour: Colour::White }));
        game.get_board_mut().set_coords(&Coords::new(3, File::E), Some(Piece { kind: PieceType::Pawn, colour: Colour::Black }));

        let mut legal_moves = Vec::new();
        MoveGenerator::generate_legal_moves_into(&mut game, Colour::White, true, &mut legal_moves);

        for mv in &legal_moves {
            println!("{}", mv);
        }
        assert!(
            !legal_moves.iter().any(|m| m.to() == Coords::new(3, File::E)),
            "Pawn move to e3 should be illegal because it leaves the king in check"
        );
    }


    #[test]
    fn test_magic_and_classic_move_generation_equivalence() {
        let mut game = Game::new();
        let mut magic_game = Game::new();

        let mut moves_classic = Vec::new();
        MoveGenerator::generate_legal_moves_into(&mut game, Colour::White, true, &mut moves_classic);

        let mut moves_magic = Vec::new();
        MoveGenerator::generate_legal_moves_into(&mut game, Colour::White, true, &mut moves_magic);

        assert_eq!(
            moves_classic.len(),
            moves_magic.len(),
            "Move counts should match between classic and magic bitboard generator"
        );
    }


}
