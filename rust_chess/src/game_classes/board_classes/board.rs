use strum::IntoEnumIterator;

use crate::enums::piece_types::PIECE_COUNT;
use crate::enums::{Colour, PieceType};
use crate::coords::Coords;
use crate::piece::Piece;
use crate::game_classes::board_classes::bit_board::{self, BitBoard};
// use crate::coords::Coords;
// use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    white_bit_boards: [BitBoard; PIECE_COUNT],
    black_bit_boards: [BitBoard; PIECE_COUNT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_bit_boards: [BitBoard::new(); PIECE_COUNT],
            black_bit_boards: [BitBoard::new(); PIECE_COUNT],
        }   
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
    }

    pub fn set_coords(&mut self, coords: &Coords, maybe_piece: Option<Piece>) {
        // Remove any piece (white or black) from these coords
        for colour in [Colour::White, Colour::Black] {
            for piece_type in PieceType::iter() {
                let piece = Piece { kind: piece_type, colour: colour };
                let mut bitboard = self.get_bit_board_mut(&piece);

                let set = maybe_piece.is_some_and(
                    |p| p.colour == colour && p.kind == piece_type
                );
                bitboard.set_bit(coords, set);
            }
        }
    }

    pub fn move_piece(&mut self, piece: &Piece, from: &Coords, to: &Coords) {
        {
            let bitboard = self.get_bit_board(piece);
            if !bitboard.is_set(from) {
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

    pub fn get_material(&self, colour: Colour) -> i32 {
        let bitboards = match colour {
            Colour::White => &self.white_bit_boards,
            Colour::Black => &self.black_bit_boards,
        };

        let mut out = 0;

        for piece_type in PieceType::iter() {
            let value = match piece_type {
                PieceType::Pawn   => 1,
                PieceType::Knight => 3,
                PieceType::Bishop => 3,
                PieceType::Rook   => 5,
                PieceType::Queen  => 9,
                PieceType::King   => 0,
            };

            out += value * bitboards[piece_type as usize].num_set_bits();
        }

        out
    }
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
    fn test_get_material_count() {
        let mut board = Board::setup_startposition();

        assert_eq!(board.get_material(Colour::White), 39);
        assert_eq!(board.get_material(Colour::Black), 39);
    }
}
