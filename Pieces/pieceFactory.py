from enums import *

from Pieces.piece import Piece
from Pieces.pawn import Pawn
from Pieces.rook import Rook
from Pieces.knight import Knight
from Pieces.bishop import Bishop
from Pieces.queen import Queen
from Pieces.king import King
from coords import Coords

class PieceFactory:
    def init_piece(self, piece_str, coords: Coords) -> Piece:
        initializer = self._get_piece_initializer(piece_str)

        colour = Colour.WHITE if piece_str.isupper() else Colour.BLACK

        return initializer(colour, coords)

    def _get_piece_initializer(self, piece_str):
        piece_dict = {
            'P': Pawn,
            'R': Rook,
            'N': Knight,
            'B': Bishop,
            'Q': Queen,
            'K': King
        }

        return piece_dict[piece_str.upper()]

        