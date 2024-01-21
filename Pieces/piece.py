from typing import Self

from Moves.moveFactory import MoveFactory
from enums import Colour
from enums import File
from strings import *

class Piece:
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        self.move_factory = MoveFactory()
        self.colour = colour
        self.row = row
        self.column = column
        self.has_moved = False
    
    def get_representation(self) -> str:
        raise NotImplementedError

    def is_inbounds(self, i, j):
        return 0<=i<=7 and 0<=j<=7

    def get_move_str(self, start_i, start_j, new_i, new_j, capture):
        piece_str = self.get_representation().upper()

        if piece_str == 'P':
            piece_str = ""

        if capture:
            return f"{piece_str}{File(start_j).name}{8-start_i}x{File(new_j).name}{8-new_i}"
        else:
            return f"{piece_str}{File(start_j).name}{8-start_i}-{File(new_j).name}{8-new_i}"

    def get_moves(self, game) -> [str]:
        raise NotImplementedError
    
    def move(self, new_i: int, new_j: int) -> None:
        self.row = new_i
        self.column = new_j
        self.has_moved = True
    