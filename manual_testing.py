import unittest

from enums import *
from game import Game
from board import Board
from Pieces.piece import Piece
from Moves.moveFactory import MoveFactory

def main():
    factory = MoveFactory()
    m = factory.init_move("e92-e3", Colour.WHITE)
    print(f"{m=}")



if __name__ == "__main__":
    main()