from Pieces.piece import Piece
from enums import Colour

class Queen(Piece):
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        super().__init__(colour, row, column)

    def get_representation(self) -> str:
        return 'q' if self.colour == Colour.BLACK else 'Q'

    def get_moves(self, board: [[Piece]]) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [
            [-1, -1], [-1, 1], [1, -1], [1, 1],
            [-1, 0], [1, 0], [0, -1], [0, 1]
            ]:
            i, j = self.row + di, self.column + dj

            while self.is_inbounds(i, j) and board[i][j] == None:
                valid_moves.append(self.coords_to_move(self.row, self.column, i, j))
                i, j = i+di, j+dj

            if self.is_inbounds(i, j) and board[i][j] != None and board[i][j].colour != self.colour:
                valid_moves.append(self.coords_to_move(self.row, self.column, i, j))
        
        return valid_moves