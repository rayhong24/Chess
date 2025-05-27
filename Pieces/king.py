from Pieces.piece import Piece
from Pieces.moveCandidate import MoveCandidate
from enums import *
from coords import Coords

class King(Piece):
    def __init__(self, colour: Colour) -> None:
        super().__init__(colour)

    def get_representation(self) -> str:
        return 'k' if self.colour == Colour.BLACK else 'K'

    def get_candidate_moves(self, coords: Coords):
        # list of tuples of new coordinates the piece can go
        candidates = []

        for di, dj in [
            [-1, -1], [-1, 1], [1, -1], [1, 1],
            [-1, 0], [1, 0], [0, -1], [0, 1]
            ]:

            dist = 1

            # castling
            if (str(coords) == "e1" or str(coords) == "e8") and di == 0:
                dist = 2
            candidate = MoveCandidate(di, dj, dist)
            candidates.append(candidate)


        return candidates