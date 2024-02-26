from Pieces.piece import Piece
from enums import Colour
from coords import Coords

class Bishop(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)

    def get_representation(self) -> str:
        return 'b' if self.colour == Colour.BLACK else 'B'

    def get_moves(self, game) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [[-1, -1], [-1, 1], [1, -1], [1, 1]]:
            i, j = self.row + di, self.column + dj

            while self.is_inbounds(i, j) and game.board.board[i][j] == None:
                move_str = self.get_move_str(self.row, self.column, i, j, False)
                move = self.move_factory.init_move(move_str, self.colour, game)
                valid_moves.append(move)
                i, j = i+di, j+dj

            if self.is_inbounds(i, j) and game.board.board[i][j] != None and game.board.board[i][j].colour != self.colour:
                move_str = self.get_move_str(self.row, self.column, i, j, True)
                move = self.move_factory.init_move(move_str, self.colour, game)
                valid_moves.append(move)

        return valid_moves
            

        