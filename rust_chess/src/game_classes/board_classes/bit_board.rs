use crate::enums::Colour;
use crate::coords::Coords;

#[derive(Debug, Clone)]
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

    pub fn set_bit(&mut self, coords: &Coords, empty: bool) {
        let index = (coords.rank - 1) * 8 + coords.file.value() as u8;
        self.bits = if !empty {
            self.bits | (1 << index)
        } else {
            self.bits & !(1 << index)
        };
    }

    pub fn is_set(&self, coords: &Coords) -> bool {
        let index = (coords.rank - 1) * 8 + coords.file.value() as u8;
        (self.bits & (1 << index)) != 0
    }
}

#[test]
fn test_bit_board_initialization() {
    let bit_board = BitBoard::new();

    assert!(bit_board.is_empty());
}

#[test]
fn test_bit_board_set_and_check() {
    let mut bit_board = BitBoard::new();
    let coords = Coords::new(1, crate::enums::File::A);

    bit_board.set_bit(&coords, false);
    assert!(bit_board.is_set(&coords));
    assert!(!bit_board.is_empty());
    bit_board.set_bit(&coords, true);
    assert!(!bit_board.is_set(&coords));
    assert!(bit_board.is_empty());
}