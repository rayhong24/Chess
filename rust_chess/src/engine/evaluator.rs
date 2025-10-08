use crate::engine::piece_square_tables::{BISHOP_PST, KING_PST, KNIGHT_PST, PAWN_PST, QUEEN_PST, ROOK_PST};
use crate::enums::{Colour, PieceType};
use crate::coords::Coords;
use crate::piece::Piece;
use crate::game_classes::game::Game;
use crate::game_classes::game::GameResult;
use crate::engine::minimax::INF;

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 20000;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate_game_result(game: &mut Game, game_result: Option<GameResult>, depth: usize, to_move: Colour) -> i32 {
        match game_result {
            Some(GameResult::Checkmate(_)) => {
                -INF + 100 + depth as i32
            }
            Some(GameResult::Stalemate) | Some(GameResult::Draw) => 0,
            None => Self::evaluate_pst(game)
        }
    }

    pub fn get_piece_value(piece_type: PieceType) -> i32 {
        match piece_type {
            PieceType::Pawn => PAWN_VALUE,
            PieceType::Knight => KNIGHT_VALUE,
            PieceType::Bishop => BISHOP_VALUE,
            PieceType::Rook => ROOK_VALUE,
            PieceType::Queen => QUEEN_VALUE,
            PieceType::King => KING_VALUE,
        }
    }

    fn pst_value(pst: &[[i32; 8]; 8], coords: Coords, colour: Colour) -> i32 {
        match colour {
            Colour::White => pst[8 - coords.rank as usize][coords.file as usize], // rank 0 is White’s back rank
            Colour::Black => pst[coords.rank as usize - 1][coords.file as usize],     // rank 7 is Black’s back rank
        }
    }

    fn evaluate_piece(piece: Piece, coords: Coords) -> i32 {
        match piece.kind {
            PieceType::Pawn => PAWN_VALUE + Self::pst_value(&PAWN_PST, coords, piece.colour),
            PieceType::Knight => KNIGHT_VALUE + Self::pst_value(&KNIGHT_PST, coords, piece.colour),
            PieceType::Bishop => BISHOP_VALUE + Self::pst_value(&BISHOP_PST, coords, piece.colour),
            PieceType::Rook => ROOK_VALUE+ Self::pst_value(&ROOK_PST, coords, piece.colour),
            PieceType::Queen => QUEEN_VALUE+ Self::pst_value(&QUEEN_PST, coords, piece.colour),
            PieceType::King => KING_VALUE + Self::pst_value(&KING_PST, coords, piece.colour)
        }


    }

    fn evaluate_pst(game: &mut Game) -> i32 {
        let current_player = game.get_game_state().get_turn();

        let mut eval = 0;

        for (piece, coords) in game.get_board().get_player_pieces(current_player) {
            eval += Self::evaluate_piece(piece, coords);
        }

        for (piece, coords) in game.get_board().get_player_pieces(current_player.other()) {
            eval -= Self::evaluate_piece(piece, coords);
        }

        eval
    }
}