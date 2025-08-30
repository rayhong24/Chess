use rust_chess::pieces::knight::Knight;
use rust_chess::pieces::Piece;
use rust_chess::enums::{Colour, File};
use rust_chess::coords::Coords;


#[test]fn test_knight_representation() {
    let white_knight = Knight::new(Colour::White);
    let black_knight = Knight::new(Colour::Black);  
    assert_eq!(white_knight.get_representation(), 'N');
    assert_eq!(black_knight.get_representation(), 'n');
}

#[test]
fn test_knight_moves_initial() {
    let knight = Knight::new(Colour::White);
    let coords = Coords::new(1, File::B);
    let moves = knight.get_move_rays(coords);

    // Knight should have 8 move rays (L-shaped moves)
    assert_eq!(moves.len(), 8);

    // Check each L-shaped move
    let move_positions = [
        (2, 1), (2, -1),
        (1, 2), (1, -2),
        (-1, 2), (-1, -2),
        (-2, 1), (-2, -1)
    ];

    for (rank_diff, file_diff) in &move_positions {
        let mv = moves.iter().find(|m| m.rank_diff == *rank_diff && m.file_diff == *file_diff).unwrap();
        assert_eq!(mv.dist, 1);
        assert!(mv.capture_allowed && !mv.capture_forced);
    }
}