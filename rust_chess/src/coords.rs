use crate::enums::File;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn diff_inbounds(&self, di: i8, dj: i8) -> bool {
        let new_rank = self.rank as i8 + di;
        let new_file = self.file.value() as i8 + dj;
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
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file.name(), self.rank)
    }
}