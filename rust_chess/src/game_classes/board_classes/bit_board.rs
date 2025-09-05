use crate::coords::Coords;
use crate::enums::{File};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard {
    bits: u64,
}

impl BitBoard {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn set_bit(&mut self, coords: &Coords, filled: bool) {
        let index = (coords.rank - 1) * 8 + coords.file.value() as u8;
        self.bits = if filled {
            self.bits | (1 << index)
        } else {
            self.bits & !(1 << index)
        };
    }

    pub fn is_set(&self, coords: &Coords) -> bool {
        let index = (coords.rank - 1) * 8 + coords.file.value() as u8;
        (self.bits & (1 << index)) != 0
    }

    pub fn get_set_coords(&self) -> Vec<Coords> {
        let mut result = Vec::new();
        let mut bb = self.bits; // assuming BitBoard wraps a u64, e.g. struct BitBoard(u64);

        while bb != 0 {
            // Extract index of least significant set bit
            let idx = bb.trailing_zeros() as u8;

            // Convert idx (0..63) into rank + file
            let rank = (idx / 8) + 1; // ranks usually 1–8
            let file = match idx % 8 {
                0 => File::A,
                1 => File::B,
                2 => File::C,
                3 => File::D,
                4 => File::E,
                5 => File::F,
                6 => File::G,
                7 => File::H,
                _ => unreachable!(),
            };

            result.push(Coords::new(rank as u8, file));

            // Clear the LSB
            bb &= bb - 1;
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_board_initialization() {
        let bit_board = BitBoard::new();

        assert!(bit_board.is_empty());
    }

    #[test]
    fn test_bit_board_set_and_check() {
        let mut bit_board = BitBoard::new();
        let coords = Coords::new(1, crate::enums::File::A);

        bit_board.set_bit(&coords, true);
        assert!(bit_board.is_set(&coords));
        assert!(!bit_board.is_empty());
        bit_board.set_bit(&coords, false);
        assert!(!bit_board.is_set(&coords));
        assert!(bit_board.is_empty());
    }

    #[test]
    fn test_get_set_coords_empty() {
        let bb = BitBoard::new();
        let coords = bb.get_set_coords();
        assert!(coords.is_empty());
    }

    #[test]
    fn test_get_set_coords_single_bit() {
        // Bit 0 -> a1
        let bb = BitBoard::from_bits(1u64 << 0);
        let coords = bb.get_set_coords();
        assert_eq!(coords, vec![Coords::new(1, File::A)]);
    }

    #[test]
    fn test_get_set_coords_multiple_bits() {
        // Bit 0 -> a1, Bit 7 -> h1, Bit 63 -> h8
        let bb = BitBoard::from_bits((1u64 << 0) | (1u64 << 7) | (1u64 << 63));
        let mut coords = bb.get_set_coords();

        coords.sort_by_key(|c| (c.rank, c.file as u8)); // ensure deterministic order for assert

        let expected = vec![
            Coords::new(1, File::A),
            Coords::new(1, File::H),
            Coords::new(8, File::H),
        ];
        assert_eq!(coords, expected);
    }
}
