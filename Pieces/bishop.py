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
            for line_coords in self.coords.get_line((di, dj)):
                square = game.board.get_square(line_coords)
                if square != None:
                    if square.colour != self.colour:
                        move_str = self.get_move_str(self.row, self.column, i, j, True)
                        move = self.move_factory.init_move(move_str, self.colour, game)
                        valid_moves.append(move)
                    break
                move_str = self.get_move_str(self.row, self.column, i, j, False)
                move = self.move_factory.init_move(move_str, self.colour, game)
                valid_moves.append(move)

        return valid_moves
            

        