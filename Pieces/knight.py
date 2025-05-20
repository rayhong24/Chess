from Pieces.piece import Piece

from Pieces.moveCandidate import MoveCandidate
from enums import Colour
from coords import Coords

class Knight(Piece):
    def __init__(self, colour: Colour) -> None:
        super().__init__(colour)

        self.value = 3

    def get_representation(self) -> str:
        return 'n' if self.colour == Colour.BLACK else 'N'

    def get_candidate_moves(self, coords: Coords):
        # list of tuples of new coordinates the piece can go
        candidate_moves = []

        for di, dj in [
            [-2, -1], [-2, 1], [-1, -2], [-1, 2],
            [1, -2], [1, 2], [2, -1], [2, 1]
        ]:
            candidate = MoveCandidate(di, dj)
            candidate_moves.append(candidate)
        
        return candidate_moves