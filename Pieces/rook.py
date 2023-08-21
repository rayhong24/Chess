from Pieces.piece import Piece
from enums import Colour

class Rook(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)

    def get_representation(self):
        return 'r' if self.colour == Colour.BLACK else 'R'

    def get_moves(self, board):
        super().check_move_errors(board, self.get_representation())
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
            i, j = self.row + di, self.column + dj

            while board.is_inbounds(i, j) and board.board[i][j] == None:
                valid_moves.append((i, j))
                i, j = i+di, j+dj

            if board.is_inbounds(i, j) and board[i][j] != None and board[i][j].colour != self.colour:
                valid_moves.append((i, j))
            

            


