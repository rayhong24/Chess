import unittest

from testPieces import PiecesTestCases
from enums import *

class testQueen(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        queen = self.game.board.board[7][3]
        expected_moves = []
        self.check_moves(queen, expected_moves)