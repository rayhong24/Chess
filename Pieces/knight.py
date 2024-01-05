from Pieces.piece import Piece
from enums import Colour

class Knight(Piece):
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        super().__init__(colour, row, column)

    def get_representation(self) -> str:
        return 'n' if self.colour == Colour.BLACK else 'N'

    def get_moves(self, board: [[Piece]]) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [
            [-2, -1], [-2, 1], [-1, -2], [-1, 2],
            [1, -2], [1, 2], [2, -1], [2, 1]
            ]:
            i, j = self.row + di, self.column + dj

            if self.is_inbounds(i, j) and (board[i][j] == None or board[i][j].colour != self.colour):
                is_capture = board[i][j] is not None
                valid_moves.append(self.get_move_str(self.row, self.column, i, j, is_capture))
        
        return valid_moves