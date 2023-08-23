from Pieces.piece import Piece
from enums import Colour

class Bishop(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)

    def get_representation(self):
        return 'b' if self.colour == Colour.BLACK else 'B'

    def get_moves(self, board: [[Piece]]) -> [tuple]:
        super().check_move_errors(board, self.get_representation())
        # list of tuples of new coordinates the piece can go
        