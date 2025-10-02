use crate::enums::File;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Coords {
    pub rank: u8, // 1-based (1..=8)
    pub file: File,
}

impl Coords {
    pub fn new(rank: u8, file: File) -> Self {
        assert!((1..=8).contains(&rank));
        Self { rank, file }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 2 { return None; }
        let file_char = s.chars().next().unwrap();
        let rank_char = s.chars().nth(1).unwrap();
        let file = File::from_char(file_char)?;
        let rank = rank_char.to_digit(10)? as u8;
        Some(Self::new(rank, file))
    }

    pub fn diff_inbounds(&self, rank_diff: i8, file_diff: i8) -> bool {
        let new_rank = self.rank as i8 + rank_diff;
        let new_file = self.file.value() as i8 + file_diff;
        (1..=8).contains(&new_rank) && (0..=7).contains(&new_file)
    }

    pub fn get_neighbour(&self, rank_diff: i8, file_diff: i8) -> Option<Self> {
        if self.diff_inbounds(rank_diff, file_diff) {
            let new_rank = (self.rank as i8 + rank_diff) as u8;
            let new_file = (self.file.value() as i8 + file_diff) as u8;
            Some(Self::new(new_rank, File::from_usize(new_file as usize)?))
        } else {
            None
        }
    }

    pub fn to_index(&self) -> usize {
        ((self.rank - 1) as usize) * 8 + (self.file.value() as usize)
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file.name(), self.rank)
    }
}

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