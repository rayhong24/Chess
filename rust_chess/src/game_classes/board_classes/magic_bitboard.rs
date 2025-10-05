use once_cell::sync::Lazy;
use crate::game_classes::board_classes::bit_board::BitBoard;
use std::mem::MaybeUninit;

pub static MAGIC_TABLES: Lazy<MagicTables> = Lazy::new(|| MagicTables::new());

pub struct MagicBitboard {
    pub mask: u64,             // Mask of relevant squares (ray squares)
    pub magic: u64,            // Magic multiplier for hashing
    pub shift: u8,             // Right-shift for index compression
    pub attacks: Vec<u64>,     // Precomputed attack bitboards
}

pub struct MagicTables {
    pub bishop_magics: [MagicBitboard; 64],
    pub rook_magics: [MagicBitboard; 64],
}

fn get_blocker_index(magic: &MagicBitboard, occ: u64) -> usize {
    let relevant_blockers = occ & magic.mask;
    ((relevant_blockers.wrapping_mul(magic.magic)) >> magic.shift) as usize
}


impl MagicBitboard {
    pub fn get_attacks(&self, occ_all: &BitBoard) -> BitBoard {
        let blockers = occ_all.bits() & self.mask;
        let index = ((blockers.wrapping_mul(self.magic)) >> self.shift) as usize;
        BitBoard::from_bits(self.attacks[index])
    }

}

impl MagicTables {
    pub fn new() -> Self {
        Self {
            bishop_magics: Self::init_magic_table(true),
            rook_magics: Self::init_magic_table(false),
        }
    }

    fn init_magic_table(is_bishop: bool) -> [MagicBitboard; 64] {
        let mut table: [MaybeUninit<MagicBitboard>; 64] = unsafe { MaybeUninit::uninit().assume_init() };

        for sq in 0..64 {
            let mask = if is_bishop {
                Self::bishop_mask(sq)
            } else {
                Self::rook_mask(sq)
            };

            let bits = mask.count_ones() as usize;
            let magic = Self::find_magic(sq, bits, is_bishop);
            let shift = 64 - bits as u8;
            let num_entries = 1 << bits;
            let mut attacks = vec![0u64; num_entries];

            for index in 0..num_entries {
                let blockers = Self::set_occupancy(index as u64, bits, mask);
                let attack = if is_bishop {
                    Self::bishop_attacks(sq, blockers)
                } else {
                    Self::rook_attacks(sq, blockers)
                };
                let idx = ((blockers.wrapping_mul(magic)) >> shift) as usize;
                attacks[idx] = attack;
            }

            table[sq] = MaybeUninit::new(MagicBitboard { mask, magic, shift, attacks });
        }

        unsafe { std::mem::transmute::<_, [MagicBitboard; 64]>(table) }
    }

    pub fn get_rook_attacks(&self, square: usize, occ: &BitBoard) -> BitBoard {
        self.rook_magics[square].get_attacks(occ)
    }

    pub fn get_bishop_attacks(&self, square: usize, occ: &BitBoard) -> BitBoard {
        self.bishop_magics[square].get_attacks(occ)
    }

    fn rook_mask(square: usize) -> u64 {
        let mut mask = 0u64;
        let rank = (square / 8) as i32;
        let file = (square % 8) as i32;

        for r in (rank + 1)..7 { mask |= 1u64 << (r * 8 + file); }
        for r in (1..rank).rev() { mask |= 1u64 << (r * 8 + file); }
        for f in (file + 1)..7 { mask |= 1u64 << (rank * 8 + f); }
        for f in (1..file).rev() { mask |= 1u64 << (rank * 8 + f); }

        mask
    }

    fn bishop_mask(square: usize) -> u64 {
        let mut mask = 0u64;
        let rank = (square / 8) as i32;
        let file = (square % 8) as i32;

        // NE
        let mut r = rank + 1;
        let mut f = file + 1;
        while r < 7 && f < 7 { mask |= 1u64 << (r * 8 + f); r += 1; f += 1; }

        // NW
        let mut r = rank + 1;
        let mut f = file.wrapping_sub(1);
        while r < 7 && f > 0 { mask |= 1u64 << (r * 8 + f); r += 1; f -= 1; }

        // SE
        let mut r = rank.wrapping_sub(1);
        let mut f = file + 1;
        while r > 0 && f < 7 { mask |= 1u64 << (r * 8 + f); r -= 1; f += 1; }

        // SW
        let mut r = rank.wrapping_sub(1);
        let mut f = file.wrapping_sub(1);
        while r > 0 && f > 0 { mask |= 1u64 << (r * 8 + f); r -= 1; f -= 1; }

        mask
    }

    fn set_occupancy(index: u64, bits: usize, mask: u64) -> u64 {
        let mut occupancy = 0u64;
        let mut bit_index = 0;
        let mut bits_left = mask;

        for i in 0..bits {
            let square = bits_left.trailing_zeros();
            bits_left &= bits_left - 1;
            if (index >> i) & 1 == 1 {
                occupancy |= 1u64 << square;
            }
        }
        occupancy
    }

    pub fn rook_attacks(square: usize, blockers: u64) -> u64 {
        let mut attacks = 0u64;
        let rank = (square / 8) as i32;
        let file = (square % 8) as i32;

        // North
        for r in (rank + 1)..8 {
            attacks |= 1u64 << (r * 8 + file);
            if blockers & (1u64 << (r * 8 + file)) != 0 { break; }
        }
        // South
        for r in (0..rank).rev() {
            attacks |= 1u64 << (r * 8 + file);
            if blockers & (1u64 << (r * 8 + file)) != 0 { break; }
        }
        // East
        for f in (file + 1)..8 {
            attacks |= 1u64 << (rank * 8 + f);
            if blockers & (1u64 << (rank * 8 + f)) != 0 { break; }
        }
        // West
        for f in (0..file).rev() {
            attacks |= 1u64 << (rank * 8 + f);
            if blockers & (1u64 << (rank * 8 + f)) != 0 { break; }
        }

        attacks
    }
    pub fn bishop_attacks(square: usize, blockers: u64) -> u64 {
        let mut attacks = 0u64;
        let rank = (square / 8) as i32;
        let file = (square % 8) as i32;

        // NE
        let mut r = rank + 1;
        let mut f = file + 1;
        while r < 8 && f < 8 {
            attacks |= 1u64 << (r * 8 + f);
            if blockers & (1u64 << (r * 8 + f)) != 0 { break; }
            r += 1; f += 1;
        }

        // NW
        let mut r = rank + 1;
        let mut f = file - 1;
        while r < 8 && f >= 0 {
            attacks |= 1u64 << (r * 8 + f);
            if blockers & (1u64 << (r * 8 + f)) != 0 { break; }
            r += 1;
            f = f.wrapping_sub(1);
        }

        // SE
        let mut r = rank - 1;
        let mut f = file + 1;
        while r >= 0 && f < 8 {
            attacks |= 1u64 << (r * 8 + f);
            if blockers & (1u64 << (r * 8 + f)) != 0 { break; }
            r = r.wrapping_sub(1);
            f += 1;
        }

        // SW
        let mut r = rank - 1;
        let mut f = file - 1;
        while r >= 0 && f >= 0 {
            attacks |= 1u64 << (r * 8 + f);
            if blockers & (1u64 << (r * 8 + f)) != 0 { break; }
            r = r.wrapping_sub(1);
            f = f.wrapping_sub(1);
        }

        attacks
    }

    fn find_magic(square: usize, relevant_bits: usize, bishop: bool) -> u64 {
        let mask = if bishop { MagicTables::bishop_mask(square) } else { MagicTables::rook_mask(square) };
        let occupancy_variations = 1 << relevant_bits;

        let mut blockers = vec![0u64; occupancy_variations];
        let mut attacks = vec![0u64; occupancy_variations];
        let mut used = vec![0u64; occupancy_variations];

        for i in 0..occupancy_variations {
            blockers[i] = MagicTables::set_occupancy(i as u64, relevant_bits, mask);
            attacks[i] = if bishop {
                MagicTables::bishop_attacks(square, blockers[i])
            } else {
                MagicTables::rook_attacks(square, blockers[i])
            };
        }

        // Try random magics until one works
        loop {
            let magic = MagicTables::random_sparse_u64();
            if ((mask.wrapping_mul(magic)) & 0xFF00000000000000).count_ones() < 6 {
                continue; // skip poor magics
            }

            used.fill(0);
            let mut fail = false;

            for i in 0..occupancy_variations {
                let index = ((blockers[i].wrapping_mul(magic)) >> (64 - relevant_bits)) as usize;
                if used[index] == 0 {
                    used[index] = attacks[i];
                } else if used[index] != attacks[i] {
                    fail = true;
                    break;
                }
            }

            if !fail {
                return magic; // success
            }
        }
    }

    fn random_sparse_u64() -> u64 {
        // Keeps only a few bits from random_u64() to make it sparse
        rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_classes::board_classes::bit_board::BitBoard;
    use crate::coords::Coords;
    use crate::enums::file::File;

    #[test]
    fn test_rook_mask_edges() {
        let a1_mask = MagicTables::rook_mask(0); // bottom-left corner
        let h8_mask = MagicTables::rook_mask(63); // top-right corner

        // A1: mask should exclude edge squares (0,7 and 0,0)
        assert_eq!(a1_mask & 1, 0); // square itself not included
        assert!(a1_mask & (1u64 << 8) != 0); // rank 1 north
        assert!(a1_mask & (1u64 << 1) != 0); // file a east

        // H8
        assert_eq!(h8_mask & (1u64 << 63), 0);
        assert!(h8_mask & (1u64 << 55) != 0); // south
        assert!(h8_mask & (1u64 << 62) != 0); // west
    }

    #[test]
    fn test_bishop_mask_edges() {
        let d4_mask = MagicTables::bishop_mask(27); // d4
        let c5_mask = MagicTables::bishop_mask(34); // c5

        // Just make sure some known diagonal squares are included
        assert!(d4_mask & (1u64 << 36) != 0); // e5 NE
        assert!(d4_mask & (1u64 << 18) != 0); // c3 SW
        assert!(c5_mask & (1u64 << 20) != 0); // d2 SE
    }

    #[test]
    fn test_rook_attacks_simple() {
        let blockers = 0u64;
        let attacks = MagicTables::rook_attacks(0, blockers); // rook at a1

        // Should include the first rank and first file
        assert!(attacks & (1u64 << 1) != 0); // a2 north
        assert!(attacks & (1u64 << 8) != 0); // b1 east
    }

    #[test]
    fn test_bishop_attacks_simple() {
        let blockers = 0u64;
        let attacks = MagicTables::bishop_attacks(0, blockers); // bishop at a1

        // Should include NE diagonal
        assert!(attacks & (1u64 << 9) != 0); // b2
        assert!(attacks & (1u64 << 18) != 0); // c3
    }

    #[test]
    fn test_get_attacks_with_blockers() {
        let table = MagicTables::new();
        let mut occ = BitBoard::new();

        let b2 = Coords {rank: 2, file: File::B};
        // Place a blocker at b2 for bishop at a1
        occ.set_bit(&b2, true); // b2
        let attacks = table.bishop_magics[0].get_attacks(&occ);

        let c3 = Coords { rank: 3, file: File::C};
        // Should include b2 but not c3
        assert!(attacks.is_set(&b2));
        assert!(!attacks.is_set(&c3));
    }

    #[test]
    fn test_rook_with_blockers_on_rank() {
        let table = MagicTables::new();
        let square = 0; // a1
        let mut occ = BitBoard::new();

        // Place blockers on a3 and c1
        let a3 = Coords { rank: 3, file: File::A };
        let c1 = Coords { rank: 1, file: File::C };
        occ.set_bit(&a3, true);
        occ.set_bit(&c1, true);

        let attacks = table.rook_magics[square].get_attacks(&occ);

        // Should include squares up to blockers
        assert!(attacks.is_set(&a3));
        assert!(!attacks.is_set(&Coords { rank: 4, file: File::A }));
        assert!(attacks.is_set(&c1));
        assert!(!attacks.is_set(&Coords { rank: 1, file: File::D }));
    }

    #[test]
    fn test_bishop_with_corner_blockers() {
        let table = MagicTables::new();
        let square = 0; // a1
        let mut occ = BitBoard::new();

        let b2 = Coords { rank: 2, file: File::B };
        let c3 = Coords { rank: 3, file: File::C };
        occ.set_bit(&b2, true);
        occ.set_bit(&c3, true);

        let attacks = table.bishop_magics[square].get_attacks(&occ);

        assert!(attacks.is_set(&b2));
        assert!(!attacks.is_set(&c3));
    }

    #[test]
    fn test_rook_full_file() {
        let table = MagicTables::new();
        let square = 8; // a2
        let mut occ = BitBoard::new();

        // Block every square above a2
        for r in 3..8 {
            occ.set_bit(&Coords { rank: r, file: File::A }, true);
        }

        let attacks = table.rook_magics[square].get_attacks(&occ);

        // Should stop at first blocker
        assert!(attacks.is_set(&Coords { rank: 3, file: File::A }));
        assert!(!attacks.is_set(&Coords { rank: 4, file: File::A }));
    }

    #[test]
    fn test_bishop_no_blockers_full_diagonal() {
        let table = MagicTables::new();
        let square = 18; // c3
        let occ = BitBoard::new(); // no blockers

        let attacks = table.bishop_magics[square].get_attacks(&occ);

        // Ensure diagonal squares are included
        let expected_coords = [
            Coords { rank: 1, file: File::A }, Coords { rank: 2, file: File::B },
            Coords { rank: 3, file: File::C }, Coords { rank: 4, file: File::D },
            Coords { rank: 5, file: File::E }, Coords { rank: 6, file: File::F },
            Coords { rank: 7, file: File::G }, Coords { rank: 8, file: File::H },
        ];

        for c in expected_coords.iter() {
            if c.rank != 3 || c.file != File::C { // exclude own square
                assert!(attacks.is_set(c));
            }
        }
    }

    #[test]
    fn test_rook_and_bishop_consistency_random() {
        let table = MagicTables::new();

        for sq in 0..64 {
            let occ_bits = rand::random::<u64>();
            let occ = BitBoard::from_bits(occ_bits);

            let rook_attacks = table.rook_magics[sq].get_attacks(&occ);
            let manual_rook = MagicTables::rook_attacks(sq, occ_bits);
            assert_eq!(rook_attacks.bits(), manual_rook);

            let bishop_attacks = table.bishop_magics[sq].get_attacks(&occ);
            let manual_bishop = MagicTables::bishop_attacks(sq, occ_bits);
            assert_eq!(bishop_attacks.bits(), manual_bishop);
        }
    }
}
