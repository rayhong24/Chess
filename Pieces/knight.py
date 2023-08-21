from Pieces.piece import Piece
from enums import Colour

class Knight(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'n' if self.colour == Colour.BLACK else 'N'