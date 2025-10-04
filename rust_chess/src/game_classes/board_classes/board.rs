use strum::IntoEnumIterator;

use crate::enums::piece_types::PIECE_COUNT;
use crate::enums::{Colour, PieceType};
use crate::coords::Coords;
use crate::piece::Piece;
use crate::game_classes::board_classes::bit_board::BitBoard;
// use crate::coords::Coords;
// use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    white_bit_boards: [BitBoard; PIECE_COUNT],
    black_bit_boards: [BitBoard; PIECE_COUNT],

    white_occ: BitBoard,
    black_occ: BitBoard,
    all_occ: BitBoard
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_bit_boards: [BitBoard::new(); PIECE_COUNT],
            black_bit_boards: [BitBoard::new(); PIECE_COUNT],

            white_occ: BitBoard::new(),
            black_occ: BitBoard::new(),
            all_occ: BitBoard::new(),
        }   
    }

    pub fn recompute_occupancy(&mut self) {
        let mut white_bits = 0u64;
        let mut black_bits = 0u64;

        for bb in &self.white_bit_boards {
            white_bits |= bb.bits();
        }

        for bb in &self.black_bit_boards {
            black_bits |= bb.bits();
        }

        self.white_occ = BitBoard::from_bits(white_bits);
        self.black_occ = BitBoard::from_bits(black_bits);
        self.all_occ = BitBoard::from_bits(white_bits | black_bits);
    }

    pub fn setup_startposition() -> Self {
        let mut board = Self::new();

        // Setup white pieces
        board.white_bit_boards[PieceType::Rook as usize].set_bit(&Coords::new(1, crate::enums::File::A), true);
        board.white_bit_boards[PieceType::Knight as usize].set_bit(&Coords::new(1, crate::enums::File::B), true);
        board.white_bit_boards[PieceType::Bishop as usize].set_bit(&Coords::new(1, crate::enums::File::C), true);
        board.white_bit_boards[PieceType::Queen as usize].set_bit(&Coords::new(1, crate::enums::File::D), true);
        board.white_bit_boards[PieceType::King as usize].set_bit(&Coords::new(1, crate::enums::File::E), true);
        board.white_bit_boards[PieceType::Bishop as usize].set_bit(&Coords::new(1, crate::enums::File::F), true);
        board.white_bit_boards[PieceType::Knight as usize].set_bit(&Coords::new(1, crate::enums::File::G), true);
        board.white_bit_boards[PieceType::Rook as usize].set_bit(&Coords::new(1, crate::enums::File::H), true);
        for file in crate::enums::File::iter() {
            board.white_bit_boards[PieceType::Pawn as usize].set_bit(&Coords::new(2, file), true);
        }

        // Setup black pieces
        board.black_bit_boards[PieceType::Rook as usize].set_bit(&Coords::new(8, crate::enums::File::A), true);
        board.black_bit_boards[PieceType::Knight as usize].set_bit(&Coords::new(8, crate::enums::File::B), true);
        board.black_bit_boards[PieceType::Bishop as usize].set_bit(&Coords::new(8, crate::enums::File::C), true);
        board.black_bit_boards[PieceType::Queen as usize].set_bit(&Coords::new(8, crate::enums::File::D), true);
        board.black_bit_boards[PieceType::King as usize].set_bit(&Coords::new(8, crate::enums::File::E), true);
        board.black_bit_boards[PieceType::Bishop as usize].set_bit(&Coords::new(8, crate::enums::File::F), true);
        board.black_bit_boards[PieceType::Knight as usize].set_bit(&Coords::new(8, crate::enums::File::G), true);
        board.black_bit_boards[PieceType::Rook as usize].set_bit(&Coords::new(8, crate::enums::File::H), true);

        for file in crate::enums::File::iter() {
            board.black_bit_boards[PieceType::Pawn as usize].set_bit(&Coords::new(7, file), true);
        }

        board.recompute_occupancy();

        board
    }

    pub fn get_player_pieces(&self, colour: Colour) -> Vec<(Piece, Coords)> {
        let mut out = Vec::new();

        let bitboards = match colour {
            Colour::White => &self.white_bit_boards,
            Colour::Black => &self.black_bit_boards
        };

        for piece_type in PieceType::iter() {
            let piece = Piece { kind: piece_type, colour: colour };
            for set_coords in bitboards[piece_type as usize].get_set_coords() {
                out.push((piece, set_coords));
            }
        }

        out
    }

    pub fn get_all_pieces(&self) -> Vec<(Piece, Coords)> {
        let mut out = Vec::new();

        for piece_type in PieceType::iter() {
            let white_piece = Piece { kind: piece_type, colour: Colour::White };
            let black_piece = Piece { kind: piece_type, colour: Colour::Black };
            for set_coords in self.white_bit_boards[piece_type as usize].get_set_coords() {
                out.push((white_piece, set_coords));
            }
            for set_coords in self.black_bit_boards[piece_type as usize].get_set_coords() {
                out.push((black_piece, set_coords));
            }
        }

        out
    }

    fn get_bit_board(&self, piece: &Piece) -> &BitBoard {
        match piece.colour {
            Colour::White => &self.white_bit_boards[piece.kind as usize],
            Colour::Black => &self.black_bit_boards[piece.kind as usize],
        }
    }

    fn get_bit_board_mut(&mut self, piece: &Piece) -> &mut BitBoard {
        match piece.colour {
            Colour::White => &mut self.white_bit_boards[piece.kind as usize],
            Colour::Black => &mut self.black_bit_boards[piece.kind as usize],
        }
    }


    pub fn set_board_from_fenstr(&mut self, fenstr_board: &str) {
        // Clear all bitboards
        for bitboard in &mut self.white_bit_boards {
            *bitboard = BitBoard::new();
        }
        for bitboard in &mut self.black_bit_boards {
            *bitboard = BitBoard::new();
        }

        let ranks: Vec<&str> = fenstr_board.split('/').collect();
        if ranks.len() != 8 {
            panic!("Invalid FEN string: incorrect number of ranks");
        }

        for (rank_index, rank_str) in ranks.iter().enumerate() {
            let mut file_index = 0;
            for ch in rank_str.chars() {
                if ch.is_digit(10) {
                    file_index += ch.to_digit(10).unwrap() as usize;
                } else {
                    let colour = if ch.is_uppercase() { Colour::White } else { Colour::Black };
                    let piece_type = match ch.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'r' => PieceType::Rook,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => panic!("Invalid FEN string: unknown piece {}", ch),
                    };

                    let coords = Coords::new(8 - rank_index as u8, crate::enums::File::from_usize(file_index).unwrap());
                    let piece = Piece { kind: piece_type, colour: colour };
                    self.get_bit_board_mut(&piece).set_bit(&coords, true);
                    file_index += 1;
                }
            }
            if file_index != 8 {
                panic!("Invalid FEN string: incorrect number of files in rank {}", rank_index + 1);
            }
        }

        self.recompute_occupancy();
    }

    pub fn set_coords(&mut self, coords: &Coords, maybe_piece: Option<Piece>) {
        let index = coords.to_index();
        let mask = 1u64 << index;


        self.white_occ = BitBoard::from_bits(self.white_occ.bits() & !mask);
        self.black_occ = BitBoard::from_bits(self.black_occ.bits() & !mask);


        // Clear all bitboards first
        for colour in [Colour::White, Colour::Black] {
            for piece_type in PieceType::iter() {
                let piece = Piece { kind: piece_type, colour };
                let bitboard = self.get_bit_board_mut(&piece);
                bitboard.set_bit(coords, false);
            }
        }

        // If we're setting a new piece
        if let Some(piece) = maybe_piece {
            self.get_bit_board_mut(&piece).set_bit(coords, true);

            match piece.colour {
                Colour::White => {
                    self.white_occ = BitBoard::from_bits(self.white_occ.bits() | mask);
                }
                Colour::Black => {
                    self.black_occ = BitBoard::from_bits(self.black_occ.bits() | mask);
                }
            }
        }

        // Always recompute all_occ
        self.all_occ = BitBoard::from_bits(self.white_occ.bits() | self.black_occ.bits());
    }

    pub fn move_piece(&mut self, piece: &Piece, from: &Coords, to: &Coords) {
        {
            let bitboard = self.get_bit_board(piece);
            if !bitboard.is_set(from) {
                println!("Trying to move {:?} from {:?} to {:?}", piece, from , to);
                panic!("No piece found at the source coordinates {:?}", from);

            }
        }

        self.set_coords(from, None);
        self.set_coords(to, Some(*piece));

    }

    pub fn get_coords(&self, coords: &Coords) -> Option<Piece> {
        for colour in [Colour::White, Colour::Black] {
            let bitboards = match colour {
                Colour::White => &self.white_bit_boards,
                Colour::Black => &self.black_bit_boards,
            };

            for piece in PieceType::iter() {
                if bitboards[piece as usize].is_set(coords) {
                    return Some(
                        Piece {
                            kind: piece,
                            colour: colour
                        }
                    );
                }
            }
        }
        None
    }

    pub fn get_piece_coords(&self, piece: Piece) -> Vec<Coords> {
        let bitboards = match piece.colour {
            Colour::White => &self.white_bit_boards,
            Colour::Black => &self.black_bit_boards,
        };

        bitboards[piece.kind as usize].get_set_coords()
    }

    pub fn white_occ(&self) -> BitBoard { self.white_occ }
    pub fn black_occ(&self) -> BitBoard { self.black_occ }
    pub fn all_occ(&self) -> BitBoard { self.all_occ }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::File;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();

        for bb in board.white_bit_boards.iter().chain(board.black_bit_boards.iter()) {
            assert!(bb.is_empty(), "Expected all bitboards to be empty");
        }
    }

    #[test]
    fn test_start_position_setup() {
        let board = Board::setup_startposition();

        // Check a few representative squares
        assert!(board.white_bit_boards[PieceType::Rook as usize]
            .is_set(&Coords::new(1, File::A)));
        assert!(board.white_bit_boards[PieceType::Pawn as usize]
            .is_set(&Coords::new(2, File::E)));

        assert!(board.black_bit_boards[PieceType::King as usize]
            .is_set(&Coords::new(8, File::E)));
        assert!(board.black_bit_boards[PieceType::Pawn as usize]
            .is_set(&Coords::new(7, File::D)));
    }

    #[test]
    fn test_set_fenstr_valid() {
        let mut board = Board::new();
        // FEN string for just a white king at e4
        board.set_board_from_fenstr("4K3/8/8/8/8/8/8/8");

        assert!(board.white_bit_boards[PieceType::King as usize]
            .is_set(&Coords::new(8, File::E)));
    }

    #[test]
    #[should_panic(expected = "Invalid FEN string: unknown piece")]
    fn test_set_fenstr_invalid_piece() {
        let mut board = Board::new();
        board.set_board_from_fenstr("4X3/8/8/8/8/8/8/8");
    }

    #[test]
    #[should_panic(expected = "Invalid FEN string: incorrect number of ranks")]
    fn test_set_fenstr_invalid_rank_count() {
        let mut board = Board::new();
        board.set_board_from_fenstr("8/8/8/8/8/8/8");
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();

        let white_pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords { rank: 2, file: File::A }; // e2
        let to = Coords { rank: 3, file: File::A }; // e2

        // Place a pawn at "from"
        {
            let bitboard = board.get_bit_board_mut(&white_pawn);
            bitboard.set_bit(&from, true);
        }

        // Move it
        board.move_piece(&white_pawn, &from, &to);

        let bitboard = board.get_bit_board(&white_pawn);

        // Source should be cleared
        assert!(!bitboard.is_set(&from), "Source square should be cleared");

        // Destination should be set
        assert!(bitboard.is_set(&to), "Destination square should be set");
    }

    #[test]
    #[should_panic(expected = "No piece found at the source coordinates")]
    fn test_move_piece_invalid_panics() {
        let mut board = Board::new();

        let from = Coords { rank: 2, file: File::A };
        let to   = Coords { rank: 3, file: File::A };

        let pawn = Piece { kind: PieceType::Pawn, colour: Colour::White};

        // Do not place anything at `from`
        board.move_piece(&pawn, &from, &to);
    }

    #[test]
    fn test_get_player_pieces_empty_board() {
        let board = Board::new();
        let white_pieces = board.get_player_pieces(Colour::White);
        let black_pieces = board.get_player_pieces(Colour::Black);

        assert!(white_pieces.is_empty());
        assert!(black_pieces.is_empty());
    }

    #[test]
    fn test_get_player_pieces_single_piece() {
        let mut board = Board::new();
        let coord = Coords::new(2, File::E);
        board.white_bit_boards[PieceType::Pawn as usize].set_bit(&coord, true);

        let white_pieces = board.get_player_pieces(Colour::White);

        assert_eq!(white_pieces.len(), 1);
        assert_eq!(white_pieces[0], (Piece { kind: PieceType::Pawn, colour: Colour::White }, coord));
    }

    #[test]
    fn test_get_player_pieces_multiple_pieces() {
        let mut board = Board::new();

        // Add a white knight on g1
        let knight_coord = Coords::new(1, File::G);
        board.white_bit_boards[PieceType::Knight as usize].set_bit(&knight_coord, true);

        // Add a white rook on a1
        let rook_coord = Coords::new(1, File::A);
        board.white_bit_boards[PieceType::Rook as usize].set_bit(&rook_coord, true);

        let white_pieces = board.get_player_pieces(Colour::White);

        assert_eq!(white_pieces.len(), 2);
        assert!(white_pieces.contains(&(Piece { kind: PieceType::Knight, colour: Colour::White }, knight_coord)));
        assert!(white_pieces.contains(&(Piece { kind: PieceType::Rook, colour: Colour::White }, rook_coord)));
    }

    #[test]
    fn test_occ_boards_start_position() {
        let board = Board::setup_startposition();

        // Each side starts with 16 pieces
        assert_eq!(board.white_occ().num_set_bits(), 16);
        assert_eq!(board.black_occ().num_set_bits(), 16);

        // Total occupancy = white + black
        assert_eq!(board.all_occ.num_set_bits(), 32);

        // Check that white and black occupancies don’t overlap
        assert_eq!(board.white_occ().bits() & board.black_occ().bits(), 0);
    }

    #[test]
    fn test_occ_boards_after_piece_move() {
        let mut board = Board::setup_startposition();
        let pawn = Piece { kind: PieceType::Pawn, colour: Colour::White };
        let from = Coords::new(2, File::E);
        let to = Coords::new(4, File::E);

        board.move_piece(&pawn, &from, &to);

        // The moved square should no longer be occupied
        assert!(!board.white_occ().is_set(&from));
        // The destination should be occupied
        assert!(board.white_occ().is_set(&to));
        // Total count remains the same (no capture)
        assert_eq!(board.white_occ().num_set_bits(), 16);
        assert_eq!(board.all_occ().num_set_bits(), 32);
    }

    #[test]
    fn test_occ_boards_after_capture() {
        let mut board = Board::new();

        let white_rook = Piece { kind: PieceType::Rook, colour: Colour::White };
        let black_pawn = Piece { kind: PieceType::Pawn, colour: Colour::Black };
        let from = Coords::new(2, File::A);
        let to = Coords::new(3, File::A);

        // Place white rook and black pawn
        board.set_coords(&from, Some(white_rook));
        board.set_coords(&to, Some(black_pawn));

        // Check initial occupancy
        assert!(board.white_occ().is_set(&from));
        assert!(board.black_occ().is_set(&to));
        assert!(board.all_occ().is_set(&from) && board.all_occ().is_set(&to));

        // White captures black pawn
        board.move_piece(&white_rook, &from, &to);

        // After capture, black’s occupancy should lose one bit
        assert!(!board.black_occ().is_set(&to));
        // White occupies destination
        assert!(board.white_occ().is_set(&to));
        // Source is now empty
        assert!(!board.white_occ().is_set(&from));

        // No overlap
        assert_eq!(board.white_occ().bits() & board.black_occ().bits(), 0);
    }

    #[test]
    fn test_occ_boards_fen_loading() {
        let mut board = Board::new();
        // A simple position: white king + rook vs black king
        board.set_board_from_fenstr("4k3/8/8/8/8/8/8/4KR2");

        assert_eq!(board.white_occ().num_set_bits(), 2);
        assert_eq!(board.black_occ().num_set_bits(), 1);
        assert_eq!(board.all_occ().num_set_bits(), 3);

        // Check exact squares
        assert!(board.white_occ().is_set(&Coords::new(1, File::E)));
        assert!(board.white_occ().is_set(&Coords::new(1, File::F)));
        assert!(board.black_occ().is_set(&Coords::new(8, File::E)));
    }

}
