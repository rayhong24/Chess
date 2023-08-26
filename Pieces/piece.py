from __future__ import annotations

from enums import Colour
from strings import *

class Piece:
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        self.colour = colour
        self.row = row
        self.column = column
        self.has_moved = False
    
    def get_representation(self) -> str:
        raise NotImplementedError

    def is_inbounds(self, i, j):
        return 0<=i<=7 and 0<=j<=7

    def get_moves(self, board: [[Piece]]) -> [tuple]:
        raise NotImplementedError
    
    def move(self, new_i: int, new_j: int) -> None:
        self.row = new_i
        self.column = new_j
        self.has_moved = True
    