from enums import Colour
from strings import *

class Piece:
    def __init__(self, colour, row, column):
        self.colour = colour
        self.row = row
        self.column = column

class Pawn(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
        self.has_moved = False
        
    
    def get_representation(self):
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_moves(self, board, i, j):
        if board[i][j] is None:
            print(invalid_move_no_piece_message.format(i, j))
            return
        elif board[i][j].colour != self.colour:
            print(invalid_move_wrong_colour.format(i, j))
            


class Rook(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)

    def get_representation(self):
        return 'r' if self.colour == Colour.BLACK else 'R'

class Knight(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'n' if self.colour == Colour.BLACK else 'N'

class Bishop(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'b' if self.colour == Colour.BLACK else 'B'

class Queen(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'q' if self.colour == Colour.BLACK else 'Q'

class King(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'k' if self.colour == Colour.BLACK else 'K'
    
