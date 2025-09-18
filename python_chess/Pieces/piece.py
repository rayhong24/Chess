from typing import Self

from python_chess.enums import Colour
from python_chess.enums import File
from python_chess.coords import Coords
from python_chess.strings import *


class Piece:
    def __init__(self, colour: Colour) -> None:
        self.colour = colour
        self.has_moved = False


    def get_representation(self) -> str:
        raise NotImplementedError

    # generator for possible coordinates moves
    def get_candidate_moves(self, coords):
        raise NotImplementedError

    def get_value(self, coords: Coords) -> int:
        return 0
