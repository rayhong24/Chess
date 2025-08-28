use rust_chess::coords::Coords;
use rust_chess::enums::File;


#[test]
fn test_new_coords() {
    let c = Coords::new(4, File::E);
    assert_eq!(c.rank, 4);
    assert_eq!(c.file, File::E);
}

#[test]
fn test_from_str() {
    let c = Coords::from_str("e4").unwrap();
    assert_eq!(c.rank, 4);
    assert_eq!(c.file, File::E);
}

#[test]
fn test_diff_inbounds() {
    let c = Coords::new(4, File::E);
    assert!(c.diff_inbounds(1, 0)); // e5
    assert!(!c.diff_inbounds(5, 0)); // e9 (out of bounds)
}

#[test]
fn test_get_neighbour() {
    let c = Coords::new(4, File::E);
    let n = c.get_neighbour(1, 0).unwrap();
    assert_eq!(n.rank, 5);
    assert_eq!(n.file, File::E);

    let n2 = c.get_neighbour(5, 0);
    assert!(n2.is_none());
}

#[test]
fn test_display() {
    let c = Coords::new(4, File::E);
    assert_eq!(format!("{}", c), "e4");
}