from python_chess.Pieces.piece import Piece
from python_chess.Pieces.moveCandidate import MoveCandidate
from python_chess.enums import Colour
from python_chess.coords import Coords

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

    def get_value(self, coords: Coords) -> int:
        base_value = 9 if self.colour == Colour.WHITE else -9
        return base_value