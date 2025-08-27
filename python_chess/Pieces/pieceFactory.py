from python_chess.enums import *

from python_chess.Pieces.piece import Piece
from python_chess.Pieces.pawn import Pawn
from python_chess.Pieces.rook import Rook
from python_chess.Pieces.knight import Knight
from python_chess.Pieces.bishop import Bishop
from python_chess.Pieces.queen import Queen
from python_chess.Pieces.king import King
from python_chess.coords import Coords

class PieceFactory:
    def init_piece(self, piece_str, coords: Coords) -> Piece:
        initializer = self._get_piece_initializer(piece_str)

        colour = Colour.WHITE if piece_str.isupper() else Colour.BLACK

        return initializer(colour)

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