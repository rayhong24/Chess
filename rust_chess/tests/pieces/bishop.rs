use rust_chess::pieces::bishop::Bishop;
use rust_chess::pieces::Piece;
use rust_chess::enums::{Colour, File};
use rust_chess::coords::Coords;

#[test]
fn test_bishop_representation() {
    let white_bishop = Bishop::new(Colour::White);
    let black_bishop = Bishop::new(Colour::Black);
    assert_eq!(white_bishop.get_representation(), 'B');
    assert_eq!(black_bishop.get_representation(), 'b');
}

#[test]
fn test_bishop_moves_initial() {
    let bishop = Bishop::new(Colour::White);
    let coords = Coords::new(1, File::C);
    let moves = bishop.get_destination_coords(coords);

    // Bishop should have 4 move rays (diagonal directions)
    assert_eq!(moves.len(), 4);

    // Check each diagonal direction
    let up_left = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == -1).unwrap();
    let up_right = moves.iter().find(|m| m.rank_diff == 1 && m.file_diff == 1).unwrap();
    let down_left = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == -1).unwrap();
    let down_right = moves.iter().find(|m| m.rank_diff == -1 && m.file_diff == 1).unwrap();

    assert_eq!(up_left.dist, 7);
    assert_eq!(up_right.dist, 7);
    assert_eq!(down_left.dist, 7);
    assert_eq!(down_right.dist, 7);

    assert!(up_left.capture_allowed && !up_left.capture_forced);
    assert!(up_right.capture_allowed && !up_right.capture_forced);
    assert!(down_left.capture_allowed && !down_left.capture_forced);
    assert!(down_right.capture_allowed && !down_right.capture_forced);
}