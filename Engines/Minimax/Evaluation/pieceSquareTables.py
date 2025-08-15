from pieceSquareTableValues import table_values

from Pieces.piece import Piece
from enums import Colour
from coords import Coords

class PieceSquareTables:
    def get_value(self, piece: Piece, coords: Coords):
        piece_table_values = table_values[type(Piece)]
        if piece.colour == Colour.WHITE:
            return piece_table_values[coords.rank-1][coords.file.value]
        else:
            return -piece_table_values[8-coords.rank][coords.file.value]
        


