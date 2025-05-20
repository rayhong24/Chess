from Pieces.piece import Piece
from Pieces.moveCandidate import MoveCandidate
from enums import Colour
from coords import Coords

class Queen(Piece):
    def __init__(self, colour: Colour) -> None:
        super().__init__(colour)
        self.value = 9

    def get_representation(self) -> str:
        return 'q' if self.colour == Colour.BLACK else 'Q'

    def get_candidate_moves(self, coords: Coords):
        # list of tuples of new coordinates the piece can go
        candidates = []

        for di, dj in [
            [-1, -1], [-1, 1], [1, -1], [1, 1],
            [-1, 0], [1, 0], [0, -1], [0, 1]
            ]:

            candidate = MoveCandidate(di, dj, 8)
            candidates.append(candidate)
        
        return candidates