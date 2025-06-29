from enums import Colour
from coords import Coords

class PieceSquareTable:
    def __init__(self, board_vals):
        self.board_values = board_vals
    
    def get_value(self, colour: Colour, coords: Coords):
        if colour == Colour.WHITE:
            return self.board_values[coords.rank-1][coords.file.value]
        else:
            return -self.board_values[9-coords.rank][coords.file.value]
        


