from Pieces.piece import Piece
from enums import Colour

class Queen(Piece):
    def __init__(self, board, colour, row, column):
        super().__init__(board, colour, row, column)
    def get_representation(self):
        return 'q' if self.colour == Colour.BLACK else 'Q'