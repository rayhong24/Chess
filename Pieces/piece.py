from typing import Self

from enums import Colour
from enums import File
from coords import Coords
from strings import *


class Piece:
    def __init__(self, colour: Colour) -> None:
        self.colour = colour
        self.has_moved = False

        self.value = 0

    def get_representation(self) -> str:
        raise NotImplementedError

    # generator for possible coordinates moves
    def get_candidate_moves(self, coords):
        raise NotImplementedError
