from typing import Self

from enums import Colour
from enums import File
from coords import Coords
from strings import *

from Pieces.pawn import Pawn

class Piece:
    def __init__(self, colour: Colour, coords: Coords) -> None:
        self.colour = colour
        self.has_moved = False

        self.value = 0

    def get_representation(self) -> str:
        raise NotImplementedError

    # generator for possible coordinates moves
    def get_candidate_moves(self, coords):
        raise NotImplementedError

    def promote(self, promotion_str: str) -> Self:
        if not isinstance(self, Pawn):
            raise ValueError("Only pawns can be promoted")
        
        if promotion_str == "q":
            return Queen(self.colour, self.coords)
        elif promotion_str == "r":
            return Rook(self.colour, self.coords)
        elif promotion_str == "b":  
            return Bishop(self.colour, self.coords)
        elif promotion_str == "n":
            return Knight(self.colour, self.coords)
        else:
            raise ValueError("Invalid promotion piece type")

    