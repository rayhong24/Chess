from Pieces.piece import Piece
from enums import Colour
from coords import Coords

from Pieces.moveCandidate import MoveCandidate

class Pawn(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)
        self.has_moved = not (
            (colour == Colour.WHITE and coords.rank == 2) or\
            (colour == Colour.BLACK and coords.rank == 7)
        )
    
    #TODO Change to __str__ or __repr__
    def get_representation(self) -> str:
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_candidate_moves(self):
        direction = 1 if self.colour == Colour.WHITE else -1

        # checking forward moves
        moves_forward = 1 if self.has_moved else 2

        move = MoveCandidate(
            False,
            direction,
            0,
            moves_forward
        )
        yield(move)


        # check captures
        move = MoveCandidate(
            True,
            direction,
            -1,
            1
        )
        yield(move)

        move = MoveCandidate(
            True,
            direction,
            1,
            1
        )
        yield(move)