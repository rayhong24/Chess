use crate::{coords::Coords, moves::move_ray};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveRay {
    pub row_diff: i8,
    pub col_diff: i8,
    pub dist: u8,
    pub capture_allowed: bool,
    pub capture_forced: bool,
}

impl MoveRay {
    pub fn new(
        row_diff: i8,
        col_diff: i8,
        dist: u8,
        capture_allowed: bool,
        capture_forced: bool,
    ) -> Self {
        Self {
            row_diff,
            col_diff,
            dist,
            capture_allowed,
            capture_forced,
        }
    }

    pub fn generate_coords<'a>(&'a self, start_coords: &'a Coords) -> MoveCandidateCoordsIter<'a> {
        MoveCandidateCoordsIter {
            candidate: self,
            current: Some(start_coords.clone()),
            step: 0,
        }
    }
}

impl std::fmt::Display for MoveRay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.row_diff, self.col_diff, self.dist)
    }
}

// Iterator for generating coords
pub struct MoveCandidateCoordsIter<'a> {
    candidate: &'a MoveRay,
    current: Option<Coords>,
    step: u8,
}

impl<'a> Iterator for MoveCandidateCoordsIter<'a> {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step >= self.candidate.dist {
            return None;
        }
        let curr = self.current.as_ref()?;
        if !curr.diff_inbounds(self.candidate.row_diff, self.candidate.col_diff) {
            return None;
        }
        let next_coords = curr.get_neighbour(self.candidate.row_diff, self.candidate.col_diff)?;
        self.current = Some(next_coords.clone());
        self.step += 1;
        Some(next_coords)
    }
}