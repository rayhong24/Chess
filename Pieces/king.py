from Pieces.piece import Piece
from enums import Colour

class King(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'k' if self.colour == Colour.BLACK else 'K'