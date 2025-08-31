use crate::enums::Colour;
use crate::coords::Coords;
use crate::enums::Piece;

pub struct NormalMove {
    pub colour: Colour,
    pub from: Coords,
    pub to: Coords,
    pub piece: Piece,
    pub captured_piece: Option<Piece>,
}

pub struct CastlingMove {
    pub colour: Colour,
    pub king_from: Coords,
    pub king_to: Coords,
    pub rook_from: Coords,
    pub rook_to: Coords,
}

pub struct PromotionMove {
    pub colour: Colour,
    pub from: Coords,
    pub to: Coords,
    pub promoted_piece: Piece,
    pub captured_piece: Option<Piece>,
}

pub struct EnPassantMove {
    pub colour: Colour,
    pub from: Coords,
    pub to: Coords,
    pub captured_pawn_coords: Coords,
}

pub enum ChessMove {
    Normal(NormalMove),
    Castling(CastlingMove),
    Promotion(PromotionMove),
    EnPassant(EnPassantMove),
}