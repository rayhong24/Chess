use crate::enums::Colour;
use crate::coords::Coords;
use crate::enums::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalMove {
    pub colour: Colour,
    pub piece: Piece,
    pub from: Coords,
    pub to: Coords,
    pub captured_piece: Option<Piece>,
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
    pub promoted_piece: Piece,
    pub captured_piece: Option<Piece>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnPassantMove {
    pub colour: Colour,
    pub from: Coords,
    pub to: Coords,
    pub captured_pawn_coords: Coords,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChessMove {
    Normal(NormalMove),
    Castling(CastlingMove),
    Promotion(PromotionMove),
    EnPassant(EnPassantMove),
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

    pub fn piece(&self) -> Piece {
        match self {
            ChessMove::Normal(mv) => mv.piece,
            ChessMove::Castling(_) => Piece::King,
            ChessMove::Promotion(_) => Piece::Pawn,
            ChessMove::EnPassant(_) => Piece::Pawn,
        }
    }
}