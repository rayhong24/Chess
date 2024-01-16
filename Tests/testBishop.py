import unittest

from testPieces import PiecesTestCases
from enums import *

class testBishop(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_bishop_simple(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[7][2]
        expected_moves = []

        self.check_moves(piece, expected_moves)
