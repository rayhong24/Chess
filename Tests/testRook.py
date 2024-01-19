import unittest

from testPieces import PiecesTestCases
from enums import *

class testRook(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        rook1 = self.game.board.board[7][0]
        expected_moves = []
        self.check_moves(rook1, expected_moves)

        rook2 = self.game.board.board[7][7]
        self.check_moves(rook2, expected_moves)
