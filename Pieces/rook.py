from Pieces.piece import Piece
from coords import Coords
from enums import Colour

class Rook(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)

    def get_representation(self) -> str:
        return 'r' if self.colour == Colour.BLACK else 'R'

    def get_moves(self, game) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
            for line_coords in self.coords.get_line(di, dj):
                square = game.board.get_square(line_coords)
                move = self.move_factory.init_normal_move(
                    self.colour,
                    'R',
                    self.coords,
                    square is not None,
                    line_coords
                )
                if square is not None:
                    if square.colour != self.colour:
                        valid_moves.append(move)
                    break
                valid_moves.append(move)

            
        return valid_moves

            


