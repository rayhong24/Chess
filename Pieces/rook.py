from Pieces.piece import Piece
from enums import Colour

class Rook(Piece):
    def __init__(self, board, colour, row, column):
        super().__init__(board, colour, row, column)

    def get_representation(self):
        return 'r' if self.colour == Colour.BLACK else 'R'

    def get_moves(self):
        super().check_move_errors(self.get_representation())
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
            i, j = self.row + di, self.column + dj

            while 0<=i<=7 and 0<=j<=7 and self.board.board[i][j] == None:
                valid_moves.append((i, j))
                i, j = i+di, j+dj

            if 0<=i<=7 and 0<=j<=7 and self.board.board[i][j] != None and self.board.board[i][j].colour != self.colour:
                valid_moves.append((i, j))
            

        return valid_moves

            


