from Pieces.piece import Piece
from enums import Colour
from coords import Coords

class Knight(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)

    def get_representation(self) -> str:
        return 'n' if self.colour == Colour.BLACK else 'N'

    def get_moves(self, game) -> [str]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for jump_coords in self.coords.get_knight_jumps():
            square = game.board.get_square(jump_coords)
            if square == None or square.colour != self.colour:
                move = self.move_factory.init_normal_move(
                    self.colour,
                    'N',
                    self.coords,
                    square is not None,
                    jump_coords
                )
                valid_moves.append(move)
        
        return valid_moves