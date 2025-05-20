from Pieces.piece import Piece
from enums import Colour
from coords import Coords

from Pieces.moveCandidate import MoveCandidate

class Bishop(Piece):
    def __init__(self, colour: Colour) -> None:
        super().__init__(colour)

        self.value = 3

    def get_representation(self) -> str:
        return 'b' if self.colour == Colour.BLACK else 'B'

    def get_candidate_moves(self, coords: Coords):
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        for di, dj in [[-1, -1], [-1, 1], [1, -1], [1, 1]]:
            valid_moves.append(
                MoveCandidate(di, dj, 8)
            )

        return valid_moves
            

        