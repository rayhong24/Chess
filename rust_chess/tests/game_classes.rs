use strum::IntoEnumIterator;

use rust_chess::game_classes::game::Game;
use rust_chess::coords::Coords;
use rust_chess::piece::Piece;
use rust_chess::enums::{Colour, PieceType, File, ChessMove};
use rust_chess::enums::moves::NormalMove;

fn normal_move(piece: PieceType, colour: Colour, from: Coords, to: Coords) -> ChessMove {
    ChessMove::Normal(NormalMove {
        piece_type: piece,
        colour,
        from: from,
        to: to,
    })
}

#[test]
fn test_initial_state() {
    let game = Game::new();

    // White pawns should be on rank 2
    for file in File::iter() {
        assert_eq!(
            game.get_board().get_coords(&Coords::new(2, file)),
            Some(Piece{kind: PieceType::Pawn, colour: Colour::White})
        );
    }

    // Check black king
    assert_eq!(
        game.get_board().get_coords(&Coords::new(8, File::E)),
        Some(Piece{kind: PieceType::King, colour: Colour::Black})
    );
}

#[test]
fn test_pawn_double_move_sets_en_passant() {
    let mut game = Game::new();

    let mut mv = normal_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
    game.make_move(&mut mv);

    assert_eq!(
        game.get_game_state().get_en_passant_target(),
        Some(Coords::new(3, File::E))
    );
}

#[test]
fn test_pawn_single_move_clears_en_passant() {
    let mut game = Game::new();

    // First, white pawn double move
    let mut mv1 = normal_move(PieceType::Pawn, Colour::White, Coords::new(2, File::E), Coords::new(4, File::E));
    game.make_move(&mut mv1);
    // Black pawn double move
    let mut mv2 = normal_move(PieceType::Pawn, Colour::Black, Coords::new(7, File::D), Coords::new(5, File::D));
    game.make_move(&mut mv2);
    // White pawn single move
    let mut mv3 = normal_move(PieceType::Pawn, Colour::White, Coords::new(4, File::E), Coords::new(5, File::E));
    game.make_move(&mut mv3);

    assert_eq!(game.get_game_state().get_en_passant_target(), None);
}

#[test]
fn test_castling_rights_revoked_after_king_move() {
    let mut game = Game::new();

    let mut mv = normal_move(PieceType::King, Colour::White, Coords::new(1, File::E), Coords::new(2, File::E));
    game.make_move(&mut mv);

    // Castling rights for white should be revoked
    assert!(!game.get_game_state().clone().can_castle_white_kingside());
    assert!(!game.get_game_state().clone().can_castle_white_queenside());
}

#[test]
#[should_panic]
fn test_illegal_move_wrong_turn_panics() {
    let mut game = Game::new();

    // White always starts, but try Black’s move
    let mut mv = normal_move(PieceType::Pawn, Colour::Black, Coords::new(7, File::E), Coords::new(5, File::E));
    game.make_move(&mut mv); // should panic
}
