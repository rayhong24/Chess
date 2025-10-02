use std::fmt;

use crate::enums::Colour;
use crate::coords::Coords;
use crate::piece::Piece;
use crate::enums::{PieceType, File};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalMove {
    pub colour: Colour,
    pub piece_type: PieceType,
    pub from: Coords,
    pub to: Coords,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastlingMove {
    pub colour: Colour,
    pub king_from: Coords,
    pub king_to: Coords,
    pub rook_from: Coords,
    pub rook_to: Coords,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PromotionMove {
    pub colour: Colour,
    pub from: Coords,
    pub to: Coords,
    pub promotion_piece_type: PieceType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnPassantMove {
    pub colour: Colour,
    pub from: Coords,
    pub to: Coords,
    pub captured_coords: Coords,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChessMove {
    Normal(NormalMove),
    Castling(CastlingMove),
    Promotion(PromotionMove),
    EnPassant(EnPassantMove),
}

pub enum ExecutedMove {
    Normal {
        mv: NormalMove,
        captured_piece: Option<Piece>
    },
    Castling {
        mv: CastlingMove
    },
    Promotion {
        mv: PromotionMove,
        captured_piece: Option<Piece>
    },
    EnPassant {
        mv: EnPassantMove
    }

}

impl ChessMove {
    pub fn colour(&self) -> Colour {
        match self {
            ChessMove::Normal(mv) => mv.colour,
            ChessMove::Castling(mv) => mv.colour,
            ChessMove::Promotion(mv) => mv.colour,
            ChessMove::EnPassant(mv) => mv.colour,
        }
    }
    pub fn from(&self) -> Coords {
        match self {
            ChessMove::Normal(mv) => mv.from,
            ChessMove::Castling(mv) => mv.king_from,
            ChessMove::Promotion(mv) => mv.from,
            ChessMove::EnPassant(mv) => mv.from,
        }
    }

    pub fn to(&self) -> Coords {
        match self {
            ChessMove::Normal(mv) => mv.to,
            ChessMove::Castling(mv) => mv.king_to,
            ChessMove::Promotion(mv) => mv.to,
            ChessMove::EnPassant(mv) => mv.to,
        }
    }

    pub fn piece(&self) -> PieceType {
        match self {
            ChessMove::Normal(mv) => mv.piece_type,
            ChessMove::Castling(_) => PieceType::King,
            ChessMove::Promotion(_) => PieceType::Pawn,
            ChessMove::EnPassant(_) => PieceType::Pawn,
        }
    }
}

impl ExecutedMove {
    pub fn is_capture(&self) -> bool {
        match self {
            ExecutedMove::Normal { mv: _, captured_piece } => {
                captured_piece.is_some()
            }
            ExecutedMove::Promotion { mv: _, captured_piece } => {
                captured_piece.is_some()
            }
            ExecutedMove::EnPassant { mv: _ } => true,
            ExecutedMove::Castling { mv: _ } => false

        }
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessMove::Normal(mv) => {
                write!(f, "{}{}", mv.from, mv.to)
            }
            ChessMove::Castling(mv) => {
                write!(f, "{}{}", mv.king_from, mv.king_to)
            }
            ChessMove::Promotion(mv) => {
                write!(f, "{}{}{}", mv.from, mv.to, mv.promotion_piece_type)
            }
            ChessMove::EnPassant(mv) => {
                write!(f, "{}{} e.p.", mv.from, mv.to)
            }
        }
    }
}

impl fmt::Display for ExecutedMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutedMove::Normal { mv, captured_piece } => {
                if let Some(_) = captured_piece {
                    write!(f, "{}x{}", mv.from, mv.to) // simple capture notation
                } else {
                    write!(f, "{}{}", mv.from, mv.to)
                }
            }
            ExecutedMove::Castling { mv } => {
                if mv.king_to.file == File::G {
                    write!(f, "O-O") // kingside
                } else {
                    write!(f, "O-O-O") // queenside
                }
            }
            ExecutedMove::Promotion { mv, captured_piece } => {
                if let Some(_piece) = captured_piece {
                    write!(f, "{}x{}={}", mv.from, mv.to, mv.promotion_piece_type)
                } else {
                    write!(f, "{}{}={}", mv.from, mv.to, mv.promotion_piece_type)
                }
            }
            ExecutedMove::EnPassant { mv } => {
                write!(f, "{}{} e.p.", mv.from, mv.to)
            }
        }
    }
}
