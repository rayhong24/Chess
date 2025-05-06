from Pieces.piece import Piece

from Pieces.moveCandidate import MoveCandidate
from coords import Coords
from enums import Colour

class Rook(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)

        self.value = 5

    def get_representation(self) -> str:
        return 'r' if self.colour == Colour.BLACK else 'R'

    def get_candidate_moves(self, coords: Coords):
        # list of tuples of new coordinates the piece can go
        candidates = []

        for di, dj in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
            candidate = MoveCandidate(di, dj, 8)

            candidates.append(candidate)
            
        return candidates

            


