from Pieces.piece import Piece
from Pieces.rook import Rook
from enums import Colour

class King(Piece):
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        super().__init__(colour, row, column)

    def get_representation(self) -> str:
        return 'k' if self.colour == Colour.BLACK else 'K'

    def get_moves(self, board: [[Piece]]) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [
            [-1, -1], [-1, 1], [1, -1], [1, 1],
            [-1, 0], [1, 0], [0, -1], [0, 1]
            ]:
            i, j = self.row + di, self.column + dj

            if self.is_inbounds(i, j) and (board[i][j] == None or board[i][j].colour != self.colour):
                is_capture = self.is_inbounds(i, j) and board[i][j] is not None
                move_str = self.get_move_str(self.row, self.column, i, j, is_capture)
                move = self.move_factory.init_move(move_str, self.colour)
                valid_moves.append(move)

        # Check king side castle
        if self.column == 4 and not self.has_moved \
            and board[self.row][5] == None and board[self.row][6] == None\
            and type(board[self.row][7]) == Rook and not board[self.row][7].has_moved:
                move = self.move_factory.init_move("O-O", self.colour)
                valid_moves.append(move)

        if self.column == 4 and not self.has_moved \
            and board[self.row][3] == None and board[self.row][2] == None and board[self.row][1] == None\
            and type(board[self.row][0]) == Rook and not board[self.row][0].has_moved:
                move = self.move_factory("O-O-O", self.colour)
                valid_moves.append(move)
        
        return valid_moves