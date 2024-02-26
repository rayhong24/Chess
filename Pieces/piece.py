from typing import Self

from Moves.moveFactory import MoveFactory
from enums import Colour
from enums import File
from coords import Coords
from strings import *

class Piece:
    def __init__(self, colour: Colour, coords: Coords) -> None:
        self.move_factory = MoveFactory()
        self.colour = colour
        self.coords = coords
        self.has_moved = False
    
    def get_representation(self) -> str:
        raise NotImplementedError

    def get_moves(self, game) -> [str]:
        raise NotImplementedError
    
    def move(self, new_coords: Coords) -> None:
        self.coords = new_coords
        self.has_moved = True
    