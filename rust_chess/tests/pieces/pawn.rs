use rust_chess::pieces::pawn::Pawn;
use rust_chess::pieces::Piece;
use rust_chess::enums::{Colour, File};
use rust_chess::coords::Coords;

#[test]
fn test_pawn_representation() {
    let white_pawn = Pawn::new(Colour::White);
    let black_pawn = Pawn::new(Colour::Black);
    assert_eq!(white_pawn.get_representation(), 'P');
    assert_eq!(black_pawn.get_representation(), 'p');
}

#[test]
fn test_pawn_moves_white_initial() {
    let pawn = Pawn::new(Colour::White);
    let coords = Coords::new(2, File::E);
    let moves = pawn.get_destination_coords(coords);

    // Forward move should allow 2 squares
    let forward = moves.iter().find(|m| m.file_diff == 0).unwrap();
    assert_eq!(forward.rank_diff, 1);
    assert_eq!(forward.dist, 2);

    // Captures
    let left = moves.iter().find(|m| m.file_diff == -1).unwrap();
    let right = moves.iter().find(|m| m.file_diff == 1).unwrap();
    assert_eq!(left.rank_diff, 1);
    assert_eq!(right.rank_diff, 1);
    assert!(left.capture_allowed && left.capture_forced);
    assert!(right.capture_allowed && right.capture_forced);
}

#[test]
fn test_pawn_moves_black_initial() {
    let pawn = Pawn::new(Colour::Black);
    let coords = Coords::new(7, File::E);
    let moves = pawn.get_destination_coords(coords);

    // Forward move should allow 2 squares
    let forward = moves.iter().find(|m| m.file_diff == 0).unwrap();
    assert_eq!(forward.rank_diff, -1);
    assert_eq!(forward.dist, 2);

    // Captures
    let left = moves.iter().find(|m| m.file_diff == -1).unwrap();
    let right = moves.iter().find(|m| m.file_diff == 1).unwrap();
    assert_eq!(left.rank_diff, -1);
    assert_eq!(right.rank_diff, -1);
    assert!(left.capture_allowed && left.capture_forced);
    assert!(right.capture_allowed && right.capture_forced);
}

#[test]
fn test_pawn_moves_white_non_initial() {
    let pawn = Pawn::new(Colour::White);
    let coords = Coords::new(3, File::E);
    let moves = pawn.get_destination_coords(coords);

    let forward = moves.iter().find(|m| m.file_diff == 0).unwrap();
    assert_eq!(forward.rank_diff, 1);
    assert_eq!(forward.dist, 1);
}