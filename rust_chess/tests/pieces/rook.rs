use rust_chess::pieces::rook::Rook;
use rust_chess::pieces::Piece;
use rust_chess::enums::{Colour, File};
use rust_chess::coords::Coords;

#[test]
fn test_rook_representation() {
    let white_rook = Rook::new(Colour::White);
    let black_rook = Rook::new(Colour::Black);
    assert_eq!(white_rook.get_representation(), 'R');
    assert_eq!(black_rook.get_representation(), 'r');
}

#[test]
fn test_rook_moves_initial() {
    let rook = Rook::new(Colour::White);
    let coords = Coords::new(1, File::A);
    let moves = rook.get_destination_coords(coords);

    // Rook should have 4 move rays (up, down, left, right)
    assert_eq!(moves.len(), 4);

    // Check each direction
    let up = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == 0).unwrap();
    let down = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == 0).unwrap();
    let left = moves.iter().find(|m| m.rank_diff == 0 && m.file_diff == -1).unwrap();
    let right = moves.iter().find(|m| m.rank_diff == 0 && m.file_diff == 1).unwrap();

    assert_eq!(up.dist, 7);
    assert_eq!(down.dist, 7);
    assert_eq!(left.dist, 7);
    assert_eq!(right.dist, 7);

    assert!(up.capture_allowed && !up.capture_forced);
    assert!(down.capture_allowed && !down.capture_forced);
    assert!(left.capture_allowed && !left.capture_forced);
    assert!(right.capture_allowed && !right.capture_forced);
}