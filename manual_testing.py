import unittest

from enums import *
from game import Game
from board import Board
from Pieces.piece import Piece
from Moves.moveFactory import MoveFactory
from coords import Coords

def main():
    c = Coords(0, File(1))
    print(c)



if __name__ == "__main__":
    main()