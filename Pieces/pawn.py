from Pieces.piece import Piece
from enums import Colour
from coords import Coords

from Pieces.moveCandidate import MoveCandidate
from Pieces.rook import Rook
from Pieces.queen import Queen
from Pieces.bishop import Bishop
from Pieces.knight import Knight

class Pawn(Piece):
    def __init__(self, colour: Colour) -> None:
        super().__init__(colour)

    
    #TODO Change to __str__ or __repr__
    def get_representation(self) -> str:
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_candidate_moves(self, coords: Coords):
        moves = []

        direction = 1 if self.colour == Colour.WHITE else -1

        # checking forward moves
        moves_forward = 2 if \
            (coords.rank == 2 and self.colour == Colour.WHITE) \
            or (coords.rank == 7 and self.colour == Colour.BLACK) else 1

        move = MoveCandidate(
            direction,
            0,
            moves_forward,
            False,
            False
        )

        moves.append(move)

        # check captures
        move = MoveCandidate(
            direction,
            -1,
            1,
            True,
            True,
        )
        moves.append(move)

        move = MoveCandidate(
            direction,
            1,
            1,
            True,
            True,
        )
        moves.append(move)

        return moves

    