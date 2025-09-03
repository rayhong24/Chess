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

    fn get_bit_board(&self, colour: Colour, piece: PieceType) -> &BitBoard {
        match colour {
            Colour::White => &self.white_bit_boards[piece as usize],
            Colour::Black => &self.black_bit_boards[piece as usize],
        }
    }

    fn get_bit_board_mut(&mut self, colour: Colour, piece: PieceType) -> &mut BitBoard {
        match colour {
            Colour::White => &mut self.white_bit_boards[piece as usize],
            Colour::Black => &mut self.black_bit_boards[piece as usize],
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
        println!("asdfasdjfsdklfjasdfklasdjflasdjfasdklf {}", fenstr_board);

        for (rank_index, rank_str) in ranks.iter().enumerate() {
            let mut file_index = 0;
            for ch in rank_str.chars() {
                if ch.is_digit(10) {
                    file_index += ch.to_digit(10).unwrap() as usize;
                } else {
                    let colour = if ch.is_uppercase() { Colour::White } else { Colour::Black };
                    let piece = match ch.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'r' => PieceType::Rook,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => panic!("Invalid FEN string: unknown piece {}", ch),
                    };

                    let coords = Coords::new(8 - rank_index as u8, crate::enums::File::from_usize(file_index).unwrap());
                    self.get_bit_board_mut(colour, piece).set_bit(&coords, true);
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
            for piece in PieceType::iter() {
                let mut bitboard = self.get_bit_board_mut(colour, piece);

                let set = maybe_piece.is_some_and(
                    |p| p.colour == colour && p.kind == piece
                );
                bitboard.set_bit(coords, set);
            }
        }
    }

    pub fn move_piece(&mut self, piece_type: &PieceType, colour: &Colour, from: &Coords, to: &Coords) {
        {
            let bitboard = self.get_bit_board(*colour, *piece_type);
            if !bitboard.is_set(from) {
                panic!("No piece found at the source coordinates {:?}", from);
            }
        }

        self.set_coords(from, None);
        self.set_coords(to, Some(Piece { kind: *piece_type, colour: *colour}));

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

        let piece = PieceType::Pawn;
        let colour = Colour::White;
        let from = Coords { rank: 2, file: File::A }; // e2
        let to = Coords { rank: 3, file: File::A }; // e2

        // Place a pawn at "from"
        {
            let bitboard = board.get_bit_board_mut(colour, piece);
            bitboard.set_bit(&from, true);
        }

        // Move it
        board.move_piece(&piece, &colour, &from, &to);

        let bitboard = board.get_bit_board(colour, piece);

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

        let pawn = PieceType::Pawn;
        let colour = Colour::White;

        // Do not place anything at `from`
        board.move_piece(&pawn, &colour, &from, &to);
    }
}
