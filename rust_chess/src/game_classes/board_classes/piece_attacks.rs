use once_cell::sync::Lazy;

pub static KNIGHT_ATTACKS: Lazy<[u64; 64]> = Lazy::new(|| init_knight_attacks());
pub static KING_ATTACKS: Lazy<[u64; 64]> = Lazy::new(|| init_king_attacks());
pub static WHITE_PAWN_ATTACKS: Lazy<[u64; 64]> = Lazy::new(|| init_pawn_attacks(true));
pub static BLACK_PAWN_ATTACKS: Lazy<[u64; 64]> = Lazy::new(|| init_pawn_attacks(false));

fn init_knight_attacks() -> [u64; 64] {
    let mut attacks = [0u64; 64];
    for sq in 0..64 {
        let rank = (sq / 8) as i32;
        let file = (sq % 8) as i32;
        let mut moves = 0u64;
        let knight_moves = [
            (2, 1), (1, 2), (-1, 2), (-2, 1),
            (-2, -1), (-1, -2), (1, -2), (2, -1)
        ];

        for (dr, df) in knight_moves {
            let r = rank + dr;
            let f = file + df;
            if (0..8).contains(&r) && (0..8).contains(&f) {
                moves |= 1u64 << (r * 8 + f);
            }
        }
        attacks[sq] = moves;
    }
    attacks
}

fn init_king_attacks() -> [u64; 64] {
    let mut attacks = [0u64; 64];
    for sq in 0..64 {
        let rank = (sq / 8) as i32;
        let file = (sq % 8) as i32;
        let mut moves = 0u64;
        for dr in -1..=1 {
            for df in -1..=1 {
                if dr == 0 && df == 0 { continue; }
                let r = rank + dr;
                let f = file + df;
                if (0..8).contains(&r) && (0..8).contains(&f) {
                    moves |= 1u64 << (r * 8 + f);
                }
            }
        }
        attacks[sq] = moves;
    }
    attacks
}

fn init_pawn_attacks(white: bool) -> [u64; 64] {
    let mut attacks = [0u64; 64];
    for sq in 0..64 {
        let rank = (sq / 8) as i32;
        let file = (sq % 8) as i32;
        let mut moves = 0u64;
        let directions = if white { [(1, -1), (1, 1)] } else { [(-1, -1), (-1, 1)] };

        for (dr, df) in directions {
            let r = rank + dr;
            let f = file + df;
            if (0..8).contains(&r) && (0..8).contains(&f) {
                moves |= 1u64 << (r * 8 + f);
            }
        }
        attacks[sq] = moves;
    }
    attacks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bit(sq: usize) -> u64 {
        1u64 << sq
    }

    fn print_bb(bb: u64) {
        // Optional helper to visualize bitboards if needed
        for r in (0..8).rev() {
            for f in 0..8 {
                let sq = r * 8 + f;
                print!("{}", if (bb & bit(sq)) != 0 { "1 " } else { ". " });
            }
            println!();
        }
        println!();
    }

    #[test]
    fn test_knight_center_attacks() {
        // Knight on d4 (rank 4, file 4) = index 27
        let sq = 27;
        let attacks = KNIGHT_ATTACKS[sq];
        let expected_squares = [
            10, 12, 17, 21, 33, 37, 42, 44, // manually computed knight moves
        ];

        for &s in &expected_squares {
            assert!(attacks & bit(s) != 0, "Missing square {}", s);
        }
        // should not include its own square
        assert_eq!(attacks & bit(sq), 0);
    }

    #[test]
    fn test_knight_corner_attacks() {
        // Knight on a1 = index 0
        let attacks = KNIGHT_ATTACKS[0];
        let expected = bit(10) | bit(17); // b3 and c2
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_king_center_attacks() {
        // King on d4 (rank 4, file 4) = index 27
        let attacks = KING_ATTACKS[27];
        let expected_squares = [
            18, 19, 20,
            26,      28,
            34, 35, 36,
        ];

        for &s in &expected_squares {
            assert!(attacks & bit(s) != 0, "Missing square {}", s);
        }
        assert_eq!(attacks & bit(27), 0);
    }

    #[test]
    fn test_king_corner_attacks() {
        // King on a1
        let attacks = KING_ATTACKS[0];
        let expected = bit(1) | bit(8) | bit(9); // b1, a2, b2
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_white_pawn_attacks_from_start() {
        // White pawn on d2 (rank 2, file 4) = index 12
        let attacks = WHITE_PAWN_ATTACKS[12];
        let expected = bit(20 - 1) | bit(20 + 1); // c3, e3 → indices 18 and 20
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_black_pawn_attacks_from_start() {
        // Black pawn on d7 (rank 7, file 4) = index 52
        let attacks = BLACK_PAWN_ATTACKS[52];
        let expected = bit(43) | bit(45); // c6, e6
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_pawns_on_edge_files() {
        // White pawn on a2: can only attack b3
        let white_a2 = WHITE_PAWN_ATTACKS[8];
        assert_eq!(white_a2, bit(17));

        // Black pawn on h7: can only attack g6
        let black_h7 = BLACK_PAWN_ATTACKS[55];
        assert_eq!(black_h7, bit(46));
    }

    #[test]
    fn test_no_self_attack_for_all_pieces() {
        for sq in 0..64 {
            assert_eq!(KNIGHT_ATTACKS[sq] & bit(sq), 0);
            assert_eq!(KING_ATTACKS[sq] & bit(sq), 0);
            assert_eq!(WHITE_PAWN_ATTACKS[sq] & bit(sq), 0);
            assert_eq!(BLACK_PAWN_ATTACKS[sq] & bit(sq), 0);
        }
    }
}

