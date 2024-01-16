from Pieces.piece import Piece
from enums import Colour

class Rook(Piece):
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        super().__init__(colour, row, column)

    def get_representation(self) -> str:
        return 'r' if self.colour == Colour.BLACK else 'R'

    def get_moves(self, board: [[Piece]]) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
            i, j = self.row + di, self.column + dj

            while self.is_inbounds(i, j) and board[i][j] == None:
                move_str = self.get_move_str(self.row, self.column, i, j, False)
                move = self.move_factory(move_str, self.colour)
                valid_moves.append(move)
                i, j = i+di, j+dj

            if self.is_inbounds(i, j) and board[i][j] != None and board[i][j].colour != self.colour:
                is_capture = self.is_inbounds(i, j) and board[i][j] is not None
                move_str = self.get_move_str(self.row, self.column, i, j, is_capture)
                move = self.move_factory(move_str, self.colour)
                valid_moves.append(move)
            
        return valid_moves

            


